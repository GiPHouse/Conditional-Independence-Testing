run_citest_standard_tests = function(test) {
  test_that("independent data is not rejected", {
    x = c(1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0)
    y = c(1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0)
    z = matrix(0, nrow = 8, ncol = 0)

    result = test$run_test(x, y, z)
    expect_equal(result$kind, "statistic")
    expect_true(result$statistic < 1e-9)
    expect_true(result$p_value >= 0.99)
    expect_equal(result$df, 1)
  })

  test_that("dependent data is rejected", {
    x = c(1., 1., 1., 1., 2., 2., 2., 2.)
    y = c(1., 1., 1., 1., 2., 2., 2., 2.)
    z = matrix(0, nrow = 8, ncol = 0)

    result = test$run_test(x, y, z)
    expect_equal(result$kind, "statistic")
    expect_true(abs(result$statistic - 8.0) < 1e-9)
    expect_equal(result$df, 1)
  })
}
