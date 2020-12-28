#[cfg(test)]
mod year19day01 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug, Eq, PartialEq)]
    enum Continuation {
        FuelNeeded(i64, i64),
        Done(i64),
    }
    fn naive_fuel_needed(k: Continuation) -> Continuation {
        match k {
            Continuation::FuelNeeded(unaccounted, amount) => {
                let inc_amount = (unaccounted / 3) - 2;
                if inc_amount > 0 {
                    return Continuation::FuelNeeded(inc_amount, amount + inc_amount);
                } else {
                    return Continuation::Done(amount);
                }
            }
            Continuation::Done(_) => panic!("Was passed a done continuation"),
        }
    }

    fn fuel_needed(mass: i64) -> i64 {
        let mut k = Continuation::FuelNeeded(mass, 0);
        loop {
            match k {
                Continuation::FuelNeeded(_, _) => k = naive_fuel_needed(k),
                Continuation::Done(amount) => {
                    return amount;
                }
            }
        }
    }

    #[test]
    fn day_one_part_one_examples() {
        assert_eq!(
            naive_fuel_needed(Continuation::FuelNeeded(12, 0)),
            Continuation::FuelNeeded(2, 2)
        );
        assert_eq!(
            naive_fuel_needed(Continuation::FuelNeeded(100756, 0)),
            Continuation::FuelNeeded(33583, 33583)
        );
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
                    let k = naive_fuel_needed(Continuation::FuelNeeded(inputval, 0));
                    match k {
                        Continuation::FuelNeeded(_, amount) => {
                            res += amount;
                        }
                        Continuation::Done(amount) => {
                            res += amount;
                        }
                    }
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
