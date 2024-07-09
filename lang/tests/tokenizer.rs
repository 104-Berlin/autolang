use lang::tokenizer::{identifier::Identifier, literal::Literal, token::Token, Tokenizer};

const INPUT_FUNCTION_CALL: &str = "function_call()";
const INPUT_FUNCTION_CALL_ERR1: &str = "function_call(";
const INPUT_FUNCTION_DEF: &str = "fn function_call()";
const INPUT_LET: &str = "let x = 32";
const MATH_EXPR_01: &str = "32 + 2";
const MATH_EXPR_02: &str = "32 + 2 * 4";
const MATH_EXPR_03: &str = "32 * 2 + 4";
const MATH_EXPR_04: &str = "(32 + 2) * 4 + (43 * (4 + 5))";
const STRING_LITERAL: &str = "\"Hello\\\", World!\"";

#[test]
fn test_tokenizer() {
    let tokens = Tokenizer::new(INPUT_FUNCTION_CALL)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::UserDefined("function_call".to_string())),
            Token::Identifier(Identifier::LParen),
            Token::Identifier(Identifier::RParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_FUNCTION_CALL_ERR1)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::UserDefined("function_call".to_string())),
            Token::Identifier(Identifier::LParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_FUNCTION_DEF)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::Function),
            Token::Identifier(Identifier::UserDefined("function_call".to_string())),
            Token::Identifier(Identifier::LParen),
            Token::Identifier(Identifier::RParen),
        ]
    );

    let tokens = Tokenizer::new(INPUT_LET)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::Let),
            Token::Identifier(Identifier::UserDefined("x".to_string())),
            Token::Identifier(Identifier::Assignment),
            Token::Literal(Literal::NumberInt(32)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_01)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Literal(Literal::NumberInt(32)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(2)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_02)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Literal(Literal::NumberInt(32)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(2)),
            Token::Identifier(Identifier::Star),
            Token::Literal(Literal::NumberInt(4)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_03)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Literal(Literal::NumberInt(32)),
            Token::Identifier(Identifier::Star),
            Token::Literal(Literal::NumberInt(2)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(4)),
        ]
    );

    let tokens = Tokenizer::new(MATH_EXPR_04)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Identifier::LParen),
            Token::Literal(Literal::NumberInt(32)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(2)),
            Token::Identifier(Identifier::RParen),
            Token::Identifier(Identifier::Star),
            Token::Literal(Literal::NumberInt(4)),
            Token::Identifier(Identifier::Plus),
            Token::Identifier(Identifier::LParen),
            Token::Literal(Literal::NumberInt(43)),
            Token::Identifier(Identifier::Star),
            Token::Identifier(Identifier::LParen),
            Token::Literal(Literal::NumberInt(4)),
            Token::Identifier(Identifier::Plus),
            Token::Literal(Literal::NumberInt(5)),
            Token::Identifier(Identifier::RParen),
            Token::Identifier(Identifier::RParen),
        ]
    );
}

#[test]
fn test_string_literal() {
    let tokens = Tokenizer::new(STRING_LITERAL)
        .map(|t| t.value)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![Token::Literal(Literal::String(
            "Hello\", World!".to_string()
        ))]
    );
}

#[test]
fn test_float_literal() {
    let tokens = Tokenizer::new("32.0").map(|t| t.value).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Token::Literal(Literal::NumberFloat(32.0))]);
}

#[test]
fn test_small_tokens() {
    let mut tokens =
        Tokenizer::new("(){}[];,.:<> :: -> = != == + - *  / % ! && || <= >= //asdsf\n ( /* fd*/ )")
            .map(|t| t.value);

    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::LParen)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::RParen)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::LBrace)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::RBrace)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::LBracket)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::RBracket)));
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::Semicolon))
    );
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Comma)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Dot)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Colon)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::LessThan)));
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::GreaterThan))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::DoubleColon))
    );
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Arrow)));
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::Assignment))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::NotEquals))
    );
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Equals)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Plus)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Minus)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Star)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Slash)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::Modulus)));
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::LogicalNot))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::LogicalAnd))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::LogicalOr))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::LessThanOrEqual))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Identifier(Identifier::GreaterThanOrEqual))
    );
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::LParen)));
    assert_eq!(tokens.next(), Some(Token::Identifier(Identifier::RParen)));
}
