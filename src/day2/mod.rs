use rayon::prelude::*;

pub fn solution(file: String) -> (u64, u64) {
    let (sum_of_invalid_ids_part_1, sum_of_invalid_ids_part_2) = file
        .split(',')
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|line| {
            let mut sum_of_invalid_ids_part_1: u64 = 0;
            let mut sum_of_invalid_ids_part_2: u64 = 0;
            let (start, end) = parse_line(line);
            for id in start..=end {
                if !is_valid_id_part_1(id) {
                    sum_of_invalid_ids_part_1 += id;
                }
                if !is_valid_id_part_2(id) {
                    sum_of_invalid_ids_part_2 += id;
                }
            }
            (sum_of_invalid_ids_part_1, sum_of_invalid_ids_part_2)
        })
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    (sum_of_invalid_ids_part_1, sum_of_invalid_ids_part_2)
}

/// code is in format `<number>-<number>`, e.g. "1-3" "10-20"
fn parse_line(line: &str) -> (u64, u64) {
    let segments: Vec<&str> = line.split('-').collect();
    let start_text = segments[0];
    let end_text = segments[1];
    let start = start_text.parse().expect("Failed to parse direction");
    let end = end_text.parse().expect("Failed to parse number");

    (start, end)
}

/// determine if an ID is invalid for part 1
fn is_valid_id_part_1(id: u64) -> bool {
    let id_string = id.to_string();
    if !id_string.len().is_multiple_of(2) {
        return true;
    }

    let half_len = id_string.len() / 2;
    let first_half = &id_string[0..half_len];
    let second_half = &id_string[half_len..];
    if first_half == second_half {
        return false;
    }
    true
}

/// determine if an ID is invalid for part 2
fn is_valid_id_part_2(id: u64) -> bool {
    let id_string = id.to_string();
    let len = id_string.len();
    if len == 1 {
        return true;
    }
    let first_char = id_string.chars().next().unwrap();
    if id_string.chars().all(|c| c == first_char) {
        return false;
    }

    for chunk_size in 1..=len / 2 {
        if !len.is_multiple_of(chunk_size) {
            continue;
        }

        let mut all_chunks_equal = true;
        let first_chunk = &id_string[0..chunk_size];
        for start in (0..len).step_by(chunk_size) {
            let end = start + chunk_size;
            let chunk = &id_string[start..end];
            if chunk != first_chunk {
                all_chunks_equal = false;
                break;
            }
        }

        if all_chunks_equal {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let (start, end) = parse_line("1-3");
        assert_eq!(start, 1);
        assert_eq!(end, 3);

        let (start, end) = parse_line("10-20");
        assert_eq!(start, 10);
        assert_eq!(end, 20);
    }

    #[test]
    fn test_is_valid_id_part_1() {
        // odd number of characters is valid
        assert!(is_valid_id_part_1(123));
        // even number of characters but not repeated is valid
        assert!(is_valid_id_part_1(1234));
        //even number of characters and repeated is invalid
        assert!(!is_valid_id_part_1(1212));
    }

    #[test]
    fn test_is_valid_id_part_2() {
        assert!(is_valid_id_part_2(1));
        assert!(!is_valid_id_part_2(1111111111111111111));
        assert!(!is_valid_id_part_2(11111111111111111111));
        assert!(!is_valid_id_part_2(1212));
        assert!(!is_valid_id_part_2(121212));
        assert!(!is_valid_id_part_2(1212121212));
        assert!(!is_valid_id_part_2(12121212121212));
        assert!(!is_valid_id_part_2(121212121212121212));
        assert!(!is_valid_id_part_2(123123123123123123));
        assert!(!is_valid_id_part_2(12341234123412341234));
        assert!(!is_valid_id_part_2(12345123451234512345));
        assert!(!is_valid_id_part_2(123456123456123456));
        assert!(!is_valid_id_part_2(12345671234567));
        assert!(!is_valid_id_part_2(1234567812345678));
        assert!(!is_valid_id_part_2(123456789123456789));
        assert!(!is_valid_id_part_2(12345678901234567890));
        assert!(is_valid_id_part_2(123451234));
        assert!(is_valid_id_part_2(123412345));
    }

    #[test]
    fn test_solution() {
        let input = String::from("1-9,11-15,22-26,1212-1216,121212-121216");
        let (part1, part2) = solution(input);
        // invalid IDs are 11, 22, 1212, 121212
        assert_eq!(part1, 1245);
        assert_eq!(part2, 122457);
    }
}
