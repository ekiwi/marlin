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

    let mut dut = runtime
        .create_dyn_model(
            "wide",
            "tests/wide.v",
            &[
                ("in", 2999, 0, PortDirection::Input),
                ("out", 2999, 0, PortDirection::Output),
                ("ni", 31, 0, PortDirection::Input),
                ("no", 31, 0, PortDirection::Output),
            ],
            VerilatedModelConfig::default(),
        )
        .unwrap();

    // test with narrow values
    dut.pin("ni", 1234u16).unwrap();
    dut.eval();
    let out: u32 = dut.read("no").unwrap().try_into().unwrap();
    assert_eq!(out, 1234u32);

    // wide values
    dut.pin("in", 1234u32).unwrap();
    dut.eval();
    let out: u32 = dut.read("out").unwrap().try_into().unwrap();
    assert_eq!(out, 1234u32);
}
