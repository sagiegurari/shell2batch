# shell2batch

[![crates.io](https://img.shields.io/crates/v/shell2batch.svg)](https://crates.io/crates/shell2batch) [![Build Status](https://travis-ci.org/sagiegurari/shell2batch.svg)](http://travis-ci.org/sagiegurari/shell2batch) [![Build status](https://ci.appveyor.com/api/projects/status/yrb4y9cbaf6wtlk7?svg=true)](https://ci.appveyor.com/project/sagiegurari/shell2batch) [![codecov](https://codecov.io/gh/sagiegurari/shell2batch/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/shell2batch)<br>
[![license](https://img.shields.io/crates/l/shell2batch.svg)](https://github.com/sagiegurari/shell2batch/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/shell2batch.svg)](https://libraries.io/cargo/shell2batch) [![Documentation](https://docs.rs/shell2batch/badge.svg)](https://docs.rs/crate/shell2batch/) [![downloads](https://img.shields.io/crates/d/shell2batch.svg)](https://crates.io/crates/shell2batch)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Coverts simple basic shell scripts to windows batch scripts.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/shell2batch/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](#history)
* [License](#license)

<a name="overview"></a>
## Overview
While it is not really possible to take every shell script and automatically convert it to a windows batch file, this library provides a way to convert simple basic shell commands to windows batch commands.<br>
The original goal of this library is to provide users of [cargo-make](https://sagiegurari.github.io/cargo-make/) a way to write simple tasks with shell scripts without duplicating their code for each platform.<br>
<br>
It is possible to provide custom conversion hints by using the ```# shell2batch:``` prefix (see below example).

<a name="usage"></a>
## Usage
Simply include the library and invoke the convert function as follows:

```rust
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

copy /B .\file3+,, .\file3

@REM flags are supported
rmdir /S /Q %MY_DIR%

set MY_DIR=

@REM provide custom windows command for specific shell command
complex_windows_command /flag10 windows_value
"#
    );

    println!("Script: {}", script);
}
```

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
shell2batch = "*"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/shell2batch/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/envmnt/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
