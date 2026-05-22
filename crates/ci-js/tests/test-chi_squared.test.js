import init, { JSCITest } from '../pkg/ci_js.js';
import { describe, test, expect } from "vitest";

const toFloat64 = (...vals) => new Float64Array(vals);

describe('chi_squared_test', () => {

  test('independent data is not rejected', () => {
    const x = toFloat64(1, 1, 2, 2, 1, 1, 2, 2);
    const y = toFloat64(1, 2, 1, 2, 1, 2, 1, 2);
    const z = new Float64Array(0);

    const [p_value, statistic, dof] = JSCITest.run_test(
      'chi_squared', z, 0, 0, x, y, false, 0.05
    );

    expect(statistic).toBeLessThan(1e-9);
    expect(p_value).toBeGreaterThanOrEqual(0.99);
    expect(dof).toBe(1);
  });

  test('dependent data is rejected', () => {
    const x = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const y = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const z = new Float64Array(0);

    const [p_value, statistic, dof] = JSCITest.run_test(
      'chi_squared', z, 0, 0, x, y, false, 0.05
    );

    expect(Math.abs(statistic - 8.0)).toBeLessThan(1e-9);
    expect(Math.abs(p_value - 0.004677734981047276)).toBeLessThan(1e-12);
    expect(dof).toBe(1);
  });

  test('boolean mode returns true for independent data', () => {
    const x = toFloat64(1, 1, 2, 2, 1, 1, 2, 2);
    const y = toFloat64(1, 2, 1, 2, 1, 2, 1, 2);
    const z = new Float64Array(0);

    const result = JSCITest.run_test(
      'chi_squared', z, 0, 0, x, y, true, 0.05
    );

    expect(result).toBe(true);
  });

  test('boolean mode returns false for dependent data', () => {
    const x = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const y = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const z = new Float64Array(0);

    const result = JSCITest.run_test(
      'chi_squared', z, 0, 0, x, y, true, 0.05
    );

    expect(result).toBe(false);
  });

});