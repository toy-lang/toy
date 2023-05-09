/*!

*/

pub mod diagnostics;
pub mod lexer;
pub mod parser;

pub struct SourceCode {
    pub display_name: String,
    pub code: String,
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

pub trait CcfValue {}

impl CcfValue for bool {}
impl CcfValue for String {}
impl CcfValue for toy_share::Bytecode {}

pub struct SkeletonStatusMessage {

}

pub trait Skeleton {
    fn import(name: String, name: String) -> SourceCode;
    fn entry() -> SourceCode;
    #[allow(unused_variables)] //This is needed, status should be used if overrided but not normally.
    fn ccf<T: CcfValue>(key: String) -> T {
        todo!()
    }
    #[allow(unused_variables)] //This is needed, status should be used if overrided but not normally.
    fn report_status(status: SkeletonStatusMessage) {}
}