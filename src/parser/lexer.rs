use std::iter::Peekable;
use std::{iter::FromIterator, str::CharIndices};

pub type Loc = usize;
pub type SpannedTok = (Loc, Result<Tok, LexicalError>, Loc);

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    QString(String),
    UString(String),
    Plus,
    Semicolon,
    LBrace,
    RBrace,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LexicalError {
    UnexpectedEOF,
    IllegalToken,
    IllegalStringEscape,
}

pub struct Lexer<'input> {
    chars: Peekable<CharIndices<'input>>,
    line_loc: usize,
}

#[derive(Copy, Clone)]
struct LocatedChar(Loc, char);

impl LocatedChar {
    fn from((loc, ch): (Loc, char)) -> Self {
        LocatedChar(loc, ch)
    }

    fn loc(self) -> Loc {
        self.0
    }

    fn ch(self) -> char {
        self.1
    }
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let chars = input.char_indices().peekable();
        Lexer { chars, line_loc: 0 }
    }

    fn pop(&mut self) -> Option<LocatedChar> {
        let next_char = self.chars.next().map(LocatedChar::from);
        if let Some(ch) = next_char {
            if ch.1 == '\n' {
                self.line_loc = 0;
            } else {
                self.line_loc += 1;
            }
        }
        next_char
    }

    fn pop_or_eof(&mut self) -> Result<LocatedChar, LexicalError> {
        self.pop().ok_or(LexicalError::UnexpectedEOF)
    }

    fn peek(&mut self) -> Option<LocatedChar> {
        self.chars.peek().map(|(loc, ch)| LocatedChar(*loc, *ch))
    }

    fn consume_linecomment(&mut self) {
        while let Some(ch) = self.pop() {
            if ch.1 == '\n' {
                break;
            }
        }
    }

    fn consume_escaped_char(&mut self) -> Result<char, LexicalError> {
        let ch = self.pop_or_eof()?.ch();
        Ok(match ch {
            '\\' => '\\',
            '"' => '"',
            'n' => '\n',
            't' => '\t',
            _ => return Err(LexicalError::IllegalStringEscape),
        })
    }

    fn consume_unquoted_string(&mut self, first_char: LocatedChar) -> SpannedTok {
        let mut string_content = Vec::<char>::new();
        let mut last_pos = first_char.0;
        string_content.push(first_char.ch());
        loop {
            let ch = self.peek();
            match ch {
                Some(ch)
                    if ch.ch().is_ascii_alphanumeric()
                        || ch.ch() == '_'
                        || ch.ch() == '.'
                        || ch.ch() == ':'
                        || ch.ch() == '-' =>
                {
                    self.pop();
                    last_pos = ch.0;
                    string_content.push(ch.ch())
                }
                _ => break,
            }
        }

        (
            first_char.0,
            Ok(Tok::UString(String::from_iter(string_content))),
            last_pos + 1,
        )
    }

    fn consume_quoted_string(&mut self, starting_pos: Loc, is_double_quoted: bool) -> SpannedTok {
        let mut string_content = Vec::<char>::new();
        let first_indent = self.line_loc;
        let mut loc = starting_pos;
        loop {
            let ch = self.pop_or_eof();
            loc += 1;
            let ch = if let Ok(ch) = ch {
                ch
            } else {
                return (starting_pos, Err(LexicalError::UnexpectedEOF), loc);
            };
            match ch.ch() {
                '\'' if !is_double_quoted => {
                    return (
                        starting_pos,
                        Ok(Tok::QString(String::from_iter(string_content))),
                        ch.loc() + 1,
                    );
                }
                '"' if is_double_quoted => {
                    let trimmed_string_content = string_content
                        .split(|c| *c == '\n')
                        .map(|s| {
                            let s = String::from_iter(s);
                            let s = s.trim_end();
                            let mut pos = 0;
                            let s = s.trim_start_matches(|c| {
                                if pos >= first_indent {
                                    return false;
                                } else if c == '\t' {
                                    pos += 8;
                                    true
                                } else if c == ' ' {
                                    pos += 1;
                                    true
                                } else {
                                    false
                                }
                            });
                            s.to_string()
                        })
                        .collect::<Vec<_>>()
                        .join("\n");

                    return (
                        starting_pos,
                        Ok(Tok::QString(trimmed_string_content)),
                        ch.loc() + 1,
                    );
                }
                '\\' => {
                    let escaped_char = if let Ok(ch) = self.consume_escaped_char() {
                        ch
                    } else {
                        return (starting_pos, Err(LexicalError::IllegalStringEscape), loc);
                    };
                    string_content.push(escaped_char)
                },
                c => string_content.push(c),
            }
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = SpannedTok;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_char) = self.pop() {
            return Some(match next_char.ch() {
                ch if ch.is_ascii_alphanumeric() || ch == '_' => {
                    return Some(self.consume_unquoted_string(next_char));
                }
                ch if ch.is_whitespace() => continue,
                '/' => {
                    let loc = next_char.loc();
                    let next_char = self.pop();
                    if next_char.map(|c| c.ch()) != Some('/') {
                        return Some((loc, Err(LexicalError::IllegalToken), loc + 2));
                    }
                    self.consume_linecomment();
                    continue;
                }
                '"' => return Some(self.consume_quoted_string(next_char.loc(), true)),
                '\'' => return Some(self.consume_quoted_string(next_char.loc(), false)),
                ';' => (next_char.loc(), Ok(Tok::Semicolon), next_char.loc() + 1),
                '+' => (next_char.loc(), Ok(Tok::Plus), next_char.loc() + 1),
                '{' => (next_char.loc(), Ok(Tok::LBrace), next_char.loc() + 1),
                '}' => (next_char.loc(), Ok(Tok::RBrace), next_char.loc() + 1),
                _ => {
                    return Some((
                        next_char.0,
                        Err(LexicalError::IllegalToken),
                        next_char.0 + 1,
                    ))
                }
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens(
        input: &str,
        expected_tokens: Vec<(Loc, Tok, Loc)>,
        error: Option<LexicalError>,
    ) {
        let mut lexer = Lexer::new(input);

        for expected_token in expected_tokens {
            let (loc, actual_token, end_loc) = lexer.next().unwrap();
            assert_eq!((loc, actual_token.unwrap(), end_loc), expected_token);
        }

        if let Some(error) = error {
            assert_eq!(lexer.next().unwrap().1.err().unwrap(), error);
        } else {
            assert_eq!(lexer.next(), None);
        }
    }

    #[test]
    fn test_invalid_token() {
        let input = "#2423";
        assert_tokens(input, vec![], Some(LexicalError::IllegalToken));
    }

    #[test]
    fn test_valid_token() {
        let input = r#"
leaf host-name {
    type string;
    description
        "Hostname for this system." +
        'This is a string';
    test-multiline
        "This is a
           multiline string";
    test-singlequote
        'This is a single"
           \" quote string';
}
        "#;
        assert_tokens(
            input,
            vec![
                (1, Tok::UString("leaf".to_string()), 5),
                (6, Tok::UString("host-name".to_string()), 15),
                (16, Tok::LBrace, 17),
                (22, Tok::UString("type".to_string()), 26),
                (27, Tok::UString("string".to_string()), 33),
                (33, Tok::Semicolon, 34),
                (39, Tok::UString("description".to_string()), 50),
                (
                    59,
                    Tok::QString("Hostname for this system.".to_string()),
                    86,
                ),
                (87, Tok::Plus, 88),
                (97, Tok::QString("This is a string".to_string()), 115),
                (115, Tok::Semicolon, 116),
                (121, Tok::UString("test-multiline".to_string()), 135),
                (
                    144,
                    Tok::QString("This is a\n  multiline string".to_string()),
                    183,
                ),
                (183, Tok::Semicolon, 184),
                (189, Tok::UString("test-singlequote".to_string()), 205),
                (
                    214,
                    Tok::QString("This is a single\"\n           \" quote string".to_string()),
                    260,
                ),
                (260, Tok::Semicolon, 261),
                (262, Tok::RBrace, 263),
            ],
            None,
        );
    }
}
