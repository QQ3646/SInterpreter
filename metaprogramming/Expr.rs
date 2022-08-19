//That file created by "metaprogramming" package.

mod ast {
    enum Object {
        Number(f64),
        Str(String),
    }

    enum Expr {
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
mod visitor {
    use super::ast::*;

    pub trait Visitor<T> {
        fn visit_expr(&mut self, expr: &Expr) -> T;
    }
}

