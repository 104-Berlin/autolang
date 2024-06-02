use lang::tokenizer::{Token, Tokenizer};

const INPUT_FUNCTION_CALL: &str = "function_call()";
const _INPUT_FUNCTION_CALL_ERR1: &str = "32function_call()";
const INPUT_FUNCTION_DEF: &str = "fn function_call()";
const INPUT_LET: &str = "let x = 32";
const MATH_EXPR_01: &str = "32 + 2";
const MATH_EXPR_02: &str = "32 + 2 * 4";
const MATH_EXPR_03: &str = "(32 + 2) * 4 + (43 * (4 + 5))";

#[test]
fn tokenize_function_call() {
    let tokenizer = Tokenizer::new(INPUT_FUNCTION_CALL);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("function_call".into()),
            Token::LParen,
            Token::RParen
        ]
    );
}

#[test]
fn tokenize_function_def() {
    let tokenizer = Tokenizer::new(INPUT_FUNCTION_DEF);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Fn,
            Token::Identifier("function_call".into()),
            Token::LParen,
            Token::RParen
        ]
    );
}

#[test]
fn tokenize_let() {
    let tokenizer = Tokenizer::new(INPUT_LET);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Identifier("x".into()),
            Token::Assign,
            Token::NumberInt(32)
        ]
    );
}

#[test]
fn tokenize_math_expr_01() {
    let tokenizer = Tokenizer::new(MATH_EXPR_01);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![Token::NumberInt(32), Token::Plus, Token::NumberInt(2)]
    );
}

#[test]
fn tokenize_math_expr_02() {
    let tokenizer = Tokenizer::new(MATH_EXPR_02);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::NumberInt(32),
            Token::Plus,
            Token::NumberInt(2),
            Token::Star,
            Token::NumberInt(4)
        ]
    );
}

#[test]
fn tokenize_math_expr_03() {
    let tokenizer = Tokenizer::new(MATH_EXPR_03);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::LParen,
            Token::NumberInt(32),
            Token::Plus,
            Token::NumberInt(2),
            Token::RParen,
            Token::Star,
            Token::NumberInt(4),
            Token::Plus,
            Token::LParen,
            Token::NumberInt(43),
            Token::Star,
            Token::LParen,
            Token::NumberInt(4),
            Token::Plus,
            Token::NumberInt(5),
            Token::RParen,
            Token::RParen
        ]
    );
}
