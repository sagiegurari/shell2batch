extern crate shell2batch;

#[test]
fn convert() {
    let script = shell2batch::convert("rm ./myfile.txt");

    assert_eq!(script, "del ./myfile.txt");
}
