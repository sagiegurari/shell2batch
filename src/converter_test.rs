use super::*;

#[test]
fn add_arguments_empty_additional() {
    let value = add_arguments("test", vec![]);

    assert_eq!(value, "test".to_string());
}

#[test]
fn add_arguments_all_empty() {
    let value = add_arguments("test", vec![]);

    assert_eq!(value, "test".to_string());
}

#[test]
fn add_arguments_additional_values() {
    let value = add_arguments("test", vec![" 1", " 2", " 3"]);

    assert_eq!(value, "test 1 2 3".to_string());
}

#[test]
fn add_arguments_empty_args_and_additional_values() {
    let value = add_arguments("", vec!["1", " 2", " 3"]);

    assert_eq!(value, "1 2 3".to_string());
}

#[test]
fn replace_flags_all_empty() {
    let value = replace_flags("", vec![]);

    assert_eq!(value, "".to_string());
}

#[test]
fn replace_flags_args_empty_replacment_existing() {
    let value = replace_flags("", vec![("linux", "windows")]);

    assert_eq!(value, "".to_string());
}

#[test]
fn replace_flags_args_existing_replacment_empty() {
    let value = replace_flags("linux", vec![]);

    assert_eq!(value, "linux".to_string());
}

#[test]
fn replace_flags_multiple() {
    let value = replace_flags(
        "linux1 LiNux2 somethingelse",
        vec![
            ("linux1", "windows1"),
            ("[lL]i[nN]ux[1-9]", "windowsX"),
            ("unknown", "bad"),
        ],
    );

    assert_eq!(value, "windows1 windowsX somethingelse".to_string());
}

#[test]
fn replace_full_vars_empty() {
    let value = replace_full_vars("");

    assert_eq!(value, "".to_string());
}

#[test]
fn replace_full_vars_not_found() {
    let value = replace_full_vars("test 123");

    assert_eq!(value, "test 123".to_string());
}

#[test]
fn replace_full_vars_found() {
    let mut value = replace_full_vars("test ${myvar} 123");
    assert_eq!(value, "test %myvar% 123".to_string());

    value = replace_full_vars("test ${myvar}");
    assert_eq!(value, "test %myvar%".to_string());

    value = replace_full_vars("test ${myvar} ${myvar2} somethingelse ${myvar3}");
    assert_eq!(
        value,
        "test %myvar% %myvar2% somethingelse %myvar3%".to_string()
    );
}

#[test]
fn replace_partial_vars_empty() {
    let value = replace_partial_vars("");

    assert_eq!(value, "".to_string());
}

#[test]
fn replace_partial_vars_not_found() {
    let value = replace_partial_vars("test 123");

    assert_eq!(value, "test 123".to_string());
}

#[test]
fn replace_partial_vars_found() {
    let mut value = replace_partial_vars("test $myvar 123");
    assert_eq!(value, "test %myvar% 123".to_string());

    value = replace_partial_vars("test $myvar");
    assert_eq!(value, "test %myvar%".to_string());

    value = replace_partial_vars("test $myvar $myvar2 somethingelse $myvar3");
    assert_eq!(
        value,
        "test %myvar% %myvar2% somethingelse %myvar3%".to_string()
    );
}

#[test]
fn replace_vars_empty() {
    let value = replace_vars("");

    assert_eq!(value, "".to_string());
}

#[test]
fn replace_vars_not_found() {
    let value = replace_vars("test 123");

    assert_eq!(value, "test 123".to_string());
}

#[test]
fn replace_vars_full_syntax() {
    let mut value = replace_vars("test ${myvar} 123");
    assert_eq!(value, "test %myvar% 123".to_string());

    value = replace_vars("test ${myvar}");
    assert_eq!(value, "test %myvar%".to_string());

    value = replace_vars("test ${myvar} ${myvar2} somethingelse ${myvar3}");
    assert_eq!(
        value,
        "test %myvar% %myvar2% somethingelse %myvar3%".to_string()
    );
}

#[test]
fn replace_vars_partial_syntax() {
    let mut value = replace_vars("test $myvar 123");
    assert_eq!(value, "test %myvar% 123".to_string());

    value = replace_vars("test $myvar");
    assert_eq!(value, "test %myvar%".to_string());

    value = replace_vars("test $myvar $myvar2 somethingelse $myvar3");
    assert_eq!(
        value,
        "test %myvar% %myvar2% somethingelse %myvar3%".to_string()
    );
}

#[test]
fn replace_vars_mixed() {
    let mut value = replace_vars("test $myvar ${myvar2} 123");
    assert_eq!(value, "test %myvar% %myvar2% 123".to_string());

    value = replace_vars("${somevar1} test $myvar");
    assert_eq!(value, "%somevar1% test %myvar%".to_string());

    value = replace_vars("test $myvar ${myvar2} somethingelse $myvar3");
    assert_eq!(
        value,
        "test %myvar% %myvar2% somethingelse %myvar3%".to_string()
    );
}

#[test]
fn replace_params_full() {
    let mut value = replace_full_vars("echo 0=${0} 1=${1} 2=${2} 3=${3} 4=${4} 5=${5} 6=${6} 7=${7} 8=${8} 9=${9}");
    assert_eq!(value, "echo 0=%0 1=%1 2=%2 3=%3 4=%4 5=%5 6=%6 7=%7 8=%8 9=%9".to_string());

    value = replace_full_vars("echo ${@}");
    assert_eq!(value, "echo %*".to_string());
}

#[test]
fn replace_params_partial_syntax() {
    let mut value = replace_partial_vars("echo 0=$0 1=$1 2=$2 3=$3 4=$4 5=$5 6=$6 7=$7 8=$8 9=$9");
    assert_eq!(value, "echo 0=%0 1=%1 2=%2 3=%3 4=%4 5=%5 6=%6 7=%7 8=%8 9=%9".to_string());

    value = replace_partial_vars("echo $@");
    assert_eq!(value, "echo %*".to_string());
}

#[test]
fn replace_params_mixed() {
    let mut value = replace_vars("echo 0=$0 1=${1} 2=$2 3=${3} 4=$4 5=${5} 6=$6 7=${7} 8=$8 9=${9}");
    assert_eq!(value, "echo 0=%0 1=%1 2=%2 3=%3 4=%4 5=%5 6=%6 7=%7 8=%8 9=%9".to_string());

    value = replace_vars("echo $@ ${@}");
    assert_eq!(value, "echo %* %*".to_string());
}

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
    let output = run(r#"

        #this is some test code
        cp file1 file2

        #another
        mv file2 file3
        "#);

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
    let output = convert_line("newcommand path/arg1 path/arg2");

    assert_eq!(output, "newcommand path/arg1 path/arg2".to_string());
}

#[test]
fn convert_line_comment() {
    let output = convert_line("#test/test");

    assert_eq!(output, "@REM test/test".to_string());
}

#[test]
fn convert_line_cp() {
    let output = convert_line("cp dir/file1 dir/file2");

    assert_eq!(output, "xcopy dir\\file1 dir\\file2".to_string());
}

#[test]
fn convert_line_cp_recursive() {
    let output = convert_line("cp -r directory/sub1 director/sub2");

    assert_eq!(
        output,
        "xcopy /E directory\\sub1 director\\sub2".to_string()
    );
}

#[test]
fn convert_line_mv() {
    let output = convert_line("mv dir/file1 dir/file2");

    assert_eq!(output, "move dir\\file1 dir\\file2".to_string());
}

#[test]
fn convert_line_ls() {
    let output = convert_line("ls");

    assert_eq!(output, "dir".to_string());
}

#[test]
fn convert_line_rm() {
    let output = convert_line("rm dir/file");

    assert_eq!(output, "del dir\\file".to_string());
}

#[test]
fn convert_line_rm_no_prompt() {
    let output = convert_line("rm -f dir/file");

    assert_eq!(output, "del /Q dir\\file".to_string());
}

#[test]
fn convert_line_rm_recursive() {
    let output = convert_line("rm -r dir/file");

    assert_eq!(output, "rmdir /S dir\\file".to_string());
}

#[test]
fn convert_line_rm_no_prompt_and_recursive_v1() {
    let output = convert_line("rm -rf dir/file");

    assert_eq!(output, "rmdir /S /Q dir\\file".to_string());
}

#[test]
fn convert_line_rm_no_prompt_and_recursive_v2() {
    let output = convert_line("rm -fr dir/file");

    assert_eq!(output, "rmdir /S /Q dir\\file".to_string());
}

#[test]
fn convert_line_rm_no_prompt_and_recursive_v3() {
    let output = convert_line("rm -Rf dir/file");

    assert_eq!(output, "rmdir /S /Q dir\\file".to_string());
}

#[test]
fn convert_line_rm_no_prompt_and_recursive_v4() {
    let output = convert_line("rm -fR dir/file");

    assert_eq!(output, "rmdir /S /Q dir\\file".to_string());
}

#[test]
fn convert_line_mkdir() {
    let output = convert_line("mkdir dir1/dir2");

    assert_eq!(output, "mkdir dir1\\dir2".to_string());
}

#[test]
fn convert_line_mkdir_and_parents() {
    let output = convert_line("mkdir -p dir1/dir2");

    assert_eq!(output, "mkdir  dir1\\dir2".to_string());
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

#[test]
fn convert_line_export() {
    let output = convert_line("export A=B");

    assert_eq!(output, "set A=B".to_string());
}

#[test]
fn convert_line_unset() {
    let output = convert_line("unset A");

    assert_eq!(output, "set A=".to_string());
}
