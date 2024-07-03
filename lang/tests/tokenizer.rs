use lang::tokenizer::{Identifier, Literal, TokenKind, Tokenizer};

const INPUT_FUNCTION_CALL: &str = "function_call()";
const INPUT_FUNCTION_CALL_ERR1: &str = "function_call(";
const INPUT_FUNCTION_DEF: &str = "fn function_call()";
const INPUT_LET: &str = "let x = 32";
const MATH_EXPR_01: &str = "32 + 2";
const MATH_EXPR_02: &str = "32 + 2 * 4";
const MATH_EXPR_03: &str = "32 * 2 + 4";
const MATH_EXPR_04: &str = "(32 + 2) * 4 + (43 * (4 + 5))";

#[test]
fn test_tokenizer() {
    let tokens = Tokenizer::new(INPUT_FUNCTION_CALL)
        .map(|t| t.kind)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            TokenKind::Identifier(Identifier::UserDefined("function_call".to_string())),
            TokenKind::Identifier(Identifier::LParen),
            TokenKind::Identifier(Identifier::RParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_FUNCTION_CALL_ERR1)
        .map(|t| t.kind)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            TokenKind::Identifier(Identifier::UserDefined("function_call".to_string())),
            TokenKind::Identifier(Identifier::LParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_FUNCTION_DEF)
        .map(|t| t.kind)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            TokenKind::Identifier(Identifier::Function),
            TokenKind::Identifier(Identifier::UserDefined("function_call".to_string())),
            TokenKind::Identifier(Identifier::LParen),
            TokenKind::Identifier(Identifier::RParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_LET)
        .map(|t| t.kind)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            TokenKind::Identifier(Identifier::Let),
            TokenKind::Identifier(Identifier::UserDefined("x".to_string())),
            TokenKind::Identifier(Identifier::Assignment),
            TokenKind::Literal(Literal::NumberInt(32)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_01)
        .map(|t| t.kind)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            TokenKind::Literal(Literal::NumberInt(32)),
            TokenKind::Identifier(Identifier::Plus),
            TokenKind::Literal(Literal::NumberInt(2)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_02)
        .map(|t| t.kind)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            TokenKind::Literal(Literal::NumberInt(32)),
            TokenKind::Identifier(Identifier::Plus),
            TokenKind::Literal(Literal::NumberInt(2)),
            TokenKind::Identifier(Identifier::Star),
            TokenKind::Literal(Literal::NumberInt(4)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_03)
        .map(|t| t.kind)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            TokenKind::Literal(Literal::NumberInt(32)),
            TokenKind::Identifier(Identifier::Star),
            TokenKind::Literal(Literal::NumberInt(2)),
            TokenKind::Identifier(Identifier::Plus),
            TokenKind::Literal(Literal::NumberInt(4)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_04)
        .map(|t| t.kind)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            TokenKind::Identifier(Identifier::LParen),
            TokenKind::Literal(Literal::NumberInt(32)),
            TokenKind::Identifier(Identifier::Plus),
            TokenKind::Literal(Literal::NumberInt(2)),
            TokenKind::Identifier(Identifier::RParen),
            TokenKind::Identifier(Identifier::Star),
            TokenKind::Literal(Literal::NumberInt(4)),
            TokenKind::Identifier(Identifier::Plus),
            TokenKind::Identifier(Identifier::LParen),
            TokenKind::Literal(Literal::NumberInt(43)),
            TokenKind::Identifier(Identifier::Star),
            TokenKind::Identifier(Identifier::LParen),
            TokenKind::Literal(Literal::NumberInt(4)),
            TokenKind::Identifier(Identifier::Plus),
            TokenKind::Literal(Literal::NumberInt(5)),
            TokenKind::Identifier(Identifier::RParen),
            TokenKind::Identifier(Identifier::RParen),
        ]
    );
}
