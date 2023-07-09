use std::ops::Range;

pub enum Diagnostic {
    Error { location: (String, Range<usize>) },
    Warning { location: (String, Range<usize>) },
    Info { location: (String, Range<usize>) },
    ICE { details: String }
}

pub struct Progress {
    message: String
}

pub trait LogReciever {
    #[allow(unused)]
    fn recv_diagnostic(d: &Diagnostic) {}
    #[allow(unused)]
    fn recv_progress(d: &Progress) {}
}