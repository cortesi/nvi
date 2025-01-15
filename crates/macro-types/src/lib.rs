//! An internal library of types shared between our macros and the nvi library.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodType {
    Request,
    Notify,
    /// Methods that are passed through straight into the NviPlugin impl block
    Connected,
    Highlights,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    pub name: String,
    pub typ: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Return {
    /// A void return
    Void,
    /// A Result<()> return
    ResultVoid,
    /// A Result with an inner type
    Result(String),
    /// A naked non-Result return
    Type(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct AutoCmd {
    pub events: Vec<String>,
    pub patterns: Vec<String>,
    pub group: Option<String>,
    pub nested: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Method {
    pub name: String,
    pub docs: String,
    pub ret: Return,
    pub method_type: MethodType,
    pub args: Vec<Arg>,
    pub autocmd: Option<AutoCmd>,
    pub is_mut: bool,
}
