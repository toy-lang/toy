pub static mut DIAGNOSTIC_STREAM:Vec<Diagnostic> = Vec::new();

pub struct Diagnostic {
    
}

#[macro_export]
macro_rules! diagnose {
    (v:Expr) => {
        unsafe { toyc::DIAGNOSTIC_STREAM.push(v) }
    };
}