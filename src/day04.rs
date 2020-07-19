#[cfg(test)]
mod day04 {
    fn has_two_adjacent_digits_same(num: u32) -> bool {
        let mut prev_char: char = 'x';
        for c in num.to_string().chars() {
            if c == prev_char {
                return true;
            }
            prev_char = c;
        }
        false
    }

    fn has_an_exact_double(num: u32) -> bool {
        let mut current_streak = 0;
        let mut prev_char: Option<char> = None;
        for c in num.to_string().chars() {
            if Some(c) == prev_char {
                current_streak += 1;
            } else {
                if current_streak == 2 {
                    return true;
                }
                current_streak = 1;
                prev_char = Some(c);
            }
        }
        current_streak == 2
    }

    fn has_non_descending_digits(num: u32) -> bool {
        let mut prev_digit: u32 = 0;
        for c in num.to_string().chars() {
            let digit = c.to_string().parse::<u32>().unwrap();
            if digit < prev_digit {
                return false;
            }
            prev_digit = digit;
        }
        true
    }

    fn count_part_one_matches(start: u32, end: u32) -> u32 {
        let mut num_matches = 0;
        for candidate in start..end {
            if has_two_adjacent_digits_same(candidate) && has_non_descending_digits(candidate) {
                num_matches += 1;
            }
        }
        num_matches
    }

    fn count_part_two_matches(start: u32, end: u32) -> u32 {
        let mut num_matches = 0;
        for candidate in start..end {
            if has_an_exact_double(candidate) && has_non_descending_digits(candidate) {
                num_matches += 1;
            }
        }
        num_matches
    }

    #[test]
    fn test_has_two_adjancent_digits_same() {
        assert!(has_two_adjacent_digits_same(1223456));
        assert!(!has_two_adjacent_digits_same(123456));
    }

    #[test]
    fn test_has_non_descending_digits() {
        assert!(has_non_descending_digits(111111));
        assert!(!has_non_descending_digits(223450));
    }

    #[test]
    fn test_has_an_exact_double() {
        assert!(has_an_exact_double(112233));
        assert!(!has_an_exact_double(123444));
        assert!(has_an_exact_double(111122));
    }

    #[test]
    fn iterate_through_string() {
        for c in 12345.to_string().chars() {
            println!("{}", c)
        }
    }

    #[test]
    fn day_four_part_one_challenge() {
        let result = count_part_one_matches(245182, 790572);
        println!("THE ANSWER TO DAY FOUR PART ONE IS {}", result);
    }

    #[test]
    fn day_four_part_two_challenge() {
        let result = count_part_two_matches(245182, 790572);
        println!("THE ANSWER TO DAY FOUR PART ONE IS {}", result);
    }
}
