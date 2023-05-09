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

pub struct SkeletonStatusMessage {
    pub message_type: SkeletonStatusMessageType
}

pub enum SkeletonStatusMessageType {
    OpeningFile(String)
}

pub trait Skeleton {
    fn import(&self, name: String, display_name: String) -> SourceCode;
    fn entry(&self) -> SourceCode;
    #[allow(unused_variables)] //This is needed, status should be used if overrided but not normally.
    fn ccf(&self, key: String) -> toy_share::Bytecode {
        return toy_share::Bytecode {
            children: vec![toy_share::Command::Bool(false)] //If no ovveride is given, insert false.
        };
    }
    #[allow(unused_variables)] //This is needed, status should be used if overrided but not normally.
    fn report_status(&self, status: SkeletonStatusMessage) {}
}