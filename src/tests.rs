use super::*;

macro_rules! SETUP_READER {
    ($r:ident, $i:expr) => {
        let input = $i;
        let mut $r = Reader::from_text(input);
        $r.parse_all_lines();
    };
}

macro_rules! SETUP_WRITER {
    ($i:expr, $w:ident) => {
        let input = $i;
        let mut reader = Reader::from_text(input);
        reader.parse_all_lines();

        let mut $w = Writer::new();
        $w.process_bookmarks(&reader);
        $w.process_lines(&reader);
    };
}

macro_rules! SETUP_BOOKMARKS {
    ($e:expr, $j:expr, $w:ident) => {
        $w.bookmarks.insert($e, $j);
    };
}

// --- READER TESTS ---

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

// --- WRITER TEST ---

#[test]
fn test_writer_print_one() {
    SETUP_WRITER!("Hello world", writer);

    assert_eq!(writer.output, "P;Hello world");
}

#[test]
fn test_writer_print_two() {
    SETUP_WRITER!(
        r#"Hello world
Ciao mondo"#,
        writer
    );

    assert_eq!(writer.output, "P;Hello world|P;Ciao mondo");
}

#[test]
fn test_writer_print_three() {
    SETUP_WRITER!(
        r#"Hello world
Ciao mondo
Bonjour monde"#,
        writer
    );

    assert_eq!(writer.output, "P;Hello world|P;Ciao mondo|P;Bonjour monde");
}

#[test]
fn test_writer_question_fake_jump_one() {
    SETUP_WRITER!("+ [Hello world] -> example", writer);

    SETUP_BOOKMARKS!(String::from("example"), 1, writer);

    assert_eq!(writer.output, "Q;Hello world;00000");
}

#[test]
fn test_writer_question_fake_jump_two() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample",
        writer
    );

    SETUP_BOOKMARKS!(String::from("example"), 1, writer);
    SETUP_BOOKMARKS!(String::from("sample"), 2, writer);

    assert_eq!(writer.output, "Q;Hello world;00000;Ciao mondo;00000");
}

#[test]
fn test_writer_question_fake_jump_and_print() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample
Bonjour monde",
        writer
    );

    SETUP_BOOKMARKS!(String::from("example"), 1, writer);
    SETUP_BOOKMARKS!(String::from("sample"), 2, writer);

    assert_eq!(
        writer.output,
        "Q;Hello world;00000;Ciao mondo;00000|P;Bonjour monde"
    );
}

#[test]
fn test_writer_question_fake_jump_multiple() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample
Bonjour monde
+ [Hello world] -> example
+ [Ciao mondo] -> sample
",
        writer
    );

    SETUP_BOOKMARKS!(String::from("example"), 1, writer);
    SETUP_BOOKMARKS!(String::from("sample"), 2, writer);

    assert_eq!(
        writer.output,
        "Q;Hello world;00000;Ciao mondo;00000|P;Bonjour monde|Q;Hello world;00000;Ciao mondo;00000"
    );
}

// #[test]
fn test_writer_question_one() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample
=== example
Hello world
=== sample
Ciao mondo
",
        writer
    );

    assert_eq!(writer.bookmarks["example"], 38);
    assert_eq!(writer.bookmarks["sample"], 52);

    assert_eq!(
        writer.output,
        "Q;Hello world;00000;Ciao mondo;00000|P;Hello world|P;Ciao mondo"
    );
}

#[test]
fn test_writer_end_one() {
    SETUP_WRITER!("-> END", writer);

    assert_eq!(writer.output, "E;");
}

#[test]
fn test_writer_end_two() {
    SETUP_WRITER!(
        "Hello world
-> END",
        writer
    );

    assert_eq!(writer.output, "P;Hello world|E;");
}

#[test]
fn test_writer_bookmark_position_zero_one() {
    SETUP_WRITER!("=== hello", writer);

    assert_eq!(writer.bookmarks["hello"], 0);
}

#[test]
fn test_writer_bookmark_position_zero_two() {
    SETUP_WRITER!(
        "=== hello
=== world",
        writer
    );

    assert_eq!(writer.bookmarks["hello"], 0);
    assert_eq!(writer.bookmarks["world"], 0);
}

#[test]
fn test_writer_bookmark_one() {
    SETUP_WRITER!(
        "Hello world
=== hello
Ciao mondo",
        writer
    );

    assert_eq!(writer.output, "P;Hello world|P;Ciao mondo");
    assert_eq!(writer.bookmarks["hello"], 14);
}

#[test]
fn test_writer_bookmark_two() {
    SETUP_WRITER!(
        "Hello world
=== hello
Ciao mondo
=== world
Bonjour monde",
        writer
    );

    assert_eq!(writer.output, "P;Hello world|P;Ciao mondo|P;Bonjour monde");
    assert_eq!(writer.bookmarks["hello"], 14);
    assert_eq!(writer.bookmarks["world"], 27);
}

#[test]
fn functional_test_one() {
    SETUP_WRITER!(
        "Hello there

I'm a VN written in the Ink format

Do you like it?

+ [Yes, I like it!] -> like
+ [No, I do not like it] -> hate

=== like

Thank you!

-> END

=== hate

Oh, I see

-> END",
        writer
    );

    assert_eq!(writer.output, "P;Hello There|P;I'm a VN written in the Ink format|P;Do you like it?|Q;Yes, I like it!;00120;No, I do not like it;00136|P;Thank you!|E;|P;Oh, I see|E;");
}
