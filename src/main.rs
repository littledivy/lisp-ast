mod ast;
mod parser;

fn main() {
    let args = std::env::args();
    if args.len() > 1 {
        for arg in args.skip(1) {
            let source = std::fs::read_to_string(&arg).expect("Could not read source file");
            println!("{:#?}", parser::parse(&source));
        }
    }
}
