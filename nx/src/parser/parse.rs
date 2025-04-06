use crate::error::{ensure, Result};
//
use crate::data::ast::{AstNode, AstNodeData, AstNodeType};
use crate::lexer::token::Token;

pub fn parse_token<'code>(tokens: &'code [Token<'code>]) -> Result<()> {
    ensure!(!tokens.is_empty(), "can not parse an empty token list");

    let mut xast: Vec<AstNode> = vec![];
    let mut ast: Vec<AstNode> = vec![];
    let mut token_index = 0;

    /// Store nodes from temporary ast
    macro_rules! store_into_ast {
        ($xast:ident) => {
            for &node in $xast.iter() {
                ast.push(node);
            }
            xast = $xast; // so xast isn't drop'd
            xast.clear();
        };
    }

    // Parse tokens one by one until all tokens are processed.
    // Since top level entities such as modules, functions, structs
    // are expected to occur first in our code, they are parsed
    // using a recursive descent algorithm.
    while token_index < tokens.len() {
        let token = tokens.get(token_index);
        match token {
            Some(Token::Fn(_, _)) => {
                let (new_xast, next_token_index) = parse_function(xast, tokens, token_index)?;
                // Store ast nodes and reset state
                store_into_ast!(new_xast);
                // Advance token_index to process rest of the remaining tokens
                // Warning: Without this the loop will run forever
                token_index = next_token_index;
            }
            _ => {
                // Reset
                xast.clear();
                // Advance token_index to process rest of the remaining tokens
                // Warning: Without this the loop will run forever
                token_index += 1;
            }
        }
    }
    dbg!(&ast);
    dbg!("--");

    Ok(())
}

fn parse_function<'code>(
    mut ast: Vec<AstNode>,
    tokens: &'code [Token<'code>],
    token_index: usize,
) -> Result<(Vec<AstNode>, usize)> {
    // Mark the start of function
    ast.push(AstNode::StartFunction(AstNodeData::default()));

    // Parse rest of the ast nodes
    let (ast, next_token_index) = parse_visibility(ast, tokens, token_index)?; // Should we search backwards?
    let (ast, next_token_index) = parse_identifier(ast, tokens, next_token_index)?;
    let (ast, next_token_index) = parse_argument(ast, tokens, next_token_index)?;
    let (ast, next_token_index) = parse_type(ast, tokens, next_token_index)?;
    let (ast, next_token_index) = parse_block(ast, tokens, next_token_index)?;

    // Mark the end of function
    let mut ast = ast;
    ast.push(AstNode::EndFunction(AstNodeData::default()));

    // where ...
    Ok((ast, next_token_index))
}

fn parse_visibility<'code>(
    mut ast: Vec<AstNode>,
    _tokens: &'code [Token<'code>],
    token_index: usize,
) -> Result<(Vec<AstNode>, usize)> {
    let visibility = AstNode::Invisible(AstNodeData::default());
    let next_token_index = token_index + 1;
    ast.push(visibility);
    Ok((ast, next_token_index))
}

fn parse_identifier<'code>(
    mut ast: Vec<AstNode>,
    tokens: &'code [Token<'code>],
    token_index: usize,
) -> Result<(Vec<AstNode>, usize)> {
    let identifier_token = tokens.get(token_index).unwrap();
    let next_token_index = token_index + 1;
    let span = identifier_token.into_idx_val().unwrap().0;
    let identifier = AstNode::Identifier(AstNodeData {
        span,
        type_: AstNodeType::default(),
    });
    ast.push(identifier);
    Ok((ast, next_token_index))
}

fn parse_argument<'code>(
    mut ast: Vec<AstNode>,
    _tokens: &'code [Token<'code>],
    token_index: usize,
) -> Result<(Vec<AstNode>, usize)> {
    ast.push(AstNode::None);
    Ok((ast, token_index))
}

fn parse_type<'code>(
    mut ast: Vec<AstNode>,
    _tokens: &'code [Token<'code>],
    token_index: usize,
) -> Result<(Vec<AstNode>, usize)> {
    ast.push(AstNode::None);
    Ok((ast, token_index))
}

fn parse_block<'code>(
    mut ast: Vec<AstNode>,
    _tokens: &'code [Token<'code>],
    token_index: usize,
) -> Result<(Vec<AstNode>, usize)> {
    ast.push(AstNode::None);
    Ok((ast, token_index))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::lex::tokenize_string;

    #[test]
    fn test_parse_token() -> Result<()> {
        let xa = tokenize_string("fn main() { }")?;
        let xb = tokenize_string("/// Returns zero\n fn zero() int { let x = 0; return x; }")?;
        let _a = parse_token(&xa)?;
        let _b = parse_token(&xb)?;
        Ok(())
    }
}
