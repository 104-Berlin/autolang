use std::fmt::Display;

use nom::{
    branch::alt,
    combinator::{cut, map},
    multi::many0,
    sequence::{delimited, tuple},
    Parser,
};

use crate::{
    error::Error,
    tokenizer::{self, Literal},
};

use super::{
    binary_expression::{BinaryExpression, BinaryOperator}, helpers::ws, parse_function_call, spans::{unite_spans, with_span, InputSpan, NomResult, Spanned}
};

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr<'a> {
    FunctionCall(String),
    Binary(BinaryExpression<'a>),

    Literal(Literal),
    Variable(String),
}

pub type SpannedExpr<'a> = Spanned<'a, Expr<'a>>;

pub(super) fn parse_expression(input: InputSpan) -> NomResult<'_, Spanned<Expr>> {
    alt((parse_binary_expr, parse_primary_expr))(input)
}

pub fn parse_primary_expr(input: InputSpan) -> NomResult<'_, Spanned<Expr>> {
    alt((
        with_span(map(tokenizer::numbers, |l| Expr::Literal(l))),
        with_span(map(tokenizer::identifier, |ident| {
            Expr::Variable(ident.fragment().to_string())
        })),
        parse_function_call,
    ))(input)
}

pub fn parse_binary_expr<'a>(input: InputSpan) -> NomResult<'_, Spanned<Expr>> {
    let mut binary_parser = tuple((
        parse_primary_expr,
        many0(tuple((delimited(ws, tokenizer::binary_operator, ws), cut(parse_primary_expr)))),
    ));

    let (remaining_input, (first, rest)) = binary_parser.parse(input)?;
    let folded = fold_binary_expr(input, first, rest).map_err(nom::Err::Failure)?;
    Ok((remaining_input, folded))
}

fn fold_binary_expr<'a>(
    input: InputSpan<'a>,
    first: SpannedExpr<'a>,
    rest: Vec<(Spanned<'a, BinaryOperator>, SpannedExpr<'a>)>,
) -> Result<SpannedExpr<'a>, Error> {
    let mut right_contour: Vec<BinaryOperator> = vec![];

    rest.into_iter()
        .try_fold(first, |mut acc, (next_op, expr)| {
            let united_span = unite_spans(input, &acc, &expr);

            let insert_pos = right_contour
                .iter()
                .position(|past_op| past_op.precedence() >= next_op.extra.precedence())
                .unwrap_or(right_contour.len());

            right_contour.truncate(insert_pos);
            right_contour.push(next_op.extra.clone());

            if insert_pos == 0 {
                Ok(united_span.copy_with_extra(Expr::Binary(BinaryExpression {
                    lhs: Box::new(acc),
                    op: next_op,
                    rhs: Box::new(expr),
                })))
            } else {
                let mut parent = &mut acc;
                for _ in 1..insert_pos {
                    parent = match &mut parent.extra {
                        Expr::Binary(BinaryExpression { rhs, .. }) => rhs,
                        _ => unreachable!(),
                    }
                }

                *parent = unite_spans(input, parent, &expr).copy_with_extra(parent.extra.clone());
                if let Expr::Binary(BinaryExpression { rhs, .. }) = &mut parent.extra {
                    let rhs_span = unite_spans(input, rhs, &expr);

                    let old_rhs = Box::new(rhs.copy_with_extra(Expr::Variable("".into())));
                    let old_rhs = std::mem::replace(rhs, old_rhs);

                    let new_expr = Expr::Binary(BinaryExpression {
                        lhs: old_rhs,
                        op: next_op,
                        rhs: Box::new(expr),
                    });
                    *rhs = Box::new(rhs_span.copy_with_extra(new_expr));
                }

                Ok(united_span.copy_with_extra(acc.extra))
            }
        })
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FunctionCall(name) => write!(f, "{}()", name),
            Expr::Binary(expr) => {
                write!(
                    f,
                    "({} {} {})",
                    expr.lhs.extra, expr.op.extra, expr.rhs.extra
                )
            }
            Expr::Literal(literal) => write!(
                f,
                "{}",
                match literal {
                    Literal::NumberFloat(val) => val.to_string(),
                    Literal::NumberInt(val) => val.to_string(),
                }
            ),
            Expr::Variable(name) => write!(f, "{}", name),
        }
    }
}
