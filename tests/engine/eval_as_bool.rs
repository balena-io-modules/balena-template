use balena_template::engine::context::Context;
use balena_template::engine::Engine;

macro_rules! test_eval_eq {
    ($e:expr, $r:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert_eq!(engine.eval_as_bool(&$e.parse().unwrap(), &context).unwrap(), $r);
    }};
}

macro_rules! test_eval_err {
    ($e:expr) => {{
        let engine = Engine::default();
        let context = Context::default();
        assert!(engine.eval_as_bool(&$e.parse().unwrap(), &context).is_err());
    }};
}

#[test]
fn test_boolean() {
    test_eval_eq!("true", true);
    test_eval_eq!("false", false);
}

#[test]
fn test_string() {
    test_eval_err!("\"\"");
    test_eval_err!("\"hallo\"");
}

#[test]
fn test_integer() {
    test_eval_err!("10");
    test_eval_err!("-12");
    test_eval_err!("0");
}

#[test]
fn test_float() {
    test_eval_err!("10.2");
    test_eval_err!("-3.2");
    test_eval_err!("0.0");
}

#[test]
fn test_logical_and() {
    test_eval_eq!("true and true", true);
    test_eval_eq!("true and false", false);
    test_eval_eq!("false and true", false);
    test_eval_eq!("false and false", false);
}

#[test]
fn test_logical_or() {
    test_eval_eq!("true or true", true);
    test_eval_eq!("true or false", true);
    test_eval_eq!("false or true", true);
    test_eval_eq!("false or false", false);
}

#[test]
fn test_logical_not() {
    test_eval_eq!("not false", true);
    test_eval_eq!("not 1 == 2", true);
}

#[test]
fn test_logical_equal() {
    test_eval_eq!("true == true", true);
    test_eval_eq!("1 == 1", true);
    test_eval_eq!("2.3 == 2.3", true);
    test_eval_eq!("`a` == `a`", true);
    test_eval_eq!("`a` == `b`", false);
    test_eval_eq!("`1` == 1", false);
}

#[test]
fn test_logical_not_equal() {
    test_eval_eq!("true != true", false);
    test_eval_eq!("1 != 1", false);
    test_eval_eq!("2.3 != 2.3", false);
    test_eval_eq!("`a` != `a`", false);
    test_eval_eq!("`a` != `b`", true);
    test_eval_eq!("`1` != 1", true);
}

#[test]
fn test_relational_greater_than() {
    test_eval_eq!("1 > 2", false);
    test_eval_eq!("3 > 2", true);
    test_eval_eq!("3.1 > 2", true);
}

#[test]
fn test_relational_greater_than_or_equal() {
    test_eval_eq!("1 >= 2", false);
    test_eval_eq!("3 >= 2", true);
    test_eval_eq!("3.1 >= 2", true);
    test_eval_eq!("3.1 >= 3.1", true);
    test_eval_eq!("3 >= 3.0", true);
}

#[test]
fn test_relational_lower_than() {
    test_eval_eq!("1 < 2", true);
    test_eval_eq!("3 < 2", false);
    test_eval_eq!("3.1 < 2", false);
    test_eval_eq!("3.1 < 3.1", false);
    test_eval_eq!("3 < 3.0", false);
}

#[test]
fn test_relational_lower_than_or_equal() {
    test_eval_eq!("1 <= 2", true);
    test_eval_eq!("3 <= 2", false);
    test_eval_eq!("2 <= 3.1", true);
    test_eval_eq!("3.1 <= 3.1", true);
    test_eval_eq!("3 <= 3.0", true);
}

#[test]
fn test_math() {
    test_eval_err!("1 - 1");
    test_eval_err!("2 - 1");
}

#[test]
fn test_function() {
    test_eval_err!("uuidv4()");
}
