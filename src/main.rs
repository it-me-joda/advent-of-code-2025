fn main() {
    let d1_file = std::fs::read_to_string("data/d1.txt").expect("Failed to read file");
    let (d1_p1_password, d1_p2_password) = day1(d1_file);
    println!("Day 1 Part 1 Password: {}", d1_p1_password);
    println!("Day 1 Part 2 Password: {}", d1_p2_password);
}

/// Returns (part1_result, part2_result)
fn day1(file: String) -> (i32, i32) {
    let mut current = 50;
    let mut landed_on_zero_count = 0;
    let mut zero_count = 0;

    for line in file.lines() {
        let (direction, number) = parse_line(line);
        let (new_position, zero_times) = rotate(current, direction, number);

        if new_position == 0 {
            landed_on_zero_count += 1;
        }

        current = new_position;
        zero_count += zero_times;
    }

    (landed_on_zero_count, zero_count)
}

/// code is in format `<letter><number>`, e.g. "L40" "R20"
fn parse_line(line: &str) -> (&str, i32) {
    let (direction, number) = line.split_at(1);
    let number: i32 = number.parse().expect("Failed to parse number");

    (direction, number)
}

/// Rotates the current position left or right by the given number
/// Returns the new position and the number of times zero was passed
fn rotate(current: i32, direction: &str, number: i32) -> (i32, i32) {
    let mut new_position = current;
    let mut zero_passed = number / 100;
    let movement = number % 100;
    if movement == 0 {
        return (new_position, zero_passed);
    }

    match direction {
        "L" => {
            new_position -= movement;
            if new_position <= 0 && current != 0 {
                zero_passed += 1;
            }
            if new_position < 0 {
                new_position += 100;
            }
        }
        "R" => {
            new_position += movement;
            if new_position > 99 {
                zero_passed += 1;
                new_position -= 100;
            }
        }
        _ => panic!("Invalid direction"),
    }

    (new_position, zero_passed)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_line() {
        let (dir, num) = super::parse_line("L40");
        assert_eq!(dir, "L");
        assert_eq!(num, 40);

        let (dir, num) = super::parse_line("R120");
        assert_eq!(dir, "R");
        assert_eq!(num, 120);
    }

    #[test]
    fn test_rotate() {
        // rotate without passing zero
        assert_eq!(super::rotate(50, "L", 20), (30, 0));
        assert_eq!(super::rotate(0, "R", 99), (99, 0));

        // rotate passing zero once
        assert_eq!(super::rotate(5, "L", 10), (95, 1));
        assert_eq!(super::rotate(10, "L", 20), (90, 1));
        assert_eq!(super::rotate(90, "R", 15), (5, 1));

        // rotate passing zero multiple times
        assert_eq!(super::rotate(50, "R", 1000), (50, 10));
        assert_eq!(super::rotate(50, "L", 1000), (50, 10));
        assert_eq!(super::rotate(0, "L", 603), (97, 6));

        // rotate landing on zero
        assert_eq!(super::rotate(50, "L", 550), (0, 6));
        assert_eq!(super::rotate(50, "R", 550), (0, 6));
        assert_eq!(super::rotate(10, "L", 10), (0, 1));
        assert_eq!(super::rotate(0, "R", 100), (0, 1));
        assert_eq!(super::rotate(99, "R", 1), (0, 1));
        assert_eq!(super::rotate(1, "L", 1), (0, 1));
    }

    #[test]
    fn test_day1() {
        let input = "L50\nR100";
        let (part1, part2) = super::day1(input.to_string());
        assert_eq!(part1, 2);
        assert_eq!(part2, 2);

        let input = "R150\nL200\nR50";
        let (part1, part2) = super::day1(input.to_string());
        assert_eq!(part1, 2);
        assert_eq!(part2, 4);

        let input = "L40\nR20\nL70\nR150\nL300\nR50";
        let (part1, part2) = super::day1(input.to_string());
        assert_eq!(part1, 0);
        assert_eq!(part2, 6);

        let input = "L40\nR27\nR20\nR12\nR28\nL9\nR31\nR45\nL19\nR2\nL25\nL46\nR1\nR26\nL44\nL27\nR11\nR41\nL11\nR14";
        let (part1, part2) = super::day1(input.to_string());
        assert_eq!(part1, 0);
        assert_eq!(part2, 4);
    }
}
