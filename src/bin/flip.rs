use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("give me a source file");
    let source = fs::read_to_string(path).expect("couldn't read source file");
    let flipped = source
        .chars()
        .rev()
        .map(aussie_plus_plus::upside_down::upside_down)
        .collect::<String>();
    print!("{}", flipped);
}
