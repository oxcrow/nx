use enum_as_inner::EnumAsInner;
//
use crate::lexer::token::SourceSpan;

#[derive(Default, Debug, Clone, Copy, EnumAsInner)]
pub enum AstNode {
    // Visibility
    Visible(AstNodeData),
    Invisible(AstNodeData),

    // Types
    Unit(AstNodeData),
    Usize(AstNodeData),
    Int(AstNodeData),
    Flt(AstNodeData),
    Str(AstNodeData),

    // Values
    Integer(AstNodeData),
    Float(AstNodeData),
    String(AstNodeData),
    Identifier(AstNodeData),

    // Helpers
    StartFunction(AstNodeData),
    EndFunction(AstNodeData),
    StartStatement(AstNodeData),
    EndStatement(AstNodeData),
    StartExpression(AstNodeData),
    EndExpression(AstNodeData),

    #[default]
    None,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct AstNodeData {
    pub span: SourceSpan,
    pub type_: AstNodeType,
}

#[derive(Default, Debug, Clone, Copy, EnumAsInner)]
pub enum AstNodeType {
    #[default]
    None,
}

#[derive(Default, Debug, Clone, Copy, EnumAsInner)]
pub enum AstNodeProxy {
    #[default]
    None,
}

#[derive(Default, Debug, Clone, Copy, EnumAsInner)]
pub enum AstNodeProxyMut {
    #[default]
    None,
}
