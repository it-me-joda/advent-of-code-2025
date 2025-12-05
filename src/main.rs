mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let d1_file = std::fs::read_to_string("data/d1.txt").expect("Failed to read file");
    let (d1_p1_solution, d1_p2_solution) = day1::solution(d1_file);
    println!("Day 1 Part 1 Solution: {}", d1_p1_solution);
    println!("Day 1 Part 2 Solution: {}", d1_p2_solution);

    let d2_file = std::fs::read_to_string("data/d2.txt").expect("Failed to read file");
    let (d2_p1_solution, d2_p2_solution) = day2::solution(d2_file);
    println!("Day 2 Part 1 Solution: {}", d2_p1_solution);
    println!("Day 2 Part 2 Solution: {}", d2_p2_solution);

    let d3_file = std::fs::read_to_string("data/d3.txt").expect("Failed to read file");
    let (d3_p1_solution, d3_p2_solution) = day3::solution(d3_file);
    println!("Day 3 Part 1 Solution: {}", d3_p1_solution);
    println!("Day 3 Part 2 Solution: {}", d3_p2_solution);

    let d4_file = std::fs::read_to_string("data/d4.txt").expect("Failed to read file");
    let (d4_p1_solution, d4_p2_solution) = day4::solution(d4_file);
    println!("Day 4 Part 1 Solution: {}", d4_p1_solution);
    println!("Day 4 Part 2 Solution: {}", d4_p2_solution);
}
