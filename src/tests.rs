use super::*;

macro_rules! SETUP_READER {
    ($r:ident, $i:expr) => {
        let input = $i;
        let mut $r = Reader::from_text(input);
        $r.parse_all_lines();
    };
}

#[test]
fn test_parse_text_line_one() {
    SETUP_READER!(reader, r#"Hello world"#);

    assert_eq!(reader.lines.len(), 1);

    assert_eq!(reader.lines[0].text, "Hello world");
    assert_eq!(reader.lines[0].type_, LineType::Text);
}

#[test]
fn test_parse_text_line_two() {
    SETUP_READER!(
        reader,
        r#"Hello world
Ciao mondo"#
    );

    assert_eq!(reader.lines.len(), 2);

    assert_eq!(reader.lines[0].text, "Hello world");
    assert_eq!(reader.lines[1].text, "Ciao mondo");

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
}

#[test]
fn test_parse_text_line_three() {
    SETUP_READER!(
        reader,
        r#"Hello world
Ciao mondo
Bonjour monde"#
    );

    assert_eq!(reader.lines.len(), 3);

    assert_eq!(reader.lines[0].text, "Hello world");
    assert_eq!(reader.lines[1].text, "Ciao mondo");
    assert_eq!(reader.lines[2].text, "Bonjour monde");

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
    assert_eq!(reader.lines[2].type_, LineType::Text);
}

#[test]
fn test_parse_empty_line_one() {
    SETUP_READER!(reader, r#""#);

    assert_eq!(reader.lines.len(), 0);
}

#[test]
fn test_parse_empty_line_two() {
    SETUP_READER!(
        reader, r#"
"#
    );

    assert_eq!(reader.lines.len(), 0);
}

#[test]
fn test_parse_empty_line_three() {
    SETUP_READER!(
        reader, r#"

"#
    );

    assert_eq!(reader.lines.len(), 0);
}

#[test]
fn test_parse_empty_line_multiple_one() {
    SETUP_READER!(
        reader,
        r#"Hello
"#
    );

    assert_eq!(reader.lines.len(), 1);
    assert_eq!(reader.lines[0].text, "Hello");
    assert_eq!(reader.lines[0].type_, LineType::Text);
}

#[test]
fn test_parse_empty_line_multiple_two() {
    SETUP_READER!(
        reader,
        r#"Hello

World"#
    );

    assert_eq!(reader.lines.len(), 2);

    assert_eq!(reader.lines[0].text, "Hello");
    assert_eq!(reader.lines[1].text, "World");

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
}

#[test]
fn test_parse_question_one() {
    SETUP_READER!(reader, r#"+ [Hello!] -> hello"#);

    assert_eq!(reader.lines.len(), 1);
    assert_eq!(reader.lines[0].type_, LineType::Question);
}

#[test]
fn test_parse_question_two() {
    SETUP_READER!(
        reader,
        r#"+ [Hello!] -> hello
+ [World!] -> world"#
    );

    assert_eq!(reader.lines.len(), 2);

    assert_eq!(reader.lines[0].type_, LineType::Question);
    assert_eq!(reader.lines[1].type_, LineType::Question);
}

#[test]
fn test_parse_bookmark_one() {
    SETUP_READER!(reader, r#"=== hello"#);

    assert_eq!(reader.lines.len(), 1);

    assert_eq!(reader.lines[0].type_, LineType::Bookmark);
}

#[test]
fn test_parse_bookmark_two() {
    SETUP_READER!(
        reader,
        r#"=== hello
=== world"#
    );

    assert_eq!(reader.lines.len(), 2);

    assert_eq!(reader.lines[0].type_, LineType::Bookmark);
    assert_eq!(reader.lines[1].type_, LineType::Bookmark);
}

#[test]
fn test_parse_end_one() {
    SETUP_READER!(reader, r#"-> END"#);

    assert_eq!(reader.lines.len(), 1);

    assert_eq!(reader.lines[0].type_, LineType::End);
}

#[test]
fn test_parse_end_two() {
    SETUP_READER!(
        reader,
        r#"Hello
-> END"#
    );

    assert_eq!(reader.lines.len(), 2);

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::End);
}

// #[test]
// fn test_writer_undefined() {
//     assert_eq!("", "A;");
// }

// #[test]
// fn test_writer_print() {
//     assert_eq!("", "P;Hello There");
// }

// #[test]
// fn test_writer_question_one() {
//     assert_eq!("", "Q;Yes, I like it!;00120");
// }

// #[test]
// fn test_writer_question_two() {
//     assert_eq!("", "Q;Yes, I like it!;00120;No, I do not like it;00136");
// }

// #[test]
// fn test_writer_question_missing_jump() {
//     assert_eq!("", "Q;Yes, I like it!");
// }

// #[test]
// fn test_writer_jump() {
//     assert_eq!("", "J;00001");
// }

// #[test]
// fn test_writer_end() {
//     assert_eq!("", "E;");
// }
