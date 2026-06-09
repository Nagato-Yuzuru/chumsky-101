use crate::ast::*;
use chumsky::IterParser;
use chumsky::prelude::{Parser, Simple, choice, extra, just, recursive, text};

pub fn parser<'src>() -> impl Parser<'src, &'src str, Expr, extra::Err<Simple<'src, char>>> + Clone
{
    recursive(|expr| {
        let int = text::int(10).map(|s: &str| Expr::Num(s.parse().unwrap()));
        let atom = int.or(expr.delimited_by(just('('), just(')'))).padded();
        let unary = just('-')
            .padded()
            .repeated()
            .foldr(atom, |_, rhs| Expr::Neg(Box::new(rhs)));
        let op = |c, kind| just(c).padded().to(kind);
        let product = unary.clone().foldl(
            choice((op('*', BinOp::Mul), op('/', BinOp::Div)))
                .then(unary)
                .repeated(),
            |lhs, (kind, rhs)| Expr::Bin(kind, Box::new(lhs), Box::new(rhs)),
        );

        product.clone().foldl(
            choice((op('+', BinOp::Add), op('-', BinOp::Sub)))
                .then(product)
                .repeated(),
            |lhs, (kind, rhs)| Expr::Bin(kind, Box::new(lhs), Box::new(rhs)),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence_makes_mul_bind_tighter() {
        // 1 + 2 * 3  =>  Add(1, Mul(2, 3))   —— 乘法在右子树,先结合
        let ast = parser().parse("1 + 2 * 3").into_result().unwrap();
        assert_eq!(
            ast,
            Expr::Bin(
                BinOp::Add,
                Box::new(Expr::Num(1.0)),
                Box::new(Expr::Bin(
                    BinOp::Mul,
                    Box::new(Expr::Num(2.0)),
                    Box::new(Expr::Num(3.0)),
                )),
            )
        );
    }

    #[test]
    fn subtraction_is_left_associative() {
        // 1 - 2 - 3  =>  Sub(Sub(1,2), 3)
        let ast = parser().parse("1 - 2 - 3").into_result().unwrap();
        assert_eq!(
            ast,
            Expr::Bin(
                BinOp::Sub,
                Box::new(Expr::Bin(
                    BinOp::Sub,
                    Box::new(Expr::Num(1.0)),
                    Box::new(Expr::Num(2.0)),
                )),
                Box::new(Expr::Num(3.0)),
            )
        );
    }
}
