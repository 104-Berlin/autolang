use crate::{execution::value::Value, parser::type_def::TypeID};

pub mod print;

macro_rules! impl_system {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, R, $($params: SystemParam),*> System for SystemFunction<($($params,)*), F>
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),* ) -> R,
                    R: Into<Value>,
        {
            fn run(&self, resources: Vec<Value>) -> Value {
                fn call_inner<R: Into<Value>, $($params),*>(
                    f: impl Fn($($params),*) -> R,
                    $($params: $params),*
                ) -> R {
                    f($($params),*)
                }

                let mut iter = resources.into_iter();

                $(
                    let $params = $params::retrieve(&mut iter);
                )*

                call_inner(&self.function, $($params),*).into()
            }
        }
    }
}

macro_rules! impl_into_system {
    (
        $($params:ident),*
    ) => {
        impl<F, R, $($params: SystemParam),*> IntoSystem<($($params,)*)> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),* ) -> R,
                    R: Into<Value>,
        {
            type System = SystemFunction<($($params,)*), Self>;

            fn into_system(self) -> Self::System {
                SystemFunction {
                    function: self,
                    marker: Default::default(),
                }
            }
        }
    }
}

pub trait SystemParam {
    fn retrieve(args: &mut impl Iterator<Item = Value>) -> Self;
}

pub struct SystemFunction<Input, F> {
    function: F,
    marker: std::marker::PhantomData<fn() -> Input>,
}

pub trait System {
    fn run(&self, args: Vec<Value>) -> Value;
}

impl_system!();
impl_system!(T1);
impl_system!(T1, T2);
impl_system!(T1, T2, T3);
impl_system!(T1, T2, T3, T4);
impl_system!(T1, T2, T3, T4, T5);
impl_system!(T1, T2, T3, T4, T5, T6);

pub trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

impl_into_system!();
impl_into_system!(T1);
impl_into_system!(T1, T2);
impl_into_system!(T1, T2, T3);
impl_into_system!(T1, T2, T3, T4);
impl_into_system!(T1, T2, T3, T4, T5);
impl_into_system!(T1, T2, T3, T4, T5, T6);

impl SystemParam for i64 {
    fn retrieve(args: &mut impl Iterator<Item = Value>) -> Self {
        let value = args.next().unwrap();
        match value.type_id {
            TypeID::Int => value.as_int().unwrap(),
            _ => panic!("Expected an integer"),
        }
    }
}

impl SystemParam for i32 {
    fn retrieve(args: &mut impl Iterator<Item = Value>) -> Self {
        let value = args.next().unwrap();
        match value.type_id {
            TypeID::Int => value.as_int().unwrap() as i32,
            _ => panic!("Expected an integer"),
        }
    }
}

impl SystemParam for f64 {
    fn retrieve(args: &mut impl Iterator<Item = Value>) -> Self {
        let value = args.next().unwrap();
        match value.type_id {
            TypeID::Float => value.as_float().unwrap(),
            _ => panic!("Expected a float"),
        }
    }
}

impl SystemParam for f32 {
    fn retrieve(args: &mut impl Iterator<Item = Value>) -> Self {
        let value = args.next().unwrap();
        match value.type_id {
            TypeID::Float => value.as_float().unwrap() as f32,
            _ => panic!("Expected a float"),
        }
    }
}

impl SystemParam for bool {
    fn retrieve(args: &mut impl Iterator<Item = Value>) -> Self {
        let value = args.next().unwrap();
        match value.type_id {
            TypeID::Bool => value.as_bool().unwrap(),
            _ => panic!("Expected a bool"),
        }
    }
}

impl SystemParam for String {
    fn retrieve(args: &mut impl Iterator<Item = Value>) -> Self {
        let value = args.next().unwrap();
        match value.type_id {
            TypeID::String => value.as_string().unwrap().to_string(),
            _ => panic!("Expected a string"),
        }
    }
}
