//! An internal library of types shared between our macros and the nvi library.

/// The type of method being generated
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodType {
    /// A request method
    Request,
    /// A notification method
    Notify,
    /// Methods that are passed through straight into the NviPlugin impl block
    Connected,
    /// A highlights method
    Highlights,
}

/// An argument to a method
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    /// The name of the argument
    pub name: String,
    /// The type of the argument
    pub typ: String,
}

/// The return type of a method
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

/// An autocommand definition
#[derive(Debug, Eq, PartialEq)]
pub struct AutoCmd {
    /// The events to listen for
    pub events: Vec<String>,
    /// The patterns to match
    pub patterns: Vec<String>,
    /// The group the autocommand belongs to
    pub group: Option<String>,
    /// Whether the autocommand is nested
    pub nested: bool,
}

/// A method definition
#[derive(Debug, Eq, PartialEq)]
pub struct Method {
    /// The name of the method
    pub name: String,
    /// The documentation for the method
    pub docs: String,
    /// The return type of the method
    pub ret: Return,
    /// The type of the method
    pub method_type: MethodType,
    /// The arguments to the method
    pub args: Vec<Arg>,
    /// The autocommand associated with the method
    pub autocmd: Option<AutoCmd>,
    /// Whether the method is mutable
    pub is_mut: bool,
}
