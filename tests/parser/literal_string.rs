#[test]
fn test_escaped_empty() {
    let (_, res) = SString::parse_caos("").expect("Valid string");
    assert_eq!(res, String::new());
}

#[test]
fn test_escaped_simple() {
    let (_, res) = SString::parse_caos("Hello world!").expect("Valid string");
    assert_eq!(res, "Hello world!");
}

#[test]
fn test_escaped_newline() {
    let (_, res) = SString::parse_caos("Hello\\nworld!").expect("Valid string");
    assert_eq!(res, "Hello\nworld!");
}

#[test]
fn test_escaped_quote() {
    let (_, res) = SString::parse_caos(r#"Hello\"\\world!"#).expect("Valid string");
    assert_eq!(res, r#"Hello"\world!"#);
}

#[test]
fn test_raw_delimited_empty() {
    let (_, res) = SString::parse_caos(r#""""#).expect("Valid variable");
    assert_eq!(res, String::new().into());
}

#[test]
fn test_literal_empty() {
    let (_, res) = SString::parse_caos(r#""""#).expect("Valid variable");
    assert_eq!(res, String::new().into());
}

#[test]
fn test_literal_single_escape() {
    let (_, res) = SString::parse_caos(r#""\"""#).expect("Valid variable");
    assert_eq!(res, "\"".to_owned().into());
}

#[test]
fn test_literal() {
    let (_, res) = SString::parse_caos(r#""Hello world!""#).expect("Valid variable");
    assert_eq!(res, "Hello world!".to_owned().into());
}

#[test]
fn test_end() {
    let (_, res) = SString::parse_caos(r#""Hello " world!""#).expect("Valid variable");
    assert_eq!(res, "Hello ".to_owned().into());
}

#[test]
fn test_escape() {
    let (_, res) = SString::parse_caos(r#""Hello \" world!""#).expect("Valid variable");
    assert_eq!(res, "Hello \" world!".to_owned().into());
}