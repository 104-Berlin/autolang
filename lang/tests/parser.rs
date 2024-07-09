use lang::{
    parser::{expression::Expr, type_def::TypeID, Parser},
    spanned::Spanned,
};

#[test]
fn test_full_language_parser() {
    let mut parser = Parser::new(include_str!("full_parsing.al"));
    let module = parser.parse_module().unwrap().value;
    let mut fn_iter = module.functions().iter();

    let main_fn = &fn_iter.next().unwrap().value;
    assert_eq!(main_fn.proto.value.name.value, "main");
    assert!(matches!(main_fn.body.value, Expr::Block(_, _)));
    if let Expr::Block(statements, return_value) = &main_fn.body.value {
        assert_eq!(statements.len(), 5);
        assert!(matches!(
            &statements[0].value,
            Expr::Let(Spanned { value, .. }, Spanned { value: TypeID::Int, .. }, _) if &value == &"a"
        ));
        assert!(matches!(
            &statements[1].value,
            Expr::Let(Spanned { value, .. }, Spanned { value: TypeID::Float, .. }, _) if &value == &"b"
        ));
        assert!(matches!(
            &statements[2].value,
            Expr::Let(Spanned { value, .. }, Spanned { value: TypeID::Bool, .. }, _) if &value == &"c"
        ));
        assert!(matches!(
            &statements[3].value,
            Expr::Let(Spanned { value, .. }, Spanned { value: TypeID::Bool, .. }, _) if &value == &"d"
        ));
        assert!(matches!(
            dbg!(&statements[4].value),
            Expr::Let(Spanned { value, .. }, Spanned { value: TypeID::String, .. }, _) if &value == &"e"
        ));

        assert!(matches!(return_value, None));
    }
}
