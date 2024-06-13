use lang::parser::Parser;

const INPUT_FUNCTION_CALL: &str = "function_call()";
const INPUT_FUNCTION_CALL_ERR1: &str = "function_call(";
const INPUT_FUNCTION_DEF: &str = "fn function_call()";
const INPUT_LET: &str = "let x = 32";
const MATH_EXPR_01: &str = "32 + 2";
const MATH_EXPR_02: &str = "32 + 2 * 4";
const MATH_EXPR_03: &str = "32 * 2 + 4";
const MATH_EXPR_04: &str = "(32 + 2) * 4 + (43 * (4 + 5))";

#[test]
fn parse_function_call() {
    //let tokenizer = Parser::new(INPUT_FUNCTION_CALL);
    //let module = tokenizer.parse().unwrap();
    //assert_eq!(module.functions().len(), 1);
}

#[test]
fn parse_function_call_err1() {
    let tokenizer = Parser::new(INPUT_FUNCTION_CALL_ERR1);
    assert!(tokenizer.parse().is_err());
}

#[test]
fn parse_function_def() {
    let tokenizer = Parser::new(INPUT_FUNCTION_DEF);
    let module = tokenizer.parse().unwrap();
    assert_eq!(module.functions().len(), 1);
}

#[test]
fn parse_expression() {
    let parsed_expr = Parser::new(MATH_EXPR_01).parse_expression().unwrap();
    assert_eq!(parsed_expr.to_string(), "(32 + 2)");
}

#[test]
fn parse_expression_precedence_1() {
    let parsed_expr = Parser::new(MATH_EXPR_02).parse_expression().unwrap();
    assert_eq!(parsed_expr.to_string(), "(32 + (2 * 4))");
}

#[test]
fn parse_expression_precedence_2() {
    let parsed_expr = Parser::new(MATH_EXPR_03).parse_expression().unwrap();
    assert_eq!(parsed_expr.to_string(), "((32 * 2) + 4)");
}
