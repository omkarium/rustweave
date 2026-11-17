// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for error behaviour.

mod common;

const ERROR_CASES: &[&str] = &[
    r####"["a", 1] filter (($ as Number) > 0)"####,
    r####"notAFunction(1)"####,
    r####"payload.orders map ($.total / 0)"####,
];

#[test]
fn behaviour_cases() {
    for script in ERROR_CASES {
        assert!(
            common::run_result(script).is_err(),
            "expected error: {script}"
        );
    }
}
