use crate::error::{ensure, Result};
//
use crate::lexer::token::{SourceSpan, Token};

/// Tokenize code.
///
/// # Performance Consideration
///
/// A Vec<Token> will be created to return the result.
/// Thus this method is expected to be used mostly for tokenizing
/// large sections of code (such as files). Not small lines of code.
///
/// This can be slow if used in performance critical code.
#[allow(clippy::needless_lifetimes)]
pub fn tokenize_string<'code>(code: &'code str) -> Result<Vec<Token<'code>>> {
    tokenize_string_standard(code, vec![])
}

/// Tokenize code.
///
/// # Performance Consideration
///
/// A Vec<Token> is required as an argument to return the result.
///
/// Thus to optimize performance tokens should be preallocated.
/// Internally the tokens.capacity() is checked and only allocated
/// if the current exisitng allocated memory capacity is not enough.
///
/// This can be used in performance critical code.
pub fn tokenize_string_standard<'code>(
    mut code: &'code str,
    mut tokens: Vec<Token<'code>>,
) -> Result<Vec<Token<'code>>> {
    ensure!(!code.is_empty(), "code can not be empty string");
    tokens.clear();

    // Allocate enough memory for tokens
    let num_lines = code.chars().filter(|&c| c == '\n').count();
    let guess_num_tokens_per_line = 20;
    let guess_num_tokens = (num_lines + 1) * guess_num_tokens_per_line;
    let max_num_tokens = guess_num_tokens * 5;
    //
    if tokens.capacity() < guess_num_tokens {
        tokens.reserve(guess_num_tokens - tokens.capacity());
    }

    let mut code_index = 0;

    // Tokenize one by one until the code is not empty
    while !code.is_empty() {
        // Find next token
        let (token, remaining_code, new_code_index) = tokenize_next_word(code, code_index)?;
        // Check if token is identified corrrectly
        ensure!(!token.is_none(), "Token::None was found during lexing.");
        // Check for memory overflow
        ensure!(
            tokens.len() < max_num_tokens,
            "can not store more than max_num_tokens as it may cause memory overflow"
        );
        // Store token
        tokens.push(token);
        // Truncate code to process rest of the remaining code
        // Warning: Without this the loop will run forever
        code = remaining_code;
        code_index = new_code_index;
    }

    Ok(tokens)
}

/// Find the next token.
///
/// The code is truncated and the remaining is returned.
///
/// # Algorithm
///
/// + Truncate leading whitespace.
/// + Find the next word that ends at a delimiter.
/// + Match the word in two steps as,
///   - Check if the word is a reserved token (use peek).
///   - Check if the word is a float, integer, string, or identifier.
pub fn tokenize_next_word(code: &str, code_index: usize) -> Result<(Token, &str, usize)> {
    let (code, code_index) = truncate_leading_whitespace(code, code_index);

    let (word, remaining_code, new_index) = search_next_word(code, code_index);
    let span = SourceSpan::new(code_index, code_index + word.len());

    // Tokenize documentation comments
    if word == "/" {
        let (next_word_1, remaining_code, new_index) = search_next_word(remaining_code, new_index);
        let (next_word_2, remaining_code, _________) = search_next_word(remaining_code, new_index);
        if next_word_1 == "/" && next_word_2 == "/" {
            let next_newline_index = remaining_code
                .chars()
                .position(|c| c == '\n')
                .unwrap_or(remaining_code.len());
            let remaining_truncated_code = &remaining_code[next_newline_index..];
            let comment = &code[0..(next_newline_index + 3)];
            let span = SourceSpan::new(code_index, code_index + comment.len());
            let token = Token::Documentation(span, comment);
            return Ok((token, remaining_truncated_code, span.end as usize));
        }
    }

    // Tokenize comments
    if word == "/" {
        let (next_word, remaining_code, _) = search_next_word(remaining_code, new_index);
        if next_word == "/" {
            let next_newline_index = remaining_code
                .chars()
                .position(|c| c == '\n')
                .unwrap_or(remaining_code.len());
            let remaining_truncated_code = &remaining_code[next_newline_index..];
            let comment = &code[0..(next_newline_index + 2)];
            let span = SourceSpan::new(code_index, code_index + comment.len());
            let token = Token::Comment(span, comment);
            return Ok((token, remaining_truncated_code, span.end as usize));
        }
    }

    // Tokenize reserved words
    let token = match word {
        // Symbols
        ";" => Token::Semicolon(span, word),
        ":" => Token::Colon(span, word),
        "," => Token::Comma(span, word),
        "." => Token::Dot(span, word),
        //
        "=" => Token::Equal(span, word),
        "+" => Token::Plus(span, word),
        "-" => Token::Minus(span, word),
        "*" => Token::Star(span, word),
        "/" => Token::Slash(span, word),
        //
        "(" => Token::LParenthesis(span, word),
        ")" => Token::RParenthesis(span, word),
        "[" => Token::LBracket(span, word),
        "]" => Token::RBracket(span, word),
        "<" => Token::LAngle(span, word),
        ">" => Token::RAngle(span, word),
        "{" => Token::LBrace(span, word),
        "}" => Token::RBrace(span, word),
        //
        "!" => Token::Exclamation(span, word),
        "?" => Token::Question(span, word),
        "$" => Token::Dollar(span, word),
        "#" => Token::Hash(span, word),

        // Directives
        "use" => Token::Use(span, word),
        "let" => Token::Let(span, word),
        "var" => Token::Var(span, word),
        "as" => Token::As(span, word),
        "in" => Token::In(span, word),
        "return" => Token::Return(span, word),
        "break" => Token::Break(span, word),
        "continue" => Token::Continue(span, word),

        // Blocks
        "macro" => Token::Macro(span, word),
        "module" => Token::Module(span, word),
        "fn" => Token::Fn(span, word),
        "struct" => Token::Struct(span, word),
        "enum" => Token::Enum(span, word),
        "instance" => Token::Instance(span, word),
        "implement" => Token::Implement(span, word),
        "match" => Token::Match(span, word),
        "if" => Token::If(span, word),
        "else" => Token::Else(span, word),
        "for" => Token::For(span, word),
        "while" => Token::While(span, word),
        "loop" => Token::Loop(span, word),

        // Types
        "()" => Token::Unit(span, word),
        "usize" => Token::Usize(span, word),
        "int" => Token::Int(span, word),
        "flt" => Token::Flt(span, word),
        "str" => Token::Str(span, word),
        "i8" => Token::I8(span, word),
        "u8" => Token::U8(span, word),
        "i16" => Token::I16(span, word),
        "u16" => Token::U16(span, word),
        "i32" => Token::I32(span, word),
        "u32" => Token::U32(span, word),
        "i64" => Token::I64(span, word),
        "u64" => Token::U64(span, word),
        "f32" => Token::F32(span, word),
        "f64" => Token::F64(span, word),

        _ => Token::None,
    };

    // Test if the token is an integer value
    //
    // + It is necessary to test for an integer value before testing
    //   if it is an identifier (since we do not use regexes).
    // + Otherwise we would have terrible bugs when values such
    //   as 0 would be falsely tokenized as identifiers.
    // + Since we do not use regex this is the only safe way.
    let (token, remaining_code) = if token.is_none() {
        let new_token = {
            if word.chars().nth(0) != Some('_') && word.chars().all(character_is_integer) {
                Token::IntVal(span, word)
            } else {
                Token::None
            }
        };
        (new_token, remaining_code)
    } else {
        (token, remaining_code)
    };

    // Test if the token is an identifier
    //
    // + Since integers have already been identified this is safe.
    // + Since we know that the first character can not be an integer
    //   we can treat everything as an identifier and tokenize them.
    let (token, remaining_code) = if token.is_none() {
        let new_token = {
            if word.chars().all(character_is_identifier) {
                Token::IdxVal(span, word)
            } else {
                Token::None
            }
        };
        (new_token, remaining_code)
    } else {
        (token, remaining_code)
    };

    // where ...

    fn truncate_leading_whitespace(code: &str, code_index: usize) -> (&str, usize) {
        let next_non_whitespace_character_index = {
            code.chars()
                .position(|c| c != ' ' && c != '\t' && c != '\n')
                .unwrap_or(0)
        };
        let new_index = code_index + next_non_whitespace_character_index;
        (&code[next_non_whitespace_character_index..], new_index)
    }

    fn search_next_word(code: &str, code_index: usize) -> (&str, &str, usize) {
        let next_delimiter_character_index = {
            code.chars()
                .position(character_is_delimiter)
                .unwrap_or(code.len())
        };
        let current_character_is_delimiter = next_delimiter_character_index == 0;
        let word = if current_character_is_delimiter {
            &code[0..1] // BUG: Won't work for multi-character delimiters like ++ --
        } else {
            &code[0..next_delimiter_character_index]
        };
        (word, &code[word.len()..], code_index + word.len())
    }

    fn character_is_delimiter(c: char) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match c {
            '0'..='9' => false,
            'a'..='z' => false,
            'A'..='Z' => false,
            '_' => false,
            _ => true,
        }
    }

    fn character_is_integer(c: char) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match c {
            '0'..='9' => true,
            '_' => true,
            _ => false,
        }
    }

    fn character_is_identifier(c: char) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match c {
            '0'..='9' => true,
            'a'..='z' => true,
            'A'..='Z' => true,
            '_' => true,
            _ => false,
        }
    }

    Ok((token, remaining_code, span.end as usize))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize_string() -> Result<()> {
        let _a = tokenize_string("fn main() { }")?;
        let _b = tokenize_string("fn main() int { return 0; }")?;
        let _c = tokenize_string("fn main() int { let x = 0; return x; }")?;
        let _d = tokenize_string("fn main() int { let x:int = 0; return x; }")?;
        let _e = tokenize_string("fn main() int {\n 0\n}")?;
        let _f = tokenize_string("fn main() int {\n let x = 0\n x\n}")?;
        let _g = tokenize_string("fn main() int {\n let x:int = 0\n x\n}")?;
        let _h = tokenize_string("/// this is a documentation comment\n fn main() {}")?;
        let _i = tokenize_string("// this is a comment\n fn main() {}")?;
        let _j = tokenize_string("// this is a comment /// with a nested comment\n fn main() {}")?;
        let _k = tokenize_string("0 _ _0 0_ 000_000_000")?;
        let _l = tokenize_string("123_456_789")?;
        Ok(())
    }
}
