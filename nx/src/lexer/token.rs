use enum_as_inner::EnumAsInner;

/// Tokens of the language.
///
/// The tokens are expected to exist as long as the code.
/// + This is necessary as the IntVal, IdxVal, StrVal etc.
///   use references to strings stored within the code.
#[derive(Default, Debug, EnumAsInner)]
pub enum Token<'code> {
    Documentation(&'code str),
    Comment(&'code str),
    // Symbols
    Semicolon,
    Colon,
    Comma,
    Dot,
    //
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    //
    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,
    LAngle,
    RAngle,
    LBrace,
    RBrace,
    //
    Exclamation,
    Question,
    Dollar,
    Hash,

    // Directives
    Use,
    Let,
    Var,
    As,
    In,
    Return,
    Break,
    Continue,

    // Blocks
    Macro,
    Module,
    Fn,
    Struct,
    Enum,
    Instance,
    Implement,
    Match,
    If,
    Else,
    For,
    While,
    Loop,

    // Types
    Unit,
    Usize,
    Int,
    Flt,
    Str,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,

    // Values
    IntVal(&'code str),
    FltVal(&'code str),
    StrVal(&'code str),
    IdxVal(&'code str),

    #[default]
    None,
}
