use std::fs;

const INPUT: &str = "input/01.txt";
const RESULT: i32 = 2020;

pub fn solve01() {
    println!("\ntest 01: \n");

    let vec: Vec<i32> = fs::read_to_string(INPUT)
        .expect("reading fail")
        .lines().filter_map(|s| s.parse().ok()).collect() ;

    //part1
    for n in vec.iter(){
       let n2 = RESULT - n;
        if vec.contains(&n2) {
            println!("part1: \n{} and {} result = {}", n2, RESULT-n2, n2*(RESULT-n2));
            break;
        }
    }

    //part2
    'outer: for n in vec.iter(){
        for n2 in vec.iter(){
            let n3 = RESULT - (n + n2);
            if vec.contains(&n3) {
                println!("part2: \n{} , {} and {} result = {}", n, n2, n3, n*n2*n3);
                break 'outer;
            }
        }
    }
}