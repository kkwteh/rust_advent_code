#[cfg(test)]
mod advent20191209 {
    use crate::intcode::intcode;
    use std::fs;
    #[test]
    fn advent20191209_part_one_example_01() {
        let program: Vec<String> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();

        let result = intcode::run_program(program, Vec::<String>::new());
        let expected: Vec<String> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn advent20191209_part_one_example_02() {
        let program: Vec<String> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]
            .into_iter()
            .map(|i| i.to_string())
            .collect();

        let result = intcode::run_program(program, Vec::<String>::new());
        assert_eq!(result.last().unwrap().len(), 16);
    }

    #[test]
    fn advent20191209_part_one_example_03() {
        let program: Vec<String> = vec![
            "104".to_string(),
            "1125899906842624".to_string(),
            "99".to_string(),
        ];

        let result = intcode::run_program(program, Vec::<String>::new());
        assert_eq!(*result.last().unwrap(), "1125899906842624".to_string());
    }
    #[test]
    fn advent20191209_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/advent20191209.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let result = intcode::run_program(program, vec!["1".to_string()]);
            println!("THE ANSWER TO 2019 DAY NINE PART ONE IS {:?}", result)
        }
    }

    #[test]
    fn advent20191209_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/advent20191209.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let result = intcode::run_program(program, vec!["2".to_string()]);
            println!("THE ANSWER TO 2019 DAY NINE PART TWO IS {:?}", result)
        }
    }
}
