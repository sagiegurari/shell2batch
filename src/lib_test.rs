use super::*;

#[test]
fn convert_empty() {
    let output = convert("");

    assert_eq!(output, "".to_string());
}

#[test]
fn convert_comment() {
    let output = convert("#comment");

    assert_eq!(output, "@REM comment".to_string());
}

#[test]
fn convert_command() {
    let output = convert("cp file1 file2");

    assert_eq!(output, "xcopy file1 file2".to_string());
}

#[test]
fn convert_multi_line() {
    let output = convert(
        r#"
        #this is some test code
        cp file1 file2

        #another
        mv file2 file3

        #flags are supported
        rm -Rf ./directory
        "#,
    );

    assert_eq!(
        output,
        r#"
@REM this is some test code
xcopy file1 file2

@REM another
move file2 file3

@REM flags are supported
rmdir /S /Q ./directory
"#
    );
}
