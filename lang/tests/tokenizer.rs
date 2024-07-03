use lang::tokenizer::{Identifier, Literal, Token, Tokenizer};

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
    let tokens = Tokenizer::new(INPUT_FUNCTION_CALL).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::UserDefined("function_call".to_string())),
            Token::Identifier(Identifier::LParen),
            Token::Identifier(Identifier::RParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_FUNCTION_CALL_ERR1).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::UserDefined("function_call".to_string())),
            Token::Identifier(Identifier::LParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_FUNCTION_DEF).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::Function),
            Token::Identifier(Identifier::UserDefined("function_call".to_string())),
            Token::Identifier(Identifier::LParen),
            Token::Identifier(Identifier::RParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_LET).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::Let),
            Token::Identifier(Identifier::UserDefined("x".to_string())),
            Token::Identifier(Identifier::Assignment),
            Token::Literal(Literal::NumberInt(32)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_01).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Literal(Literal::NumberInt(32)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(2)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_02).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Literal(Literal::NumberInt(32)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(2)),
            Token::Identifier(Identifier::Multiply),
            Token::Literal(Literal::NumberInt(4)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_03).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Literal(Literal::NumberInt(32)),
            Token::Identifier(Identifier::Multiply),
            Token::Literal(Literal::NumberInt(2)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(4)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_04).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::LParen),
            Token::Literal(Literal::NumberInt(32)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(2)),
            Token::Identifier(Identifier::RParen),
            Token::Identifier(Identifier::Multiply),
            Token::Literal(Literal::NumberInt(4)),
            Token::Identifier(Identifier::Plus),
            Token::Identifier(Identifier::LParen),
            Token::Literal(Literal::NumberInt(43)),
            Token::Identifier(Identifier::Multiply),
            Token::Identifier(Identifier::LParen),
            Token::Literal(Literal::NumberInt(4)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(5)),
            Token::Identifier(Identifier::RParen),
            Token::Identifier(Identifier::RParen),
        ]
    );
}
