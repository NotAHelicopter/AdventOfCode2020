use std::fs;

const INPUT: &str = "input/02.txt";


pub fn solve02() {
    println!("\ntest 02: \npart 1");
    let vec: Vec<i32> = fs::read_to_string(INPUT)
        .expect("reading fail")
        .lines().filter_map(|s| s.parse().ok()).collect() ;

    println!("{:?}", vec);

}