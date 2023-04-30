pub struct DiagnoseStream(pub Vec<Diagnostic>);


pub static mut DIAGNOSTIC_STREAM:DiagnoseStream = DiagnoseStream(Vec::new());
impl DiagnoseStream {
    pub fn push(&mut self, v: Diagnostic) {
        self.0.push(v);
    }
    pub fn pop(&mut self) -> Option<Diagnostic> {
        return self.0.pop();
    }
}
#[derive(PartialEq, Debug, Eq, PartialOrd, Ord, Clone)]
pub struct Diagnostic {
    pub location: (String, usize)
}

#[macro_export]
macro_rules! diagnose_push {
    ($v:expr) => {
        unsafe {toyc::diagnostics::DIAGNOSTIC_STREAM.push($v)}
    };
}

#[macro_export]
macro_rules! diagnose_pop {
    () => {
        unsafe {toyc::diagnostics::DIAGNOSTIC_STREAM.pop()}
    };
}