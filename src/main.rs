mod day1;
mod day2;

fn main() {
    let d1_file = std::fs::read_to_string("data/d1.txt").expect("Failed to read file");
    let (d1_p1_password, d1_p2_password) = day1::solution(d1_file);
    println!("Day 1 Part 1 Solution: {}", d1_p1_password);
    println!("Day 1 Part 2 Solution: {}", d1_p2_password);

    let d2_file = std::fs::read_to_string("data/d2.txt").expect("Failed to read file");
    let (d2_p1_password, d2_p2_password) = day2::solution(d2_file);
    println!("Day 2 Part 1 Solution: {}", d2_p1_password);
    println!("Day 2 Part 2 Solution: {}", d2_p2_password);
}
