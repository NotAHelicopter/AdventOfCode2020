use std::env;
use std::fs;
mod test_01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dir = String::from("input/");
    let file = &args[1].to_string();
    dir.push_str(file);
    println!("Running test with directory: {}", dir);

    let _cont = fs::read_to_string(dir)
        .expect("reading fail");

    test_01::solve01();
}

