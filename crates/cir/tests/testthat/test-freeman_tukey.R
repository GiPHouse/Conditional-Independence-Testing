library(cir)
test = RFreemanTukey$new(FALSE, 0.05)

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
    x = c(1., 1., 1., 1., 1., 1., 2., 2., 2., 2., 2., 2.)
    y = c(1., 1., 1., 1., 1., 2., 1., 2., 2., 2., 2., 2.)
    z = matrix(0, nrow = 12, ncol = 0)

    result = test$run_test(x, y, z)
    expect_equal(result$kind, "statistic")
    expect_true(abs(result$statistic - 6.319453539579289) < 1e-9)
    expect_true(abs(result$p_value - 0.011942042564347121) < 1e-12)
    expect_equal(result$df, 1)
  })

test_bool = RFreemanTukey$new(TRUE, 0.05)

test_that("boolean mode returns independent=TRUE for independent data", {
    x = c(1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0)
    y = c(1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0)
    z = matrix(0, nrow = 8, ncol = 0)

    result = test_bool$run_test(x, y, z)
    expect_equal(result$kind, "boolean")
    expect_true(result$independent)
  })
