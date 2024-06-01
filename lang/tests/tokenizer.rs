use lang::tokenizer::{Token, Tokenizer};

const INPUT_FUNCTION_CALL: &str = "function_call()";
const INPUT_FUNCTION_CALL_ERR1: &str = "32function_call()";

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
