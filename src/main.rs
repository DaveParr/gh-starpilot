use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let name = &args[1];
        println!("Hello, {}!", name);
    } else {
        println!("Hello, world!");
    }
}
