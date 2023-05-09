pub struct Error {
    pub location: (String, usize)
}
pub struct Warning {
    pub location: (String, usize)
}
pub struct Info {
    pub location: (String, usize)
}
pub struct ICE {
    pub details: (String, usize)
}