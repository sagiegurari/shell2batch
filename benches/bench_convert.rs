#![feature(test)]
extern crate shell2batch;
extern crate test;

use test::Bencher;

#[bench]
fn convert(bencher: &mut Bencher) {
    bencher.iter(|| {
        let output = shell2batch::convert(
            r#"

            #this is some test code
            cp file1 file2

            #another
            mv file2 file3
            "#,
        );

        assert_eq!(
            output,
            r#"

@REM this is some test code
copy file1 file2

@REM another
move file2 file3
"#
        );
    });
}
