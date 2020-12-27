#[cfg(test)]
mod year19day01 {

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    fn naive_fuel_needed(mass: i64) -> i64 {
        (mass / 3) - 2
    }

    fn fuel_needed(mass: i64) -> i64 {
        let mut res = 0;
        let mut done = false; // mut done: bool
        let mut mass_in_question = mass;
        while !done {
            let direct_fuel = naive_fuel_needed(mass_in_question);
            if direct_fuel > 0 {
                res += direct_fuel;
                mass_in_question = direct_fuel;
            } else {
                done = true;
            }
        }
        res
    }

    #[test]
    fn day_one_part_one_examples() {
        assert_eq!(naive_fuel_needed(12), 2);
        assert_eq!(naive_fuel_needed(100756), 33583);
    }

    #[test]
    fn day_one_part_one_challenge() {
        let f = File::open("adventinputs/year19day01.txt");
        match f {
            Ok(f) => {
                let f = BufReader::new(f);
                let mut res = 0;
                for line in f.lines() {
                    let inputval = line.unwrap().parse::<i64>().unwrap();
                    res += naive_fuel_needed(inputval);
                }
                println!("ANSWER TO ADVENT DAY 1 PART 1 IS {}", res);
            }
            Err(_e) => panic!("COULDN'T OPEN FILE!!!"),
        }
    }

    #[test]
    fn day_one_part_two_examples() {
        assert_eq!(fuel_needed(100756), 50346)
    }

    #[test]
    fn day_one_part_two_challenge() {
        let f = File::open("adventinputs/year19day01.txt");
        match f {
            Ok(f) => {
                let f = BufReader::new(f);
                let mut res = 0;
                for line in f.lines() {
                    let inputval = line.unwrap().parse::<i64>().unwrap();
                    res += fuel_needed(inputval);
                }
                println!("ANSWER TO ADVENT DAY 1 PART 2 IS {}", res);
            }
            Err(_e) => panic!("COULDN'T OPEN FILE!!!"),
        }
    }
}
