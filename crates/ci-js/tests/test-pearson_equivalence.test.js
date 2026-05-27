import { JSCITest } from '../pkg/ci_js.js';
import { describe, test, expect } from "vitest";

const toFloat64 = (...vals) => new Float64Array(vals);

describe('pearson_equivalence_tests', () => {

  test('independent_data_is_not_rejected', () => {
    const x = toFloat64(1, 2, 1, 2, 1, 2, 1, 2);
    const y = toFloat64(2, 1, 2, 1, 2, 1, 2, 1);
    const z = new Float64Array(0);

    const [p_value, coefficient] = JSCITest.run_test(
      'pearson_equivalence', z, 0, 0, x, y, false, 0.05
    );

    expect(p_value).toBeLessThanOrEqual(0.05);
    expect(Math.abs(coefficient)).toBeLessThan(0.1);
  });

    test('dependent_data_is_rejected', () => {
    const x = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const y = toFloat64(3, 3, 3, 3, 6, 6, 6, 6);
    const z = new Float64Array(0);

    const [p_value, coefficient] = JSCITest.run_test(
      'pearson_equivalence', z, 0, 0, x, y, false, 0.05
    );

    expect(p_value).toBeGreaterThanOrEqual(0.05);
    expect(Math.abs(coefficient)).toBeGreaterThan(0.9);
  });

  test('boolean mode returns true for independent data', () => {
    const x = toFloat64(1, 2, 1, 2, 1, 2, 1, 2);
    const y = toFloat64(2, 1, 2, 1, 2, 1, 2, 1);
    const z = new Float64Array(0);

    const result = JSCITest.run_test(
      'pearson_equivalence', z, 0, 0, x, y, true, 0.05
    );

    expect(result).toBe(true);
  });


  test('boolean mode returns false for dependent data', () => {
    const x = toFloat64(1, 1, 1, 1, 2, 2, 2, 2);
    const y = toFloat64(3, 3, 3, 3, 6, 6, 6, 6);
    const z = new Float64Array(0);

    const result = JSCITest.run_test(
      'pearson_equivalence', z, 0, 0, x, y, true, 0.05
    );

    expect(result).toBe(false);
  });
});