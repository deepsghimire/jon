use std::io;

use thiserror::Error;

use crate::scanner::{self, Scanner, TokItem, Token};

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseError<'input> {
    #[error("End of Input")]
    EOF,
    #[error("Unexpected token {0}")]
    UnexpectedToken(TokItem<'input>),
}

#[derive(Debug, PartialEq)]
enum Atom {
    Symbol(String),
    Number(f32),
    String(String),
}

#[derive(Debug, PartialEq)]
enum Expr {
    Atom(Atom),
    List(List),
}

type List = Vec<Expr>;

pub struct Parser<'input> {
    tokens: Vec<TokItem<'input>>,
    current_pos: usize,
}

impl<'input> Parser<'input> {
    fn new(scanner: &mut Scanner<'input>) -> Self {
        Self {
            tokens: scanner
                .scan_all()
                .into_iter()
                .filter(|x| {
                    !matches!(
                        x,
                        TokItem {
                            token: Token::WhiteSpace(_),
                            position: _
                        }
                    )
                })
                .collect(),
            current_pos: 0,
        }
    }

    fn get_token(&self) -> Result<&TokItem<'input>, ParseError<'input>> {
        self.tokens.get(self.current_pos).ok_or(ParseError::EOF)
    }

    fn at_eof(&self) -> bool {
        return self.current_pos >= self.tokens.len();
    }

    fn advance(&mut self) {
        self.current_pos += 1;
    }

    fn match_token(&mut self, tok: &Token) -> Result<(), ParseError<'input>> {
        eprintln!("{}:{:?}: matching {:?}", self.current_pos, self.tokens, tok);
        match self.get_token()? {
            TokItem {
                token: t,
                position: _,
            } if t == tok => {
                self.advance();
                Ok(())
            }
            x => Err(ParseError::UnexpectedToken(*x)),
        }
    }

    fn parse_atom(&mut self) -> Result<Atom, ParseError<'input>> {
        if self.at_eof() {
            return Err(ParseError::EOF);
        };

        let result = match self.get_token()? {
            TokItem {
                token: Token::Number(n),
                position: _,
            } => Ok(Atom::Number(n.parse().unwrap())),
            TokItem {
                token: Token::String(s),
                position: _,
            } => Ok(Atom::String((*s).into())),
            TokItem {
                token: Token::Symbol(s),
                position: _,
            } => Ok(Atom::Symbol((*s).into())),

            x => Err(ParseError::UnexpectedToken(*x)),
        };

        if let Ok(_) = result {
            self.advance();
        }

        result
    }

    fn parse_list(&mut self) -> Result<Expr, ParseError<'input>> {
        eprintln!("{}:{:?}: parsing list", self.current_pos, self.tokens);
        let mut list = List::new();
        if self.at_eof() {
            eprintln!("{}:{:?}: at eof", self.current_pos, self.tokens);
            return Err(ParseError::EOF);
        };

        self.match_token(&Token::LParen)?;

        while let Ok(expr) = self.parse_expr() {
            list.push(expr);
        }

        self.match_token(&Token::RParen)?;
        Ok(Expr::List(list))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError<'input>> {
        match self.parse_atom() {
            Ok(atom) => Ok(Expr::Atom(atom)),
            Err(_) => match self.parse_list() {
                Ok(list) => Ok(list),
                Err(e) => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_atomic() {
        let mut scanner = Scanner::new("1 sdf \"sadf\" ");
        let mut parser = Parser::new(&mut scanner);
        assert_eq!(parser.parse_atom(), Ok(Atom::Number(1.0)));
        assert_eq!(parser.parse_atom(), Ok(Atom::Symbol("sdf".into())));
        assert_eq!(parser.parse_atom(), Ok(Atom::String("sadf".into())));
        assert_eq!(parser.parse_atom(), Err(ParseError::EOF))
    }

    #[test]
    fn test_parser_simple_list() {
        use Atom::*;
        let mut scanner = Scanner::new("(1 sdf \"sadf\" )");
        let mut parser = Parser::new(&mut scanner);
        assert_eq!(
            parser.parse_list(),
            Ok(Expr::List(vec![
                Expr::Atom(Number(1.0)),
                Expr::Atom(Symbol("sdf".to_string())),
                Expr::Atom(String("sadf".to_string()))
            ]))
        );
    }

    #[test]
    fn test_parser_complicated_list() {
        use Atom::*;
        let mut scanner = Scanner::new("(def (add x y) (+ x y))");
        let mut parser = Parser::new(&mut scanner);

        assert_eq!(
            parser.parse_list(),
            Ok(Expr::List(vec![
                Expr::Atom(Symbol("def".into())),
                Expr::List(vec![
                    Expr::Atom(Symbol("add".into())),
                    Expr::Atom(Symbol("x".into())),
                    Expr::Atom(Symbol("y".into()))
                ]),
                Expr::List(vec![
                    Expr::Atom(Symbol("+".into())),
                    Expr::Atom(Symbol("x".into())),
                    Expr::Atom(Symbol("y".into()))
                ]),
            ]))
        );
    }
}
