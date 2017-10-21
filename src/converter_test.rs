use super::*;

#[test]
fn run_empty() {
    let output = run("");

    assert_eq!(output, "".to_string());
}

#[test]
fn run_comment() {
    let output = run("#comment");

    assert_eq!(output, "@REM comment".to_string());
}

#[test]
fn run_command() {
    let output = run("cp file1 file2");

    assert_eq!(output, "xcopy file1 file2".to_string());
}

#[test]
fn run_multi_line() {
    let output = run(
        r#"

        #this is some test code
        cp file1 file2

        #another
        mv file2 file3
        "#
    );

    assert_eq!(
        output,
        r#"

@REM this is some test code
xcopy file1 file2

@REM another
move file2 file3
"#
    );
}

#[test]
fn convert_line_empty() {
    let output = convert_line("");

    assert_eq!(output, "".to_string());
}

#[test]
fn convert_line_unhandled() {
    let output = convert_line("newcommand arg1 arg2");

    assert_eq!(output, "newcommand arg1 arg2".to_string());
}

#[test]
fn convert_line_comment() {
    let output = convert_line("#test");

    assert_eq!(output, "@REM test".to_string());
}

#[test]
fn convert_line_cp() {
    let output = convert_line("cp file1 file2");

    assert_eq!(output, "xcopy file1 file2".to_string());
}

#[test]
fn convert_line_mv() {
    let output = convert_line("mv file1 file2");

    assert_eq!(output, "move file1 file2".to_string());
}

#[test]
fn convert_line_ls() {
    let output = convert_line("ls");

    assert_eq!(output, "dir".to_string());
}

#[test]
fn convert_line_rm() {
    let output = convert_line("rm file");

    assert_eq!(output, "del file".to_string());
}

#[test]
fn convert_line_clear() {
    let output = convert_line("clear");

    assert_eq!(output, "cls".to_string());
}

#[test]
fn convert_line_grep() {
    let output = convert_line("grep");

    assert_eq!(output, "find".to_string());
}

#[test]
fn convert_line_pwd() {
    let output = convert_line("pwd");

    assert_eq!(output, "chdir".to_string());
}
