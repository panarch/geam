use crate::ast::SrcSpan;
use crate::parse::error::{InvalidUnicodeEscapeError, LexicalError, LexicalErrorType};
use crate::parse::token::Token;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned {
    pub start: u32,
    pub token: Token,
    pub end: u32,
}

impl Spanned {
    pub fn location(&self) -> SrcSpan {
        SrcSpan::new(self.start, self.end)
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Spanned>, LexicalError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

struct Lexer<'a> {
    source: &'a str,
    chars: Vec<(usize, char)>,
    index: usize,
    tokens: Vec<Spanned>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().collect(),
            index: 0,
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Spanned>, LexicalError> {
        if self.current_char() == Some('\u{feff}') {
            self.bump();
        }

        while let Some((_, chr)) = self.current() {
            if chr == '\n' || chr == '\r' {
                if let Some((start, token, end)) = self.lex_newline() {
                    self.tokens.push(Spanned {
                        start: start as u32,
                        token,
                        end: end as u32,
                    });
                }
                continue;
            }

            if chr.is_whitespace() {
                self.bump();
                continue;
            }

            if chr == '/' && self.peek_char(1) == Some('/') {
                self.skip_line_comment();
                continue;
            }

            let token = if is_name_start(chr) {
                self.lex_name()
            } else if chr.is_ascii_digit() {
                self.lex_number()?
            } else {
                self.lex_symbol_or_string()?
            };

            if let Some((start, token, end)) = token {
                self.tokens.push(Spanned {
                    start: start as u32,
                    token,
                    end: end as u32,
                });
            }
        }

        let end = self.source.len() as u32;
        self.tokens.push(Spanned {
            start: end,
            token: Token::EndOfFile,
            end,
        });

        Ok(std::mem::take(&mut self.tokens))
    }

    fn current(&self) -> Option<(usize, char)> {
        self.chars.get(self.index).copied()
    }

    fn current_char(&self) -> Option<char> {
        self.current().map(|(_, chr)| chr)
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        self.chars.get(self.index + offset).map(|(_, chr)| *chr)
    }

    fn current_byte(&self) -> usize {
        self.current()
            .map(|(index, _)| index)
            .unwrap_or_else(|| self.source.len())
    }

    fn bump(&mut self) -> Option<(usize, char, usize)> {
        let (start, chr) = self.current()?;
        self.index += 1;
        let end = start + chr.len_utf8();
        Some((start, chr, end))
    }

    fn skip_line_comment(&mut self) {
        while let Some((_, chr)) = self.current() {
            if chr == '\n' || chr == '\r' {
                break;
            }
            self.bump();
        }
    }

    fn lex_newline(&mut self) -> Option<(usize, Token, usize)> {
        let (start, chr, mut end) = self.bump()?;
        if chr == '\r' && self.current_char() == Some('\n') {
            let (_, _, next_end) = self.bump().expect("line feed disappeared");
            end = next_end;
        }
        Some((start, Token::NewLine, end))
    }

    fn lex_name(&mut self) -> Option<(usize, Token, usize)> {
        let (start, first, mut end) = self.bump()?;
        let mut word = String::new();
        word.push(first);

        while let Some((_, chr)) = self.current() {
            if is_name_continue(chr) {
                let (_, chr, next_end) = self.bump().expect("current char disappeared");
                word.push(chr);
                end = next_end;
            } else {
                break;
            }
        }

        let token = match word.as_str() {
            "as" => Token::As,
            "assert" => Token::Assert,
            "case" => Token::Case,
            "const" => Token::Const,
            "echo" => Token::Echo,
            "fn" => Token::Fn,
            "if" => Token::If,
            "import" => Token::Import,
            "let" => Token::Let,
            "opaque" => Token::Opaque,
            "panic" => Token::Panic,
            "pub" => Token::Pub,
            "todo" => Token::Todo,
            "type" => Token::Type,
            "use" => Token::Use,
            _ if word.starts_with('_') => Token::DiscardName { name: word.into() },
            _ if first.is_uppercase() => Token::UpName { name: word.into() },
            _ => Token::Name { name: word.into() },
        };

        Some((start, token, end))
    }

    fn lex_number(&mut self) -> Result<Option<(usize, Token, usize)>, LexicalError> {
        let start = self.current_byte();

        if self.current_char() == Some('0') {
            match self.peek_char(1) {
                Some('b') | Some('B') => return self.lex_radix_int(start, 2, 2),
                Some('o') | Some('O') => return self.lex_radix_int(start, 8, 2),
                Some('x') | Some('X') => return self.lex_radix_int(start, 16, 2),
                _ => {}
            }
        }

        let mut value = String::new();
        let mut end = start;
        while let Some((_, chr)) = self.current() {
            if chr.is_ascii_digit() || chr == '_' {
                let (_, chr, next_end) = self.bump().expect("current char disappeared");
                value.push(chr);
                end = next_end;
            } else {
                break;
            }
        }

        let mut is_float = false;
        if self.current_char() == Some('.') && self.peek_char(1).is_some_and(|c| c.is_ascii_digit())
        {
            is_float = true;
            let (_, chr, next_end) = self.bump().expect("current char disappeared");
            value.push(chr);
            end = next_end;

            while let Some((_, chr)) = self.current() {
                if chr.is_ascii_digit() || chr == '_' {
                    let (_, chr, next_end) = self.bump().expect("current char disappeared");
                    value.push(chr);
                    end = next_end;
                } else {
                    break;
                }
            }
        }

        if matches!(self.current_char(), Some('e' | 'E')) {
            is_float = true;
            let (_, chr, next_end) = self.bump().expect("current char disappeared");
            value.push(chr);
            end = next_end;

            if matches!(self.current_char(), Some('+' | '-')) {
                let (_, chr, next_end) = self.bump().expect("current char disappeared");
                value.push(chr);
                end = next_end;
            }

            let exponent_start = self.current_byte();
            let mut exponent_digits = 0;
            while let Some((_, chr)) = self.current() {
                if chr.is_ascii_digit() || chr == '_' {
                    if chr.is_ascii_digit() {
                        exponent_digits += 1;
                    }
                    let (_, chr, next_end) = self.bump().expect("current char disappeared");
                    value.push(chr);
                    end = next_end;
                } else {
                    break;
                }
            }

            if exponent_digits == 0 {
                return Err(self.error(
                    LexicalErrorType::MissingExponent,
                    SrcSpan::new(exponent_start as u32, exponent_start as u32),
                ));
            }
        }

        if value.ends_with('_') {
            return Err(self.error(
                LexicalErrorType::NumTrailingUnderscore,
                SrcSpan::new((end - 1) as u32, (end - 1) as u32),
            ));
        }

        let token = if is_float {
            Token::Float {
                value: value.into(),
            }
        } else {
            let clean = value.replace('_', "");
            let int_value = BigInt::parse_bytes(clean.as_bytes(), 10).unwrap_or_default();
            Token::Int {
                value: value.into(),
                int_value,
            }
        };

        Ok(Some((start, token, end)))
    }

    fn lex_radix_int(
        &mut self,
        start: usize,
        radix: u32,
        prefix_len: usize,
    ) -> Result<Option<(usize, Token, usize)>, LexicalError> {
        let mut value = String::new();
        let mut end = start;

        for _ in 0..prefix_len {
            let (_, chr, next_end) = self.bump().expect("radix prefix disappeared");
            value.push(chr);
            end = next_end;
        }

        let digits_start = self.current_byte();
        let mut digits = String::new();
        while let Some((_, chr)) = self.current() {
            if chr.is_ascii_alphanumeric() || chr == '_' {
                let (_, chr, next_end) = self.bump().expect("current char disappeared");
                value.push(chr);
                digits.push(chr);
                end = next_end;
            } else {
                break;
            }
        }

        if digits.is_empty() {
            return Err(self.error(
                LexicalErrorType::RadixIntNoValue,
                SrcSpan::new((digits_start - 1) as u32, (digits_start - 1) as u32),
            ));
        }

        if digits.ends_with('_') {
            return Err(self.error(
                LexicalErrorType::NumTrailingUnderscore,
                SrcSpan::new((end - 1) as u32, (end - 1) as u32),
            ));
        }

        for (offset, chr) in digits.char_indices() {
            if chr == '_' {
                continue;
            }
            if chr.to_digit(radix).is_none() {
                let location = digits_start + offset;
                return Err(self.error(
                    LexicalErrorType::DigitOutOfRadix,
                    SrcSpan::new(location as u32, location as u32),
                ));
            }
        }

        let clean = digits.replace('_', "");
        let int_value = BigInt::parse_bytes(clean.as_bytes(), radix).unwrap_or_default();
        Ok(Some((
            start,
            Token::Int {
                value: value.into(),
                int_value,
            },
            end,
        )))
    }

    fn lex_symbol_or_string(&mut self) -> Result<Option<(usize, Token, usize)>, LexicalError> {
        let (start, chr, end) = self.bump().expect("symbol disappeared");
        let token = match chr {
            '"' => return self.lex_string(start),
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftSquare,
            ']' => Token::RightSquare,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '#' => Token::Hash,
            '@' => Token::At,
            '%' => Token::Percent,
            '+' if self.current_char() == Some('.') => {
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::PlusDot, end)));
            }
            '+' => Token::Plus,
            '-' if self.current_char() == Some('.') => {
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::MinusDot, end)));
            }
            '-' if self.current_char() == Some('>') => {
                let (_, _, end) = self.bump().expect("arrow disappeared");
                return Ok(Some((start, Token::RArrow, end)));
            }
            '-' => Token::Minus,
            '*' if self.current_char() == Some('.') => {
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::StarDot, end)));
            }
            '*' => Token::Star,
            '/' if self.current_char() == Some('.') => {
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::SlashDot, end)));
            }
            '/' => Token::Slash,
            '<' if self.current_char() == Some('=') && self.peek_char(1) == Some('.') => {
                self.bump();
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::LessEqualDot, end)));
            }
            '<' if self.current_char() == Some('=') => {
                let (_, _, end) = self.bump().expect("equal disappeared");
                return Ok(Some((start, Token::LessEqual, end)));
            }
            '<' if self.current_char() == Some('.') => {
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::LessDot, end)));
            }
            '<' if self.current_char() == Some('>') => {
                let (_, _, end) = self.bump().expect("concat disappeared");
                return Ok(Some((start, Token::Concatenate, end)));
            }
            '<' if self.current_char() == Some('<') => {
                let (_, _, end) = self.bump().expect("ltlt disappeared");
                return Ok(Some((start, Token::LtLt, end)));
            }
            '<' if self.current_char() == Some('-') => {
                let (_, _, end) = self.bump().expect("larrow disappeared");
                return Ok(Some((start, Token::LArrow, end)));
            }
            '<' => Token::Less,
            '>' if self.current_char() == Some('=') && self.peek_char(1) == Some('.') => {
                self.bump();
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::GreaterEqualDot, end)));
            }
            '>' if self.current_char() == Some('=') => {
                let (_, _, end) = self.bump().expect("equal disappeared");
                return Ok(Some((start, Token::GreaterEqual, end)));
            }
            '>' if self.current_char() == Some('.') => {
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::GreaterDot, end)));
            }
            '>' if self.current_char() == Some('>') => {
                let (_, _, end) = self.bump().expect("gtgt disappeared");
                return Ok(Some((start, Token::GtGt, end)));
            }
            '>' => Token::Greater,
            '!' if self.current_char() == Some('=') => {
                let (_, _, end) = self.bump().expect("equal disappeared");
                return Ok(Some((start, Token::NotEqual, end)));
            }
            '!' => Token::Bang,
            '=' if self.current_char() == Some('=') => {
                let (_, _, end) = self.bump().expect("equal disappeared");
                return Ok(Some((start, Token::EqualEqual, end)));
            }
            '=' => Token::Equal,
            '|' if self.current_char() == Some('|') => {
                let (_, _, end) = self.bump().expect("pipe disappeared");
                return Ok(Some((start, Token::VbarVbar, end)));
            }
            '|' if self.current_char() == Some('>') => {
                let (_, _, end) = self.bump().expect("pipe disappeared");
                return Ok(Some((start, Token::Pipe, end)));
            }
            '|' => Token::Vbar,
            '&' if self.current_char() == Some('&') => {
                let (_, _, end) = self.bump().expect("ampersand disappeared");
                return Ok(Some((start, Token::AmperAmper, end)));
            }
            '.' if self.current_char() == Some('.') => {
                let (_, _, end) = self.bump().expect("dot disappeared");
                return Ok(Some((start, Token::DotDot, end)));
            }
            '.' => Token::Dot,
            _ => {
                return Err(self.error(
                    LexicalErrorType::UnrecognizedToken { tok: chr },
                    SrcSpan::new(start as u32, end as u32),
                ));
            }
        };

        Ok(Some((start, token, end)))
    }

    fn lex_string(&mut self, start: usize) -> Result<Option<(usize, Token, usize)>, LexicalError> {
        let mut value = String::new();

        while let Some((slash_or_quote_start, chr)) = self.current() {
            match chr {
                '"' => {
                    let (_, _, end) = self.bump().expect("quote disappeared");
                    return Ok(Some((
                        start,
                        Token::String {
                            value: value.into(),
                        },
                        end,
                    )));
                }
                '\\' => {
                    self.bump();
                    let Some((escape_start, escaped)) = self.current() else {
                        return Err(self.error(
                            LexicalErrorType::UnexpectedStringEnd,
                            SrcSpan::new(start as u32, self.source.len() as u32),
                        ));
                    };
                    match escaped {
                        '"' => {
                            value.push('"');
                            self.bump();
                        }
                        '\\' => {
                            value.push('\\');
                            self.bump();
                        }
                        'n' => {
                            value.push('\n');
                            self.bump();
                        }
                        'r' => {
                            value.push('\r');
                            self.bump();
                        }
                        't' => {
                            value.push('\t');
                            self.bump();
                        }
                        'u' => {
                            self.bump();
                            value.push(self.lex_unicode_escape(slash_or_quote_start)?);
                        }
                        _ => {
                            return Err(self.error(
                                LexicalErrorType::BadStringEscape,
                                SrcSpan::new(slash_or_quote_start as u32, escape_start as u32),
                            ));
                        }
                    }
                }
                _ => {
                    let (_, chr, _) = self.bump().expect("string character disappeared");
                    value.push(chr);
                }
            }
        }

        Err(self.error(
            LexicalErrorType::UnexpectedStringEnd,
            SrcSpan::new(start as u32, self.source.len() as u32),
        ))
    }

    fn lex_unicode_escape(&mut self, slash_start: usize) -> Result<char, LexicalError> {
        if self.current_char() != Some('{') {
            return Err(self.error(
                LexicalErrorType::InvalidUnicodeEscape(
                    InvalidUnicodeEscapeError::MissingOpeningBrace,
                ),
                SrcSpan::new(slash_start as u32, self.current_byte() as u32),
            ));
        }
        self.bump();

        let digits_start = self.current_byte();
        let mut digits = String::new();
        while let Some((_, chr)) = self.current() {
            if chr == '}' {
                break;
            }
            if !chr.is_ascii_hexdigit() {
                return Err(self.error(
                    LexicalErrorType::InvalidUnicodeEscape(
                        InvalidUnicodeEscapeError::ExpectedHexDigitOrCloseBrace,
                    ),
                    SrcSpan::new(self.current_byte() as u32, self.current_byte() as u32),
                ));
            }
            digits.push(chr);
            self.bump();
        }

        if !(1..=6).contains(&digits.len()) {
            return Err(self.error(
                LexicalErrorType::InvalidUnicodeEscape(
                    InvalidUnicodeEscapeError::InvalidNumberOfHexDigits,
                ),
                SrcSpan::new(digits_start as u32, self.current_byte() as u32),
            ));
        }

        if self.current_char() != Some('}') {
            return Err(self.error(
                LexicalErrorType::InvalidUnicodeEscape(
                    InvalidUnicodeEscapeError::ExpectedHexDigitOrCloseBrace,
                ),
                SrcSpan::new(self.current_byte() as u32, self.current_byte() as u32),
            ));
        }
        self.bump();

        let codepoint = u32::from_str_radix(&digits, 16).unwrap_or_default();
        char::from_u32(codepoint).ok_or_else(|| {
            self.error(
                LexicalErrorType::InvalidUnicodeEscape(InvalidUnicodeEscapeError::InvalidCodepoint),
                SrcSpan::new(digits_start as u32, self.current_byte() as u32),
            )
        })
    }

    fn error(&self, error: LexicalErrorType, location: SrcSpan) -> LexicalError {
        LexicalError { error, location }
    }
}

fn is_name_start(chr: char) -> bool {
    chr == '_' || chr.is_alphabetic()
}

fn is_name_continue(chr: char) -> bool {
    chr == '_' || chr.is_alphanumeric()
}

#[cfg(test)]
mod tests;
