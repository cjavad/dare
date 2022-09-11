use std::{iter::Peekable, str::Chars};

use crate::{Delimiter, Error, Operator, Span, Token, TokenKind};

struct Lexer<'a> {
    index: usize,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            index: 0,
            chars: source.chars().peekable(),
        }
    }

    const fn span(&self) -> Span {
        Span::new(self.index, 0)
    }

    fn is_empty(&mut self) -> bool {
        self.skip_whitespace();
        self.peek().is_none()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.index += ch.len_utf8();
        Some(ch)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, char::is_whitespace) {
            self.next();
        }
    }

    fn parse_symbol(&mut self) -> Result<TokenKind, Error> {
        let span = self.span();
        let ch = self.next().ok_or_else(|| {
            Error::new()
                .with_msg("unexpected end of file")
                .with_span(span)
        })?;
        let next = self.peek();

        Ok(match (ch, next) {
            ('(', _) => TokenKind::Delimiter(Delimiter::Open),
            (')', _) => TokenKind::Delimiter(Delimiter::Close),
            ('¬', _) | ('~', _) | ('!', _) => TokenKind::Operator(Operator::Negation),
            ('&', Some('&')) => {
                self.next();
                TokenKind::Operator(Operator::Conjunction)
            }
            ('∧', _) | ('&', _) | ('.', _) => TokenKind::Operator(Operator::Conjunction),
            ('|', Some('|')) => {
                self.next();
                TokenKind::Operator(Operator::Disjunction)
            }
            ('∨', _) | ('|', _) => TokenKind::Operator(Operator::Disjunction),
            ('-', Some('>')) => {
                self.next();
                TokenKind::Operator(Operator::Implication)
            }
            ('→', _) | ('⇒', _) | ('⊃', _) => TokenKind::Operator(Operator::Implication),
            ('=', Some('=')) => {
                self.next();
                TokenKind::Operator(Operator::Equivalence)
            }
            ('<', Some('-')) => {
                self.next();

                let span = self.span();
                if self.next() != Some('>') {
                    let error = Error::new()
                        .with_msg("expected symbol '>'")
                        .with_span(span + self.span());

                    return Err(error);
                }

                TokenKind::Operator(Operator::Equivalence)
            }
            ('↔', _) | ('⇔', _) | ('≡', _) => TokenKind::Operator(Operator::Equivalence),
            _ => {
                return {
                    let error = Error::new()
                        .with_msg(format!("unexpected symbol '{}'", ch))
                        .with_span(span + self.span());

                    Err(error)
                }
            }
        })
    }

    fn is_identifier_first_char(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_identifier_char(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    fn is_identifier(&mut self) -> bool {
        self.peek().map_or(false, Self::is_identifier_first_char)
    }

    /// **Note** ``self.next()`` must be a valid starting character for an identifier.
    fn parse_identifier(&mut self) -> TokenKind {
        let mut identifier = String::new();

        while self.peek().map_or(false, Self::is_identifier_char) {
            identifier.push(self.next().unwrap());
        }

        TokenKind::Identifier(identifier)
    }

    fn parse_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();

        let start = self.span();
        if self.is_identifier() {
            let identifier = self.parse_identifier();

            return Ok(Token::new(identifier, start + self.span()));
        }

        let symbol = self.parse_symbol()?;
        Ok(Token::new(symbol, start + self.span()))
    }
}

/// A stream of [`Token`]s used by the parser.
pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    /// Tries to parse `source` as a [`TokenStream`].
    pub fn parse(source: &str) -> Result<Self, Error> {
        let mut lexer = Lexer::new(source);

        let mut tokens = Vec::new();
        while !lexer.is_empty() {
            tokens.push(lexer.parse_token()?);
        }

        Ok(Self { tokens, index: 0 })
    }

    /// Returns `true` if there are no more [`Token`]s left in `self`.
    pub fn is_empty(&self) -> bool {
        self.index == self.tokens.len()
    }

    /// Returns the next [`Token`] in `self` and moves the stream forward by one.
    ///
    /// Returns [`None`] if [`Self::is_empty`].
    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }

    /// Returns the next [`Token`] in `self`.
    ///
    /// Returns [`None`] if [`Self::is_empty`].
    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_stream_parsing() {
        let source = r#"A ab _a _0_a ( ) ¬ ~ ! && ∧ & . || ∨ | -> → ⇒ ⊃ == <-> ↔ ⇔ ≡"#;
        let token_stream = TokenStream::parse(source).unwrap();

        let tokens = [
            TokenKind::Identifier(String::from("A")),
            TokenKind::Identifier(String::from("ab")),
            TokenKind::Identifier(String::from("_a")),
            TokenKind::Identifier(String::from("_0_a")),
            TokenKind::Delimiter(Delimiter::Open),
            TokenKind::Delimiter(Delimiter::Close),
            TokenKind::Operator(Operator::Negation),
            TokenKind::Operator(Operator::Negation),
            TokenKind::Operator(Operator::Negation),
            TokenKind::Operator(Operator::Conjunction),
            TokenKind::Operator(Operator::Conjunction),
            TokenKind::Operator(Operator::Conjunction),
            TokenKind::Operator(Operator::Conjunction),
            TokenKind::Operator(Operator::Disjunction),
            TokenKind::Operator(Operator::Disjunction),
            TokenKind::Operator(Operator::Disjunction),
            TokenKind::Operator(Operator::Implication),
            TokenKind::Operator(Operator::Implication),
            TokenKind::Operator(Operator::Implication),
            TokenKind::Operator(Operator::Implication),
            TokenKind::Operator(Operator::Equivalence),
            TokenKind::Operator(Operator::Equivalence),
            TokenKind::Operator(Operator::Equivalence),
            TokenKind::Operator(Operator::Equivalence),
            TokenKind::Operator(Operator::Equivalence),
        ];

        for (i, token) in token_stream.tokens.into_iter().enumerate() {
            assert_eq!(*token.kind(), tokens[i]);
        }
    }
}
