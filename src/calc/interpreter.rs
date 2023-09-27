use std::collections::HashMap;

use crate::calc::lexer::TokenKind;
use crate::calc::parser::{Expression, Atom};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&mut self, expression: Expression, context: &mut HashMap<String, Expression>) -> Expression {
        match expression {
            Expression::Literal(ref value) => match value {
                Atom::Name(variable_name) => {
                    match context.get(variable_name) {
                        Some(name) => name.clone(),
                        None => {
                            // println!("name {} not found", variable_name);
                            Expression::None
                        },
                    }
                }
                _ => expression,
            }
            Expression::Group(expression) => {
                self.evaluate(*expression.clone(), context)
            }
            // Expression::Unary(right, operator) => {
            //     let right_value = self.evaluate(*right.clone(), context);

            //     match operator.kind() {
                    // TokenKind::Not => {
                    //     let result = self.is_truthy(self.literal_value(right_value).unwrap());
                    //     Expression::Literal(Atom::Boolean(!result))
                    // }
                    // TokenKind::Minus => {
                    //     let result = self.literal_value(right_value).unwrap();
                    //     match result {
                    //         Atom::Number(x) => Expression::Literal(Atom::Number(-x)),
                    //         _ => Expression::None,
                    //     }

                    // }
            //         _ => Expression::None
            //     }
            // }
            Expression::Binary(left, right, operator) => {
                let left_expr = self.evaluate(*left.clone(), context);
                let right_expr = self.evaluate(*right.clone(), context);

                if let None = self.literal_value(left_expr.clone()) {
                    return Expression::None;
                }

                if let None = self.literal_value(right_expr.clone()) {
                    return Expression::None;
                }

                let left_value = self.literal_value(left_expr).unwrap();
                let right_value = self.literal_value(right_expr).unwrap();

                match operator.kind() {
                    TokenKind::Plus => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Number(x + y));
                        }

                        Expression::None
                    },
                    TokenKind::Minus => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Number(x - y));
                        }

                        Expression::None
                    },
                    TokenKind::Times => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Number(x * y));
                        }

                        Expression::None
                    },
                    TokenKind::Over => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Number(x / y));
                        }

                        Expression::None
                    },
                    TokenKind::Less => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Boolean(x < y));
                        }

                        Expression::None
                    },
                    TokenKind::LessEqual => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Boolean(x <= y));
                        }

                        Expression::None
                    },
                    TokenKind::Greater => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Boolean(x > y));
                        }

                        Expression::None
                    },
                    TokenKind::GreaterEqual => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Boolean(x >= y));
                        }

                        Expression::None
                    },
                    TokenKind::Equal => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Boolean(x == y));
                        }

                        Expression::None
                    },
                    TokenKind::Mod => {
                        if let Some(_) = left_value.number() {
                            let x = left_value.number().unwrap();
                            let y = right_value.number().unwrap();

                            return Expression::Literal(Atom::Number(x % y));
                        }

                        Expression::None
                    },
                    _ => Expression::None,
                }
            }
            Expression::Logical(left, right, operator) => {
                let left_value = self.evaluate(*left.clone(), context);

                if operator.kind() == TokenKind::Or {
                    if self.is_truthy(self.literal_value(*left.clone()).unwrap()) {
                        return left_value;
                    }
                } else {
                    if !self.is_truthy(self.literal_value(*left.clone()).unwrap()) {
                        return left_value;
                    }
                }

                self.evaluate(*right, context)
            }
            Expression::Variable(name, value) => {
                let expr = self.evaluate(*value, context);
                context.insert(name.clone(), expr.clone());

                // let value = match self.literal_value(expr) {
                //     Some(v) => v,
                //     None => Atom::Boolean(false),
                // };

                Expression::Literal(Atom::Name(name))
            }
            _ => expression,
        }
    }

    pub fn literal_value(&self, expression: Expression) -> Option<Atom> {
        match expression {
            Expression::Literal(value) => Some(value),
            _ => None,
        }
    }

    fn is_truthy(&self, value: Atom) -> bool {
        match value {
            Atom::Boolean(boolean) => boolean,
            _ => true,
        }
    }
}

