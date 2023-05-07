#![allow(unused)]
use toyc;
use toy_share;

#[test]
fn diagnose() {
    let x = toyc::diagnostics::Diagnostic {
        location: ("src/main.toy".into(), 1)
    };
    toyc::diagnose_push!(x.clone());
    assert_eq!(toyc::diagnose_pop!(), Some(x))
}

#[test]
fn serde_bytecode() {
    let x = toy_share::Bytecode {
        children: vec![toy_share::Command::Or(
            toy_share::Bytecode { children: vec![] }
        )]
    };
    assert_eq!(x.ser().unwrap(), vec![1, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(toy_share::de(&vec![1, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap(), x);
    let v = toy_share::de(&vec![255, 23, 73]);
    match v {
        Ok(v) => panic!("{:?} not expected", v),
        Err(e) => {}
    }
}