extern crate shell2batch;

fn main() {
    let script = shell2batch::convert(
        r#"
        #this is some test code
        cp file1 file2

        #another
        mv file2 file3

        #flags are supported
        rm -Rf ./directory
        "#
    );

    assert_eq!(
        script,
        r#"
@REM this is some test code
xcopy file1 file2

@REM another
move file2 file3

@REM flags are supported
del /Q ./directory
"#
    );

    println!("Script: {}", script);
}
