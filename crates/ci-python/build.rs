use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};
use syn::{
    visit::{self, Visit},
    Fields, File, ItemImpl, ItemStruct, Type, TypePath,
};

/// Visitor that traverses the AST and collects all structs implementing ``CITest`` (``citest_structs``)
/// all struct definitions (``struct_defs``).
struct CITestCollector {
    citest_structs: Vec<String>,
    struct_defs: HashMap<String, ItemStruct>,
}

impl<'ast> Visit<'ast> for CITestCollector {
    /// Collect all structs implementing ``CITest``.
    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        if let Some((None, trait_path, _)) = &node.trait_ {
            let is_citest = trait_path
                .segments
                .last()
                .map_or(false, |s| s.ident == "CITest");

            if is_citest {
                if let Type::Path(TypePath { path, .. }) = node.self_ty.as_ref() {
                    if let Some(seg) = path.segments.last() {
                        self.citest_structs.push(seg.ident.to_string());
                    }
                }
            }
        }
        visit::visit_item_impl(self, node);
    }

    /// Collect all struct definitions.
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.struct_defs
            .insert(node.ident.to_string(), node.clone());
        visit::visit_item_struct(self, node);
    }
}

/// Run the ``CITestCollector`` recursively on the specified directory.
fn parse_dir(dir: &Path, collector: &mut CITestCollector) {
    for entry in fs::read_dir(dir).expect(format!("Failed to read {}", dir.display()).as_str()) {
        let path = entry.unwrap().path();
        if path.is_dir() {
            parse_dir(&path, collector);
        } else if path.extension().map_or(false, |e| e == "rs") {
            let src = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e.to_string()));
            let file: File = syn::parse_file(&src).unwrap_or_else(|e| {
                panic!("Failed to parse {}: {}", path.display(), e.to_string())
            });
            visit::visit_file(collector, &file);
            println!("cargo::rerun-if-changed={}", path.display());
        }
    }
}

/// Generate the ``TokenStream`` for the specified struct ``s``.
fn generate_pyo3_wrapper(s: &ItemStruct) -> TokenStream {
    let struct_name = &s.ident.to_string();
    let struct_ident = format_ident!("{}", struct_name);
    let py_ident = format_ident!("Py{}", struct_name);
    let py_name = syn::LitStr::new(struct_name, proc_macro2::Span::call_site());

    let Fields::Named(named) = &s.fields else {
        panic!(
            "Encountered tuple field when processing `{}`. Tuples aren't supported (yet).",
            struct_name
        )
    };

    let field_names: Vec<_> = named
        .named
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    let field_types: Vec<_> = named.named.iter().map(|f| &f.ty).collect();

    let getters_setters = named.named.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let ftype = &f.ty;
        let setter_ident = format_ident!("set_{}", fname);
        quote! {
            #[getter]
            pub fn #fname(&self) -> #ftype {
                self.inner.#fname.clone()
            }
            #[setter]
            pub fn #setter_ident(&mut self, #fname: #ftype) {
                self.inner.#fname = #fname;
            }
        }
    });

    let constructor_args = field_names.iter().zip(field_types.iter()).map(|(n, t)| {
        quote! { #n: #t }
    });
    let constructor_init = field_names.iter().map(|n| quote! { #n });

    quote! {
        #[gen_stub_pyclass]
        #[pyclass(name = #py_name, module = "ci_python._ci_python")]
        pub struct #py_ident {
            inner: ::ci_core::ci_tests::#struct_ident,  // TODO: Properly handle (sub)module structure.
        }

        #[gen_stub_pymethods]
        #[pymethods]
        impl #py_ident {
            #[new]
            pub fn new(#(#constructor_args),*) -> Self {
                Self {
                    inner: ::ci_core::ci_tests::#struct_ident { #(#constructor_init),* },
                }
            }
            #(#getters_setters)*
        }
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Generate ci_tests.rs.
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let ci_core_src = manifest_dir.join("../ci-core/src");
    assert!(
        ci_core_src.exists(),
        "`ci-core/src` not found at {:?}",
        ci_core_src
    );

    let mut collector = CITestCollector {
        citest_structs: Vec::new(),
        struct_defs: HashMap::new(),
    };
    parse_dir(&ci_core_src, &mut collector);

    let mut tokens = TokenStream::new();

    for name in &collector.citest_structs {
        if let Some(def) = collector.struct_defs.get(name) {
            tokens.extend(generate_pyo3_wrapper(def));
        } else {
            panic!("Struct `{name}` implements `CITest` but its definition was not found in `ci-core/src`.");
        }
    }

    fs::write(out_dir.join("ci_tests.rs"), tokens.to_string()).unwrap();

    // Generate ci_tests_init.rs.
    let mut tokens_init = TokenStream::new();
    let py_class_idents: Vec<_> = collector
        .citest_structs
        .iter()
        .map(|n| format_ident!("Py{}", n))
        .collect();

    tokens_init.extend(quote! {
        use pyo3::prelude::*;

        pub fn init(
            m: &Bound<'_, PyModule>,
        ) -> PyResult<()> {
            #(m.add_class::<super::_ci_python::#py_class_idents>()?;)*
            Ok(())
        }
    });
    fs::write(out_dir.join("ci_tests_init.rs"), tokens_init.to_string()).unwrap();
}
