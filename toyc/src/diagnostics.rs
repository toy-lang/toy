pub struct DiagnoseStream(Vec<Diagnostic>);

pub static mut DIAGNOSTIC_STREAM:DiagnoseStream = DiagnoseStream(Vec::new());

impl core::ops::Shl<Diagnostic> for DiagnoseStream {
    type Output = DiagnoseStream;
    fn shl(mut self, rhs: Diagnostic) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

#[derive(PartialEq, Debug, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Diagnostic {
    
}