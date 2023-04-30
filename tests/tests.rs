#![allow(unused)]
use toyc;

#[test]
fn diagnose() {
    let x = toyc::diagnostics::Diagnostic {
        location: ("src/main.toy".into(), 1)
    };
    toyc::diagnose_push!(x.clone());
    assert_eq!(toyc::diagnose_pop!(), Some(x))
}