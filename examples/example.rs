extern crate shell2batch;

fn main() {
    let script = shell2batch::convert(
        r#"
        set -x

        export FILE1=file1
        export FILE2=file2

        #this is some test code
        cp ${FILE1} $FILE2
        cp -r ${DIR1} $DIR2

        #another
        mv file2 file3

        export MY_DIR=directory

        #flags are supported
        rm -Rf ${MY_DIR}

        unset MY_DIR

        touch ./file3

        #provide custom windows command for specific shell command
        complex_bash_command --flag1 value2 # shell2batch: complex_windows_command /flag10 windows_value
        "#,
    );

    assert_eq!(
        script,
        r#"
@echo on

set FILE1=file1
set FILE2=file2

@REM this is some test code
copy %FILE1% %FILE2%
xcopy /E %DIR1% %DIR2%

@REM another
move file2 file3

set MY_DIR=directory

@REM flags are supported
rmdir /S /Q %MY_DIR%

set MY_DIR=

copy /B .\file3+,, .\file3

@REM provide custom windows command for specific shell command
complex_windows_command /flag10 windows_value
"#
    );

    println!("Script: {}", script);
}
