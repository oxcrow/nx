use enum_as_inner::EnumAsInner;

/// Span of elements in the source code.
///
/// This is useful for reporting errors in the code.
#[derive(Default, Debug, Clone, Copy)]
pub struct SourceSpan {
    pub start: u32,
    pub end: u32,
}

/// Tokens of the language.
///
/// The tokens are expected to exist as long as the code.
/// + This is necessary as the IntVal, IdxVal, StrVal etc.
///   use references to strings stored within the code.
#[derive(Default, Debug, EnumAsInner)]
pub enum Token<'code> {
    Documentation(SourceSpan, &'code str),
    Comment(SourceSpan, &'code str),
    // Symbols
    Semicolon(SourceSpan, &'code str),
    Colon(SourceSpan, &'code str),
    Comma(SourceSpan, &'code str),
    Dot(SourceSpan, &'code str),
    //
    Equal(SourceSpan, &'code str),
    Plus(SourceSpan, &'code str),
    Minus(SourceSpan, &'code str),
    Star(SourceSpan, &'code str),
    Slash(SourceSpan, &'code str),
    //
    LParenthesis(SourceSpan, &'code str),
    RParenthesis(SourceSpan, &'code str),
    LBracket(SourceSpan, &'code str),
    RBracket(SourceSpan, &'code str),
    LAngle(SourceSpan, &'code str),
    RAngle(SourceSpan, &'code str),
    LBrace(SourceSpan, &'code str),
    RBrace(SourceSpan, &'code str),
    //
    Exclamation(SourceSpan, &'code str),
    Question(SourceSpan, &'code str),
    Dollar(SourceSpan, &'code str),
    Hash(SourceSpan, &'code str),

    // Directives
    Use(SourceSpan, &'code str),
    Let(SourceSpan, &'code str),
    Var(SourceSpan, &'code str),
    As(SourceSpan, &'code str),
    In(SourceSpan, &'code str),
    Return(SourceSpan, &'code str),
    Break(SourceSpan, &'code str),
    Continue(SourceSpan, &'code str),

    // Blocks
    Macro(SourceSpan, &'code str),
    Module(SourceSpan, &'code str),
    Fn(SourceSpan, &'code str),
    Struct(SourceSpan, &'code str),
    Enum(SourceSpan, &'code str),
    Instance(SourceSpan, &'code str),
    Implement(SourceSpan, &'code str),
    Match(SourceSpan, &'code str),
    If(SourceSpan, &'code str),
    Else(SourceSpan, &'code str),
    For(SourceSpan, &'code str),
    While(SourceSpan, &'code str),
    Loop(SourceSpan, &'code str),

    // Types
    Unit(SourceSpan, &'code str),
    Usize(SourceSpan, &'code str),
    Int(SourceSpan, &'code str),
    Flt(SourceSpan, &'code str),
    Str(SourceSpan, &'code str),
    I8(SourceSpan, &'code str),
    U8(SourceSpan, &'code str),
    I16(SourceSpan, &'code str),
    U16(SourceSpan, &'code str),
    I32(SourceSpan, &'code str),
    U32(SourceSpan, &'code str),
    I64(SourceSpan, &'code str),
    U64(SourceSpan, &'code str),
    F32(SourceSpan, &'code str),
    F64(SourceSpan, &'code str),

    // Values
    IntVal(SourceSpan, &'code str),
    FltVal(SourceSpan, &'code str),
    StrVal(SourceSpan, &'code str),
    IdxVal(SourceSpan, &'code str),

    #[default]
    None,
}

impl SourceSpan {
    pub fn new(start_index: usize, end_index: usize) -> Self {
        SourceSpan {
            start: start_index as u32,
            end: end_index as u32,
        }
    }
}
