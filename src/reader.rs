use std::io;

fn is_identifier(c: char) -> bool {
    return c.is_alphabetic() || "-_@#$+=*&^%!".contains(c);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'input> {
    LParen,
    RParen,
    Quote,
    Symbol(&'input str),
    Number(&'input str),
    String(&'input str),
    WhiteSpace(&'input str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokItem<'input> {
    pub token: Token<'input>,
    pub position: usize,
}

pub struct Scanner<'input> {
    current_pos: usize,
    text: &'input str,
}

impl<'input> Scanner<'input> {
    pub fn new(text: &'input str) -> Self {
        Self {
            current_pos: 0,
            text,
        }
    }
    fn peek(&self) -> Result<char, io::Error> {
        return self
            .text
            .chars()
            .nth(self.current_pos)
            .ok_or(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected EOF",
            ));
    }
    fn advance(&mut self) -> Result<char, io::Error> {
        match self.peek() {
            Ok(ch) => {
                self.current_pos += 1;
                Ok(ch)
            }
            Err(err) => Err(err),
        }
    }

    pub fn next(&mut self) -> Result<TokItem<'input>, io::Error> {
        let ch = self.peek()?;
        match ch {
            '\'' => {
                self.advance().unwrap();
                Ok(TokItem {
                    token: Token::Quote,
                    position: self.current_pos.saturating_sub(1),
                })
            }
            '\"' => {
                let start = self.current_pos;
                self.advance().unwrap();
                let string_content = self.advance_while(|ch| ch != '"').unwrap().unwrap();
                assert_eq!(self.peek().unwrap(), '"');
                self.advance().unwrap();
                Ok(TokItem {
                    token: Token::String(string_content),
                    position: start,
                })
            }
            // ignore whitespaces
            x if x.is_whitespace() => {
                // let start = self.current_pos;
                // while let Ok(x) = self.peek() {
                //     if x.is_whitespace() {
                //         self.advance()?;
                //     } else {
                //     }
                // }
                let start = self.current_pos;
                let spaces = self.advance_while(char::is_whitespace).unwrap().unwrap();

                Ok(TokItem {
                    token: Token::WhiteSpace(spaces),
                    position: start,
                })
            }

            x if x.is_digit(10) => {
                let start = self.current_pos;
                let number = self
                    .advance_while(|c| c.is_digit(10) || c == '.')
                    .unwrap()
                    .unwrap();
                Ok(TokItem {
                    token: Token::Number(number),
                    position: start,
                })
            }
            x if is_identifier(x) => {
                let start = self.current_pos;
                let identifer = self.advance_while(is_identifier).unwrap().unwrap();
                Ok(TokItem {
                    token: Token::Symbol(identifer),
                    position: start,
                })
            }
            '(' => {
                self.advance().unwrap();
                Ok(TokItem {
                    token: Token::LParen,
                    position: self.current_pos.saturating_sub(1),
                })
            }
            ')' => {
                self.advance().unwrap();
                Ok(TokItem {
                    token: Token::RParen,
                    position: self.current_pos.saturating_sub(1),
                })
            }

            _ => unreachable!(),
        }
    }

    fn advance_while<F: Fn(char) -> bool>(
        &mut self,
        check: F,
    ) -> Option<Result<&'input str, io::Error>> {
        let start = self.current_pos;
        match self.peek() {
            Ok(ch) if check(ch) => {
                while let Ok(ch) = self.peek() {
                    if check(ch) {
                        self.advance().unwrap();
                    } else {
                        break;
                    }
                }
                Some(Ok(&self.text[start..self.current_pos]))
            }
            Err(err) => {
                if self.current_pos != start {
                    Some(Ok(&self.text[start..self.current_pos]))
                } else {
                    Some(Err(err))
                }
            }
            _ => None,
        }
    }

    pub fn scan_all(&mut self) -> Vec<TokItem> {
        let mut result = Vec::new();
        while let Ok(tok) = self.next() {
            result.push(tok)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_initialization() {
        let scanner = Scanner::new("Hello World");
        assert_eq!(scanner.text, "Hello World");
    }

    #[test]
    #[should_panic]
    fn empty_text_gives_eof() {
        let mut scanner = Scanner::new("");
        let _result = scanner.next().unwrap();
    }

    #[test]
    fn test_scanner_accepts_whitespace() {
        let mut scanner = Scanner::new("     ");
        let result = scanner.next().unwrap();
        assert_eq!(
            result,
            TokItem {
                token: Token::WhiteSpace("     "),
                position: 0
            }
        )
    }

    #[test]
    #[should_panic]
    fn test_scanner_eof() {
        let mut scanner = Scanner::new("     ");
        scanner.next().unwrap();
        scanner.next().unwrap();
    }

    #[test]
    fn test_scanner_accepts_symbol() {
        let mut scanner = Scanner::new("abcde");
        let result = scanner.next().unwrap();
        assert_eq!(
            result,
            TokItem {
                token: Token::Symbol("abcde"),
                position: 0
            }
        )
    }

    #[test]
    fn test_scanner_accepts_symbol_and_space() {
        let mut scanner = Scanner::new("abcde  ");
        let result = scanner.scan_all();
        assert_eq!(
            result,
            vec![
                TokItem {
                    token: Token::Symbol("abcde"),
                    position: 0
                },
                TokItem {
                    token: Token::WhiteSpace("  "),
                    position: 5
                }
            ]
        );
    }

    #[test]
    fn test_scanner_accepts_parenthesis() {
        let mut scanner = Scanner::new("()");
        let result = scanner.scan_all();
        assert_eq!(
            result,
            vec![
                TokItem {
                    token: Token::LParen,
                    position: 0
                },
                TokItem {
                    token: Token::RParen,
                    position: 1
                }
            ]
        );
    }

    #[test]
    fn test_scanner_accepts_integer() {
        let mut scanner = Scanner::new("1234");
        let result = scanner.scan_all();
        assert_eq!(
            result,
            vec![TokItem {
                token: Token::Number("1234"),
                position: 0
            },]
        );
    }

    #[test]
    fn test_scanner_accepts_float() {
        let mut scanner = Scanner::new("1234.567");
        let result = scanner.scan_all();
        assert_eq!(
            result,
            vec![TokItem {
                token: Token::Number("1234.567"),
                position: 0
            },]
        );
    }

    #[test]
    fn test_scanner_accepts_quote() {
        let mut scanner = Scanner::new("'");
        let result = scanner.scan_all();
        assert_eq!(
            result,
            vec![TokItem {
                token: Token::Quote,
                position: 0
            },]
        );
    }

    #[test]
    fn test_scanner_accepts_string() {
        let mut scanner = Scanner::new("\"abcde\" \"a\"");
        let result = scanner.scan_all();
        assert_eq!(
            result,
            vec![
                TokItem {
                    token: Token::String("abcde"),
                    position: 0
                },
                TokItem {
                    token: Token::WhiteSpace(" "),
                    position: 7
                },
                TokItem {
                    token: Token::String("a"),
                    position: 8
                }
            ]
        );
    }
}
