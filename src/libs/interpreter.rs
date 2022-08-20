use crate::libs::expr::ast::{Expr, Object};
use crate::libs::expr::visitor::Visitor;

struct Interpreter {

}

impl Visitor<Object> for Interpreter {
    fn visit_binary(&mut self, binary: &Expr) -> Object {
        todo!()
    }

    fn visit_grouping(&mut self, grouping: &Expr) -> Object {
        todo!()
    }

    fn visit_literal(&mut self, literal: &Expr) -> Object {
        todo!()
    }

    fn visit_unary(&mut self, unary: &Expr) -> Object {
        todo!()
    }
}