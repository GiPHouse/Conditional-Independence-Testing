const response = await fetch("http://localhost:3000/tests/pearson_correlation", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
        x: [1.0, 2.0, 3.0, 4.0, 5.0],
        y: [1.0, 2.0, 3.0, 4.0, 5.0],
        z: [],
        boolean: false,
        significance_level: 0.05
    })
});

const result = await response.json();
console.log(result);