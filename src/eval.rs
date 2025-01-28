pub struct Evaluator;
use crate::parser::{Atom, Expr};

fn is_operator(expr: &Expr) -> bool {
    if let Expr::Atom(Atom::Symbol(y)) = expr {
        y == "+" || y == "-" || y == "*" || y == "/"
    } else {
        false
    }
}

impl Evaluator {
    pub fn eval(&self, expr: &Expr) -> Atom {
        match expr {
            Expr::Atom(Atom::Symbol(x)) => Atom::Symbol(x.to_owned()),
            Expr::Atom(Atom::Number(x)) => Atom::Number(*x),
            Expr::Atom(Atom::String(x)) => Atom::String(x.to_owned()),
            Expr::List(list) => {
                if list.len() <= 1 {
                    unimplemented!("later")
                };
                match &list[0] {
                    Expr::List(_) => unimplemented!("later"),
                    atom if is_operator(atom) => {
                        let Expr::Atom(Atom::Symbol(op)) = atom else {
                            unreachable!("")
                        };
                        match op.as_str() {
                            "+" => Atom::Number(list.iter().skip(1).map(|e| self.eval(e)).fold(
                                0.0,
                                |result, expr| {
                                    if let Atom::Number(n) = expr {
                                        result + n
                                    } else {
                                        result
                                    }
                                },
                            )),

                            "-" => Atom::Number(list.iter().skip(1).map(|e| self.eval(e)).fold(
                                0.0,
                                |result, expr| {
                                    if let Atom::Number(n) = expr {
                                        result - n
                                    } else {
                                        result
                                    }
                                },
                            )),

                            _ => todo!("later"),
                        }
                    }
                    _ => todo!("later"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let x = Evaluator;
        let result = x.eval(&Expr::List(vec![
            Expr::Atom(Atom::Symbol("+".to_owned())),
            Expr::Atom(Atom::Number(1.0)),
        ]));
        assert_eq!(result, Atom::Number(1.0));
    }

    #[test]
    fn test_add() {
        let x = Evaluator;
        let result = x.eval(&Expr::List(vec![
            Expr::Atom(Atom::Symbol("+".to_owned())),
            Expr::Atom(Atom::Number(1.0)),
            Expr::Atom(Atom::Number(2.0)),
        ]));
        assert_eq!(result, Atom::Number(3.0));
    }
}
