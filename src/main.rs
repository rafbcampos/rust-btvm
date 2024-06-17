use scanner::scan;

mod scanner;

fn main() {
    let source = "\"hello\"123.45";
    let scanner = scan(source);
    println!("{:?}", scanner);
}
