pub fn solution(file: String) -> (u64, u64) {
    let mut part_1_sum = 0;
    let mut part_2_sum = 0;
    for line in file.lines() {
        let joltage = parse_joltage(line);
        part_1_sum += joltage;
        let joltage_2 = parse_joltage_2(line);
        part_2_sum += joltage_2;
    }
    (part_1_sum, part_2_sum)
}

fn parse_joltage(line: &str) -> u64 {
    let mut max_joltage: u64 = 0;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let mut j = i + 1;
        while j < chars.len() {
            let c2 = chars[j];
            let candidate = format!("{}{}", c, c2).parse::<u64>().unwrap();
            if candidate > max_joltage {
                max_joltage = candidate;
            }
            j += 1;
        }
        i += 1;
    }
    max_joltage
}

fn parse_joltage_2(line: &str) -> u64 {
    let max_joltage: u64;
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    if len < 12 {
        return line.parse::<u64>().unwrap();
    }

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_joltage() {
        assert_eq!(super::parse_joltage("123"), 23);
        assert_eq!(super::parse_joltage("456"), 56);
        assert_eq!(super::parse_joltage("911111118"), 98);
        assert_eq!(super::parse_joltage("987654321111111"), 98);
        assert_eq!(super::parse_joltage("811111111111119"), 89);
        assert_eq!(super::parse_joltage("234234234234278"), 78);
        assert_eq!(super::parse_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_parse_joltage_2() {
        assert_eq!(super::parse_joltage_2("123"), 123);
        assert_eq!(super::parse_joltage_2("987654321111111"), 987654321111);
        assert_eq!(super::parse_joltage_2("811111111111119"), 811111111119);
        assert_eq!(super::parse_joltage_2("234234234234278"), 434234234278);
        assert_eq!(super::parse_joltage_2("818181911112111"), 888911112111);
    }
}
