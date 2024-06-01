#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # shell2batch
//!
//! Coverts simple basic shell scripts to windows batch scripts.
//!
//! While it is not really possible to take every shell script and automatically convert it to a windows batch file,
//! this library provides a way to convert simple basic shell commands to windows batch commands.<br>
//! The original goal of this library is to provide users of [cargo-make](https://sagiegurari.github.io/cargo-make/) a
//! way to write simple tasks with shell scripts without duplicating their code for each platform.
//!
//! It is possible to provide custom conversion hints by using the ```# shell2batch:``` prefix (see below example).
//!
//! # Examples
//!
//! ```
//! fn main() {
//!     let script = shell2batch::convert(
//!         r#"
//!         set -x
//!
//!         export FILE1=file1
//!         export FILE2=file2
//!
//!         #this is some test code
//!         cp ${FILE1} $FILE2
//!         cp -r ${DIR1} $DIR2
//!
//!         #another
//!         mv file2 file3
//!
//!         export MY_DIR=directory
//!
//!         #flags are supported
//!         rm -Rf ${MY_DIR}
//!
//!         unset MY_DIR
//!
//!         touch ./file3
//!
//!         #provide custom windows command for specific shell command
//!         complex_bash_command --flag1 value2 # shell2batch: complex_windows_command /flag10 windows_value
//!         "#,
//!     );
//!
//!     assert_eq!(
//!         script,
//!         r#"
//!@echo on
//!
//!set FILE1=file1
//!set FILE2=file2
//!
//!@REM this is some test code
//!copy %FILE1% %FILE2%
//!xcopy /E %DIR1% %DIR2%
//!
//!@REM another
//!move file2 file3
//!
//!set MY_DIR=directory
//!
//!@REM flags are supported
//!rmdir /S /Q %MY_DIR%
//!
//!set MY_DIR=
//!
//!copy /B .\file3+,, .\file3
//!
//!@REM provide custom windows command for specific shell command
//!complex_windows_command /flag10 windows_value
//!"#
//!     );
//!
//!     println!("Script: {}", script);
//! }
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/shell2batch/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/shell2batch/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod converter;

/// Converts the provided shell script and returns the windows batch script text.
///
/// # Example
///
/// ```
/// fn main() {
///     let script = shell2batch::convert(
///         r#"
///         set -x
///
///         export FILE1=file1
///         export FILE2=file2
///
///         #this is some test code
///         cp ${FILE1} $FILE2
///         cp -r ${DIR1} $DIR2
///
///         #another
///         mv file2 file3
///
///         export MY_DIR=directory
///
///         #flags are supported
///         rm -Rf ${MY_DIR}
///
///         unset MY_DIR
///
///         touch ./file3
///
///         #provide custom windows command for specific shell command
///         complex_bash_command --flag1 value2 # shell2batch: complex_windows_command /flag10 windows_value
///         "#,
///     );
///
///     assert_eq!(
///         script,
///         r#"
///@echo on
///
///set FILE1=file1
///set FILE2=file2
///
///@REM this is some test code
///copy %FILE1% %FILE2%
///xcopy /E %DIR1% %DIR2%
///
///@REM another
///move file2 file3
///
///set MY_DIR=directory
///
///@REM flags are supported
///rmdir /S /Q %MY_DIR%
///
///set MY_DIR=
///
///copy /B .\file3+,, .\file3
///
///@REM provide custom windows command for specific shell command
///complex_windows_command /flag10 windows_value
///"#
///     );
///
///     println!("Script: {}", script);
/// }
/// ```
pub fn convert(script: &str) -> String {
    converter::run(script)
}
