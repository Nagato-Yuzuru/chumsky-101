use crate::ast::*;

pub fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Num(f) => *f,
        Expr::Bin(op, lhs, rhs) => match op {
            BinOp::Add => eval(lhs) + eval(rhs),
            BinOp::Sub => eval(lhs) - eval(rhs),
            BinOp::Mul => eval(lhs) * eval(rhs),
            BinOp::Div => eval(lhs) / eval(rhs),
            BinOp::Mod => eval(lhs) % eval(rhs),
        },
        Expr::Neg(s) => -eval(s),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_eval() {
        let expr = Expr::Bin(
            BinOp::Add,
            Box::new(Expr::Num(2.0)),
            Box::new(Expr::Bin(
                BinOp::Mul,
                Box::new(Expr::Num(3.0)),
                Box::new(Expr::Num(3.0)),
            )),
        );
        assert_eq!(eval(&expr), 11.0);
    }
}
