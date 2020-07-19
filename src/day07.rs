#[cfg(test)]
mod day07 {
    use crate::intcode::intcode;
    use permutohedron::heap_recursive;
    use std::fs;

    fn optimize_signal(program: Vec<String>) -> i64 {
        let mut output = Vec::<String>::new();
        let mut max = i64::min_value();
        let mut argmax = Vec::<String>::new();
        let mut prev_output = "0".to_string();

        let mut phases = [
            0.to_string(),
            1.to_string(),
            2.to_string(),
            3.to_string(),
            4.to_string(),
        ];
        heap_recursive(&mut phases, |permutation| {
            let phase_run = permutation.to_vec();
            prev_output = "0".to_string();
            for phase in &phase_run {
                let input_phase = phase.clone();
                let input = vec![input_phase, prev_output.clone()];
                output = intcode::run_program(program.clone(), input);
                prev_output = output.last().unwrap().clone();
            }
            let final_value = output.last().unwrap().parse::<i64>().unwrap();
            if final_value > max {
                argmax = phase_run;
                max = final_value;
            }
        });
        println!("Argmax is {:?}", argmax);
        max
    }

    fn optimize_feedback_loop(program: Vec<String>) -> i64 {
        let mut max = i64::min_value();
        let mut argmax = Vec::<String>::new();

        let mut phases = [
            5.to_string(),
            6.to_string(),
            7.to_string(),
            8.to_string(),
            9.to_string(),
        ];
        heap_recursive(&mut phases, |permutation| {
            let phase_run = permutation.to_vec();
            let inputs = vec![
                vec![phase_run[0].clone(), "0".to_string()],
                vec![phase_run[1].clone()],
                vec![phase_run[2].clone()],
                vec![phase_run[3].clone()],
                vec![phase_run[4].clone()],
            ];
            let result = intcode::run_multiprogram(program.clone(), inputs);
            if result > max {
                argmax = phase_run;
                max = result;
            }
        });
        println!("Argmax is {:?}", argmax);
        max
    }

    #[test]
    fn day_seven_part_one_example_01() {
        let program: Vec<String> = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();

        let result = optimize_signal(program);
        assert_eq!(result, 43210);
    }

    #[test]
    fn day_seven_part_one_example_02() {
        let program: Vec<String> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();

        let result = optimize_signal(program);
        assert_eq!(result, 54321);
    }

    #[test]
    fn day_seven_part_one_example_03() {
        let program: Vec<String> = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();

        let result = optimize_signal(program);
        assert_eq!(result, 65210);
    }

    #[test]
    fn day_seven_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/day07.txt");
        match readresult {
            Ok(input) => {
                let program = intcode::read_tokens(&input);
                let result = optimize_signal(program);
                println!("THE ANSWER TO DAY SEVEN PART ONE IS {}", result)
            }
            Err(_e) => panic!("COULD NOT OPEN FILE"),
        }
    }

    #[test]
    fn day_seven_part_two_example_01() {
        let program: Vec<String> = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();

        let inputs = vec![
            vec!["9".to_string(), "0".to_string()],
            vec!["8".to_string()],
            vec!["7".to_string()],
            vec!["6".to_string()],
            vec!["5".to_string()],
        ];
        let result = intcode::run_multiprogram(program, inputs);
        assert_eq!(result, 139629729);
    }

    #[test]
    fn day_seven_part_two_example_01_optimize() {
        let program: Vec<String> = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();

        let result = optimize_feedback_loop(program);
        assert_eq!(result, 139629729);
    }

    #[test]
    fn day_seven_part_two_example_02_optimize() {
        let program: Vec<String> = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]
        .into_iter()
        .map(|i| i.to_string())
        .collect();
        let result = optimize_feedback_loop(program);
        assert_eq!(result, 18216);
    }

    #[test]
    fn day_seven_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/day07.txt");
        match readresult {
            Ok(input) => {
                let program = intcode::read_tokens(&input);
                let result = optimize_feedback_loop(program);
                println!("THE ANSWER TO DAY SEVEN PART TWO IS {}", result)
            }
            Err(_e) => panic!("COULD NOT OPEN FILE"),
        }
    }

    #[test]
    fn test_permutohedron() {
        let mut data = [1, 2, 3];
        let mut permutations = Vec::new();
        heap_recursive(&mut data, |permutation| {
            permutations.push(permutation.to_vec())
        });
        println!("{:?}", permutations)
    }
}
