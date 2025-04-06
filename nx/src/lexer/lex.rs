use crate::error::{ensure, Result};
//
use crate::lexer::token::Token;

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

    // Tokenize one by one until the code is not empty
    while !code.is_empty() {
        // Find next token
        let (token, remaining_code) = tokenize_next_word(code)?;
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
pub fn tokenize_next_word(code: &str) -> Result<(Token, &str)> {
    let code = truncate_leading_whitespace(code);

    let (word, remaining_code) = search_next_word(code);

    // Tokenize documentation comments
    if word == "/" {
        let (next_word_1, remaining_code) = search_next_word(remaining_code);
        let (next_word_2, remaining_code) = search_next_word(remaining_code);
        if next_word_1 == "/" && next_word_2 == "/" {
            let next_newline_index = remaining_code
                .chars()
                .position(|c| c == '\n')
                .unwrap_or(remaining_code.len());
            let remaining_truncated_code = &remaining_code[next_newline_index..];
            let comment = &code[0..(next_newline_index + 3)];
            let token = Token::Documentation(comment);
            return Ok((token, remaining_truncated_code));
        }
    }

    // Tokenize comments
    if word == "/" {
        let (next_word, remaining_code) = search_next_word(remaining_code);
        if next_word == "/" {
            let next_newline_index = remaining_code
                .chars()
                .position(|c| c == '\n')
                .unwrap_or(remaining_code.len());
            let remaining_truncated_code = &remaining_code[next_newline_index..];
            let comment = &code[0..(next_newline_index + 2)];
            let token = Token::Comment(comment);
            return Ok((token, remaining_truncated_code));
        }
    }

    // Tokenize reserved words
    let token = match word {
        // Symbols
        ";" => Token::Semicolon,
        ":" => Token::Colon,
        "," => Token::Comma,
        "." => Token::Dot,
        //
        "=" => Token::Equal,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Star,
        "/" => Token::Slash,
        //
        "(" => Token::LParenthesis,
        ")" => Token::RParenthesis,
        "[" => Token::LBracket,
        "]" => Token::RBracket,
        "<" => Token::LAngle,
        ">" => Token::RAngle,
        "{" => Token::LBrace,
        "}" => Token::RBrace,
        //
        "!" => Token::Exclamation,
        "?" => Token::Question,
        "$" => Token::Dollar,
        "#" => Token::Hash,

        // Directives
        "use" => Token::Use,
        "let" => Token::Let,
        "var" => Token::Var,
        "as" => Token::As,
        "in" => Token::In,
        "return" => Token::Return,
        "break" => Token::Break,
        "continue" => Token::Continue,

        // Blocks
        "macro" => Token::Macro,
        "module" => Token::Module,
        "fn" => Token::Fn,
        "struct" => Token::Struct,
        "enum" => Token::Enum,
        "instance" => Token::Instance,
        "implement" => Token::Implement,
        "match" => Token::Match,
        "if" => Token::If,
        "else" => Token::Else,
        "for" => Token::For,
        "while" => Token::While,
        "loop" => Token::Loop,

        // Types
        "()" => Token::Unit,
        "usize" => Token::Usize,
        "int" => Token::Int,
        "flt" => Token::Flt,
        "str" => Token::Str,
        "i8" => Token::I8,
        "u8" => Token::U8,
        "i16" => Token::I16,
        "u16" => Token::U16,
        "i32" => Token::I32,
        "u32" => Token::U32,
        "i64" => Token::I64,
        "u64" => Token::U64,
        "f32" => Token::F32,
        "f64" => Token::F64,

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
                Token::IntVal(word)
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
                Token::IdxVal(word)
            } else {
                Token::None
            }
        };
        (new_token, remaining_code)
    } else {
        (token, remaining_code)
    };

    // where ...

    fn truncate_leading_whitespace(code: &str) -> &str {
        let next_non_whitespace_character_index = {
            code.chars()
                .position(|c| c != ' ' && c != '\t' && c != '\n')
                .unwrap_or(0)
        };
        &code[next_non_whitespace_character_index..]
    }

    fn search_next_word(code: &str) -> (&str, &str) {
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
        (word, &code[word.len()..])
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

    Ok((token, remaining_code))
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
