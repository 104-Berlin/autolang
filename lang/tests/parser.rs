use lang::parser::Parser;

const INPUT_FUNCTION_CALL: &str = "function_call()";
const INPUT_FUNCTION_CALL_ERR1: &str = "function_call(";
const INPUT_FUNCTION_DEF: &str = "fn function_call()";
const INPUT_LET: &str = "let x = 32";
const MATH_EXPR_01: &str = "32 + 2";
const MATH_EXPR_02: &str = "32 + 2 * 4";
const MATH_EXPR_03: &str = "(32 + 2) * 4 + (43 * (4 + 5))";

#[test]
fn parse_function_call() {
    let tokenizer = Parser::new(INPUT_FUNCTION_CALL);
    let expr = tokenizer.parse().unwrap();
    assert!(matches!(expr, lang::parser::Expr::FunctionCall(_)));
}

#[test]
fn parse_function_call_err1() {
    let tokenizer = Parser::new(INPUT_FUNCTION_CALL_ERR1);
    assert!(tokenizer.parse().is_err());
}

#[test]
fn parse_function_def() {
    let tokenizer = Parser::new(INPUT_FUNCTION_DEF);
    let expr = tokenizer.parse().unwrap();
    assert!(matches!(expr, lang::parser::Expr::FunctionDef(_)));
}
