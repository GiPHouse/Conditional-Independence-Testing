extendr bindings

##Usage

Install R
In your command line open an R session by typing: R
install.packages("rextendr") (this takes 5-15 mins)

usethis::create_package("crates/ci-r-pkg", open = FALSE)
setwd("crates/ci-r-pkg")
rextendr::use_extendr()

to regenerate bindings -> install.packages("devtools")

devtools::document()   # generates R bindings
devtools::load_all()   # compiles Rust + loads the package

devtools::test() # Run all tests