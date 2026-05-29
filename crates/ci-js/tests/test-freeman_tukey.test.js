import init, { freeman_tukey_test } from "../pkg/ci_js.js";
import { describe, test, expect } from "vitest";

const toFloat64 = (...vals) => new Float64Array(vals);

describe("freeman_tukey_test", () => {
  test("unconditional independent data is not rejected", () => {
    const x = toFloat64(1, 1, 2, 2, 1, 1, 2, 2);
    const y = toFloat64(1, 2, 1, 2, 1, 2, 1, 2);
    const z = new Float64Array(0);

    const [p_value, statistic, dof] = freeman_tukey_test(
      "freeman_tukey",
      z,
      0,
      0,
      x,
      y,
      false,
      0.05,
    );

    expect(statistic).toBeLessThan(1e-9);
    expect(p_value).toBeGreaterThanOrEqual(0.99);
    expect(dof).toBe(1);
  });

  test("dependent data is rejected", () => {
    const x = new Float64Array([1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2]);
    const y = new Float64Array([1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 2]);
    const z = new Float64Array(0);

    const [p_value, statistic, dof] = freeman_tukey_test(
      "freeman_tukey",
      z,
      0,
      0,
      x,
      y,
      false,
      0.05,
    );

    expect(statistic).toBeGreaterThan(5.0);
    expect(p_value).toBeLessThan(0.05);
    expect(dof).toBe(1);
  });

  test("boolean mode returns independent=TRUE for independent data", () => {
    const x = new Float64Array([1, 1, 2, 2, 1, 1, 2, 2]);
    const y = new Float64Array([1, 2, 1, 2, 1, 2, 1, 2]);
    const z = new Float64Array(0);

    const result = freeman_tukey_test(
      "freeman_tukey",
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
});
