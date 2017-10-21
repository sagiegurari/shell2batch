extern crate shell2batch;

fn main() {
    let script = shell2batch::convert("rm ./myfile.txt");

    assert_eq!(script, "del ./myfile.txt");
    println!("Script: {}", script);
}
