use std::ops::Range;

pub struct Error {
    pub location: (String, Range<usize>)
}
pub struct Warning {
    pub location: (String, Range<usize>)
}
pub struct Info {
    pub location: (String, Range<usize>)
}
pub struct ICE {
    pub details: String
}