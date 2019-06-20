use super::*;

#[test]
fn add_arguments_empty_additional() {
    let value = add_arguments("test", vec![]);

    assert_eq!(value, "test");
}

#[test]
fn add_arguments_all_empty() {
    let value = add_arguments("test", vec![]);

    assert_eq!(value, "test");
}

#[test]
fn add_arguments_additional_values() {
    let value = add_arguments("test", vec![" 1", " 2", " 3"]);

    assert_eq!(value, "test 1 2 3");
}

#[test]
fn add_arguments_empty_args_and_additional_values() {
    let value = add_arguments("", vec!["1", " 2", " 3"]);

    assert_eq!(value, "1 2 3");
}

#[test]
fn replace_flags_all_empty() {
    let value = replace_flags("", vec![]);

    assert_eq!(value, "");
}

#[test]
fn replace_flags_args_empty_replacment_existing() {
    let value = replace_flags("", vec![("linux", "windows")]);

    assert_eq!(value, "");
}

#[test]
fn replace_flags_args_existing_replacment_empty() {
    let value = replace_flags("linux", vec![]);

    assert_eq!(value, "linux");
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

    assert_eq!(value, "windows1 windowsX somethingelse");
}

#[test]
fn replace_full_vars_empty() {
    let value = replace_full_vars("");

    assert_eq!(value, "");
}

#[test]
fn replace_full_vars_not_found() {
    let value = replace_full_vars("test 123");

    assert_eq!(value, "test 123");
}

#[test]
fn replace_full_vars_found() {
    let mut value = replace_full_vars("test ${myvar} 123");
    assert_eq!(value, "test %myvar% 123");

    value = replace_full_vars("test ${myvar}");
    assert_eq!(value, "test %myvar%");

    value = replace_full_vars("test ${myvar} ${myvar2} somethingelse ${myvar3}");
    assert_eq!(value, "test %myvar% %myvar2% somethingelse %myvar3%");
}

#[test]
fn replace_partial_vars_empty() {
    let value = replace_partial_vars("");

    assert_eq!(value, "");
}

#[test]
fn replace_partial_vars_not_found() {
    let value = replace_partial_vars("test 123");

    assert_eq!(value, "test 123");
}

#[test]
fn replace_partial_vars_found() {
    let mut value = replace_partial_vars("test $myvar 123");
    assert_eq!(value, "test %myvar% 123");

    value = replace_partial_vars("test $myvar");
    assert_eq!(value, "test %myvar%");

    value = replace_partial_vars("test $myvar $myvar2 somethingelse $myvar3");
    assert_eq!(value, "test %myvar% %myvar2% somethingelse %myvar3%");
}

#[test]
fn replace_vars_empty() {
    let value = replace_vars("");

    assert_eq!(value, "");
}

#[test]
fn replace_vars_not_found() {
    let value = replace_vars("test 123");

    assert_eq!(value, "test 123");
}

#[test]
fn replace_vars_full_syntax() {
    let mut value = replace_vars("test ${myvar} 123");
    assert_eq!(value, "test %myvar% 123");

    value = replace_vars("test ${myvar}");
    assert_eq!(value, "test %myvar%");

    value = replace_vars("test ${myvar} ${myvar2} somethingelse ${myvar3}");
    assert_eq!(value, "test %myvar% %myvar2% somethingelse %myvar3%");
}

#[test]
fn replace_vars_partial_syntax() {
    let mut value = replace_vars("test $myvar 123");
    assert_eq!(value, "test %myvar% 123");

    value = replace_vars("test $myvar");
    assert_eq!(value, "test %myvar%");

    value = replace_vars("test $myvar $myvar2 somethingelse $myvar3");
    assert_eq!(value, "test %myvar% %myvar2% somethingelse %myvar3%");
}

#[test]
fn replace_vars_mixed() {
    let mut value = replace_vars("test $myvar ${myvar2} 123");
    assert_eq!(value, "test %myvar% %myvar2% 123");

    value = replace_vars("${somevar1} test $myvar");
    assert_eq!(value, "%somevar1% test %myvar%");

    value = replace_vars("test $myvar ${myvar2} somethingelse $myvar3");
    assert_eq!(value, "test %myvar% %myvar2% somethingelse %myvar3%");
}

#[test]
fn replace_params_full() {
    let mut value = replace_full_vars(
        "echo 0=${0} 1=${1} 2=${2} 3=${3} 4=${4} 5=${5} 6=${6} 7=${7} 8=${8} 9=${9}",
    );
    assert_eq!(
        value,
        "echo 0=%0 1=%1 2=%2 3=%3 4=%4 5=%5 6=%6 7=%7 8=%8 9=%9"
    );

    value = replace_full_vars("echo ${@}");
    assert_eq!(value, "echo %*");
}

#[test]
fn replace_params_partial_syntax() {
    let mut value = replace_partial_vars("echo 0=$0 1=$1 2=$2 3=$3 4=$4 5=$5 6=$6 7=$7 8=$8 9=$9");
    assert_eq!(
        value,
        "echo 0=%0 1=%1 2=%2 3=%3 4=%4 5=%5 6=%6 7=%7 8=%8 9=%9"
    );

    value = replace_partial_vars("echo $@");
    assert_eq!(value, "echo %*");
}

#[test]
fn replace_params_mixed() {
    let mut value =
        replace_vars("echo 0=$0 1=${1} 2=$2 3=${3} 4=$4 5=${5} 6=$6 7=${7} 8=$8 9=${9}");
    assert_eq!(
        value,
        "echo 0=%0 1=%1 2=%2 3=%3 4=%4 5=%5 6=%6 7=%7 8=%8 9=%9"
    );

    value = replace_vars("echo $@ ${@}");
    assert_eq!(value, "echo %* %*");
}

#[test]
fn replace_params_mixed_with_non_numeric() {
    let mut value = replace_vars(
        "echo 0=$0 1=${1} 2=$2 3=${3} 4=$4 5=${5} 6=$6 7=${7} 8=$8 9=${9} ${somevar1} test $myvar",
    );
    assert_eq!(
        value,
        "echo 0=%0 1=%1 2=%2 3=%3 4=%4 5=%5 6=%6 7=%7 8=%8 9=%9 %somevar1% test %myvar%"
    );

    value = replace_vars("echo $@ ${@}");
    assert_eq!(value, "echo %* %*");
}

#[test]
fn run_empty() {
    let output = run("");

    assert_eq!(output, "");
}

#[test]
fn run_comment() {
    let output = run("#comment");

    assert_eq!(output, "@REM comment");
}

#[test]
fn run_command() {
    let output = run("cp file1 file2");

    assert_eq!(output, "copy file1 file2");
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
copy file1 file2

@REM another
move file2 file3
"#
    );
}

#[test]
fn convert_line_empty() {
    let output = convert_line("");

    assert_eq!(output, "");
}

#[test]
fn convert_line_unhandled() {
    let output = convert_line("newcommand path/arg1 path/arg2");

    assert_eq!(output, "newcommand path/arg1 path/arg2");
}

#[test]
fn convert_line_comment() {
    let output = convert_line("#test/test");

    assert_eq!(output, "@REM test/test");
}

#[test]
fn convert_line_cp() {
    let output = convert_line("cp dir/file1 dir/file2");

    assert_eq!(output, "copy dir\\file1 dir\\file2");
}

#[test]
fn convert_line_cp_recursive() {
    let output = convert_line("cp -r directory/sub1 director/sub2");

    assert_eq!(output, "xcopy /E directory\\sub1 director\\sub2");
}

#[test]
#[ignore] // Multiple arguments are currently broken.
fn convert_line_cp_multiple() {
    let output = convert_line("cp -r dir/file1 dir/file2 dir2/");

    assert_eq!(output, "for %I in (dir\\file1 dir\\file2) do copy %I dir2\\");
}

#[test]
#[ignore] // Multiple arguments are currently broken.
fn convert_line_cp_multiple_recursive() {
    let output = convert_line("cp -r dir\\sub1 dir\\sub2 dir2\\sub3");

    assert_eq!(output, "for %I in (dir\\file1 dir\\file2) do xcopy /E %I dir2\\");
}


#[test]
fn convert_line_mv() {
    let output = convert_line("mv dir/file1 dir/file2");

    assert_eq!(output, "move dir\\file1 dir\\file2");
}

#[test]
fn convert_line_ls() {
    let output = convert_line("ls");

    assert_eq!(output, "dir");
}

#[test]
fn convert_line_rm() {
    let output = convert_line("rm dir/file");

    assert_eq!(output, "del dir\\file");
}

#[test]
fn convert_line_rm_no_prompt() {
    let output = convert_line("rm -f dir/file");

    assert_eq!(output, "del /Q dir\\file");
}

#[test]
fn convert_line_rm_recursive() {
    let output = convert_line("rm -r dir/file");

    assert_eq!(output, "rmdir /S dir\\file");
}

#[test]
fn convert_line_rm_no_prompt_and_recursive_v1() {
    let output = convert_line("rm -rf dir/file");

    assert_eq!(output, "rmdir /S /Q dir\\file");
}

#[test]
fn convert_line_rm_no_prompt_and_recursive_v2() {
    let output = convert_line("rm -fr dir/file");

    assert_eq!(output, "rmdir /S /Q dir\\file");
}

#[test]
fn convert_line_rm_no_prompt_and_recursive_v3() {
    let output = convert_line("rm -Rf dir/file");

    assert_eq!(output, "rmdir /S /Q dir\\file");
}

#[test]
fn convert_line_rm_no_prompt_and_recursive_v4() {
    let output = convert_line("rm -fR dir/file");

    assert_eq!(output, "rmdir /S /Q dir\\file");
}

#[test]
fn convert_line_mkdir() {
    let output = convert_line("mkdir dir1/dir2");

    assert_eq!(output, "mkdir dir1\\dir2");
}

#[test]
fn convert_line_mkdir_and_parents() {
    let output = convert_line("mkdir -p dir1/dir2");

    assert_eq!(output, "mkdir  dir1\\dir2");
}

#[test]
fn convert_line_clear() {
    let output = convert_line("clear");

    assert_eq!(output, "cls");
}

#[test]
fn convert_line_grep() {
    let output = convert_line("grep");

    assert_eq!(output, "find");
}

#[test]
fn convert_line_pwd() {
    let output = convert_line("pwd");

    assert_eq!(output, "chdir");
}

#[test]
fn convert_line_export() {
    let output = convert_line("export A=B");

    assert_eq!(output, "set A=B");
}

#[test]
fn convert_line_unset() {
    let output = convert_line("unset A");

    assert_eq!(output, "set A=");
}
