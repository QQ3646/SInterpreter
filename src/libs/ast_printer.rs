use crate::libs::expr::{visitor::Visitor, ast::Expr};

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&mut self, binary: &Expr) -> String {
        if let Expr::Binary { ref left, ref operator, ref right } = binary {
            return self.parenthesize(operator.lexeme.to_string(), vec![left, right]);
        }
        "Something went wrong".to_string()
    }

    fn visit_grouping(&mut self, grouping: &Expr) -> String {
        if let Expr::Grouping { ref expression } = grouping {
            return self.parenthesize("group".to_string(), vec![expression]);
        }
        "Something went wrong".to_string()
    }

    fn visit_literal(&mut self, literal: &Expr) -> String {
        if let Expr::Literal { ref value } = literal {
            return value.to_string();
        }
        "Something went wrong".to_string()
    }

    fn visit_unary(&mut self, unary: &Expr) -> String {
        if let Expr::Unary { ref operator, ref right } = unary {
            return self.parenthesize(operator.lexeme.to_string(), vec![right]);
        }
        "Something went wrong".to_string()
    }
}

impl AstPrinter {
    fn parenthesize(&mut self, name: String, exprs: Vec<&Option<Box<Expr>>>) -> String {
        let mut result = String::new();

        result += "(";
        result.push_str(&name);
        for expr in exprs {
            result.push(' ');
            if let Some(string) = self.visit_expr(expr) {
                result.push_str(&string);
            } else {
                result.push_str("Error");
            }
        }
        result += ")";

        result
    }
}

#[cfg(test)]
mod test {
    use crate::libs::ast_printer::AstPrinter;
    use crate::libs::expr::ast::{Expr, Object};
    use crate::libs::expr::visitor::Visitor;
    use crate::libs::lex::{LiteralValue, Token, TokenType};

    // Ok
    #[test]
    fn test() {
        let some_expr = Some(Box::new(Expr::Binary {
            left: Some(Box::new(Expr::Unary
            {
                operator: Token {
                    token_type: TokenType::MINUS,
                    line: 1,
                    lexeme: "-".to_string(),
                    literal: LiteralValue::Nil,
                },
                right: Some(Box::new(Expr::Literal { value: Object::Number(123f64) })),
            })),
            operator: Token {
                token_type: TokenType::STAR,
                line: 1,
                lexeme: "*".to_string(),
                literal: LiteralValue::Nil,
            },
            right: Some(Box::new(Expr::Grouping {
                expression: Some(Box::new(Expr::Literal { value: Object::Number(45.67) }))
            })),
        }));
        let mut ast_printer = AstPrinter {};
        let string = ast_printer.visit_expr(&some_expr).unwrap();
        assert_eq!(string, "(* (- 123) (group 45.67))")
    }
}