import { pearson_correlation_test } from "../pkg/ci_js.js";
import { describe, test, expect } from "vitest";

const toFloat64 = (...vals) => new Float64Array(vals);

describe("pearson_correlation_test", () => {
  test("independent data is not rejected", () => {
    const x = toFloat64(1, 1, 2, 2, 1, 1, 2, 2);
    const y = toFloat64(1, 2, 1, 2, 1, 2, 1, 2);
    const z = new Float64Array(0);

    const [p_value, coefficient] = pearson_correlation_test(
      "pearson_correlation",
      z,
      0,
      0,
      x,
      y,
      false,
      0.05,
    );

    expect(Math.abs(coefficient)).toBeLessThan(1e-9);
    expect(p_value).toBeGreaterThanOrEqual(0.99);
  });

  test("dependent data is rejected", () => {
    const x = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const y = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const z = new Float64Array(0);

    const [p_value, coefficient] = pearson_correlation_test(
      "pearson_correlation",
      z,
      0,
      0,
      x,
      y,
      false,
      0.05,
    );

    expect(Math.abs(coefficient - 1.0)).toBeLessThan(1e-9);
    expect(p_value).toBeLessThan(0.05);
  });

  test("negatively correlated data is rejected", () => {
    const x = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const y = toFloat64(2, 2, 2, 2, 1, 1, 1, 1);
    const z = new Float64Array(0);

    const [p_value, coefficient] = pearson_correlation_test(
      "pearson_correlation",
      z,
      0,
      0,
      x,
      y,
      false,
      0.05,
    );

    expect(Math.abs(coefficient + 1.0)).toBeLessThan(1e-9);
    expect(p_value).toBeLessThan(0.05);
  });

  test("boolean mode returns true for independent data", () => {
    const x = toFloat64(1, 1, 2, 2, 1, 1, 2, 2);
    const y = toFloat64(1, 2, 1, 2, 1, 2, 1, 2);
    const z = new Float64Array(0);

    const result = pearson_correlation_test(
      "pearson_correlation",
      z,
      0,
      0,
      x,
      y,
      true,
      0.05,
    );

    expect(result).toBe(true);
  });

  test("boolean mode returns false for dependent data", () => {
    const x = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const y = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const z = new Float64Array(0);

    const result = pearson_correlation_test(
      "pearson_correlation",
      z,
      0,
      0,
      x,
      y,
      true,
      0.05,
    );

    expect(result).toBe(false);
  });

  test("conditional boolean accepts conditionally independent data", () => {
    const x = toFloat64(1, 1, 2, 2, 1, 1, 2, 2);
    const y = toFloat64(1, 2, 1, 2, 1, 2, 1, 2);
    const z = new Float64Array([0, 0, 0, 0, 1, 1, 1, 1]);

    const result = pearson_correlation_test(
      "pearson_correlation",
      z,
      8,
      1,
      x,
      y,
      true,
      0.05,
    );

    expect(result).toBe(true);
  });
});
