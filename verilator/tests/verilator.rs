use marlin_verilator::*;

#[test]
fn wide_input() {
    let options = VerilatorRuntimeOptions::default();

    let mut runtime = VerilatorRuntime::new(
        "test_run".into(),
        &["tests/wide.v".as_ref()],
        &[],
        [],
        options,
    )
    .unwrap();

    let _dut = runtime
        .create_dyn_model(
            "wide",
            "tests/wide.v",
            &[
                ("in", 2999, 0, PortDirection::Input),
                ("out", 2999, 0, PortDirection::Output),
            ],
            VerilatedModelConfig::default(),
        )
        .unwrap();
}
