use crate::ast::Visitor;
use crate::token::{LiteralType, Token};

struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T {}

    fn visit_grouping_expr(&mut self, expression: &Expr) -> T {}

    fn visit_literal_expr(&mut self, value: &LiteralType) -> T {}

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T {}
}
