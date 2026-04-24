library(cir)

citest = RCressieRead$new(FALSE, 0.05)
run_citest_standard_tests(citest)

test_that("Conditional case is independent", {
    x = c(1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0)
    y = c(1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0)
    z = matrix(c(0., 0., 0., 0., 1., 1., 1., 1.), nrow = 8, ncol = 1)

    result = test$run_test(x, y, z)
    expect_equal(result$kind, "statistic")
    expect_true(result$statistic < 1e-9)
    expect_true(result$p_value >= 0.99)
    expect_equal(result$df, 2)
  })