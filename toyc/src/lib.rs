/*!

*/

#[forbid(
    deprecated_in_future,
    deprecated,
    arithmetic_overflow,
    clippy::disallowed_names,
)]

#[allow(
    clippy::needless_return // Why is this warned on by default?
)]

pub mod diagnostics;
pub mod value;
pub mod lexer;
pub mod parser;

pub struct SourceCode {
    pub display_name: String,
    pub code: String,
    /// The name, usually the file name.
    pub name: String
}

impl SourceCode {
    pub fn new(name:String, display_name: String, code: String) -> SourceCode {
        return SourceCode {
            name,
            display_name,
            code
        }
    }
}

pub struct SkeletonStatusMessage {
    pub message_type: SkeletonStatusMessageType
}

pub enum SkeletonStatusMessageType {
    Error(diagnostics::Error),
    Warning(diagnostics::Warning),
    Info(diagnostics::Info),
    ICE(diagnostics::ICE),
    OpeningFile(String)
}

pub trait Skeleton {
    fn import(&self, name: String, display_name: String) -> SourceCode;
    fn entry(&self) -> SourceCode;
    #[allow(unused_variables)] //This is needed, status should be used if overrided but not normally.
    fn ccf(&self, key: String) -> value::Value {
        return value::Value::Bool(false);
    }
    fn report_status(&self, status: SkeletonStatusMessage);
}

pub fn builder<T: Skeleton>(skel: T) {
    skel.entry();
}