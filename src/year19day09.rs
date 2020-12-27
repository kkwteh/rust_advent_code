#[cfg(test)]
mod year19day09 {
    use crate::intcode::intcode;
    use std::fs;
    #[test]
    fn day_nine_part_one_example_01() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let result = intcode::run_program(program, Vec::<i64>::new());
        let expected = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn day_nine_part_one_example_02() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let result = intcode::run_program(program, Vec::<i64>::new());
        assert_eq!(result.last().unwrap().to_string().len(), 16);
    }

    #[test]
    fn year19day09_part_one_example_03() {
        let program = vec![104, 1125899906842624, 99];

        let result = intcode::run_program(program, Vec::<i64>::new());
        assert_eq!(*result.last().unwrap(), 1125899906842624);
    }
    #[test]
    fn day_nine_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day09.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let result = intcode::run_program(program, vec![1]);
            println!("THE ANSWER TO 2019 DAY NINE PART ONE IS {:?}", result)
        }
    }

    #[test]
    fn day_nine_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day09.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let result = intcode::run_program(program, vec![2]);
            println!("THE ANSWER TO 2019 DAY NINE PART TWO IS {:?}", result)
        }
    }
}
