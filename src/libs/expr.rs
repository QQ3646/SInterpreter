//That file mostly created by "metaprogramming" package.

pub mod ast {
    use crate::libs::lex::Token;

    pub enum Object {
        Number(f64),
        Str(String),

        Nil,
    }

    impl ToString for Object {
        fn to_string(&self) -> String {
            match self {
                Object::Number(n) => n.to_string(),
                Object::Str(s) => s.clone(),
                Object::Nil => "nil".to_string(),
            }
        }
    }

    pub enum Expr {
        Binary {
            left: Option<Box<Expr>>,
            operator: Token,
            right: Option<Box<Expr>>,
        },
        Grouping {
            expression: Option<Box<Expr>>,
        },
        Literal {
            value: Object,
        },
        Unary {
            operator: Token,
            right: Option<Box<Expr>>,
        },
    }
}

pub mod visitor {
    use super::ast::*;

    pub trait Visitor<T> {
        fn visit_expr(&mut self, expr: &Option<Box<Expr>>) -> Option<T, > {
            if let Some(box_) = expr {
                match **box_ {
                    Expr::Binary { .. } => Some(self.visit_binary(&box_)),
                    Expr::Grouping { .. } => Some(self.visit_grouping(&box_)),
                    Expr::Literal { .. } => Some(self.visit_literal(&box_)),
                    Expr::Unary { .. } => Some(self.visit_unary(&box_)),
                }
            } else {
                None
            }
        }

        fn visit_binary(&mut self, binary: &Expr) -> T;

        fn visit_grouping(&mut self, grouping: &Expr) -> T;

        fn visit_literal(&mut self, literal: &Expr) -> T;

        fn visit_unary(&mut self, unary: &Expr) -> T;
    }
}

