use crate::ast::{Expr, Visitor};
use crate::token::{LiteralType, Token};

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.paranthesize(&operator.lexeme, vec![left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
        self.paranthesize("group", vec![expression])
    }

    fn visit_literal_expr(&mut self, value: &LiteralType) -> String {
        match value {
            LiteralType::STRING(value) => value.clone(),
            LiteralType::NUMBER(value) => format!("{}", value),
            LiteralType::NULL => String::from("null"),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
        self.paranthesize(&operator.lexeme, vec![right])
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        return expr.accept(self);
    }

    fn paranthesize(&mut self, name: &str, expressions: Vec<&Expr>) -> String {
        let mut result = String::from("");

        result.push('(');
        result.push_str(name);

        for expr in expressions {
            result.push(' ');
            result.push_str(&expr.accept(self))
        }

        result.push(')');
        result
    }
}
