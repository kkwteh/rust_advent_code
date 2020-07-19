#[cfg(test)]
mod year19day16 {
    use std::fs;
    use std::mem::swap;

    #[derive(Debug)]
    struct PhasePattern {
        max_repeat: usize,
        repeat_index: usize,
        base_index: usize,
    }
    const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];

    impl Iterator for PhasePattern {
        type Item = i64;
        // Here, we define the sequence using `.curr` and `.next`.
        // The return type is `Option<T>`:
        //     * When the `Iterator` is finished, `None` is returned.
        //     * Otherwise, the next value is wrapped in `Some` and returned.
        fn next(&mut self) -> Option<i64> {
            if self.repeat_index == self.max_repeat {
                self.repeat_index = 0;
                if self.base_index < 3 {
                    self.base_index += 1;
                } else {
                    self.base_index = 0;
                }
            } else {
                self.repeat_index += 1;
            }
            Some(BASE_PATTERN[self.base_index])
        }
    }

    fn init_phase_pattern(max_repeat: usize) -> PhasePattern {
        PhasePattern {
            max_repeat: max_repeat,
            repeat_index: 0,
            base_index: 0,
        }
    }

    #[test]
    fn test_phase_pattern_no_repeat() {
        let mut phase_pattern = init_phase_pattern(0);
        let mut result = Vec::<i64>::new();
        for _ in 0..12 {
            if let Some(value) = phase_pattern.next() {
                result.push(value);
            }
        }
        assert_eq!(result, vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0])
    }

    #[test]
    fn test_phase_pattern_one_repeat() {
        let mut phase_pattern = init_phase_pattern(1);
        let mut result = Vec::<i64>::new();
        for _ in 0..12 {
            if let Some(value) = phase_pattern.next() {
                result.push(value);
            }
        }
        assert_eq!(result, vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0])
    }

    fn compute_output_phase(mut input_phase: [i64; 650], num_phases: i64) -> [i64; 650] {
        let mut output_phase: [i64; 650] = [0; 650];
        for _n in 0..num_phases {
            for i in 0..650 {
                let phase_pattern = init_phase_pattern(i);
                let mut new_value = 0;
                for (input_val, phase) in input_phase.iter().zip(phase_pattern) {
                    new_value += input_val * phase;
                }
                output_phase[i] = new_value.abs() % 10;
            }
            swap(&mut input_phase, &mut output_phase);
        }
        input_phase
    }

    #[test]
    fn day_sixteen_part_one_short_example() {
        let mut input_phase: [i64; 650] = [0; 650];
        for (i, c) in "12345678".chars().enumerate() {
            input_phase[i] = c.to_string().parse::<i64>().unwrap();
        }
        let output_phase = compute_output_phase(input_phase, 1);
        assert_eq!(output_phase[0..8], [4, 8, 2, 2, 6, 1, 5, 8]);
    }

    #[test]
    fn day_sixteen_part_one_long_example() {
        let mut input_phase: [i64; 650] = [0; 650];
        for (i, c) in "80871224585914546619083218645595".chars().enumerate() {
            input_phase[i] = c.to_string().parse::<i64>().unwrap();
        }
        let output_phase = compute_output_phase(input_phase, 100);
        assert_eq!(output_phase[0..8], [2, 4, 1, 7, 6, 1, 7, 6]);
    }

    #[test]
    fn day_sixteen_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day16.txt");
        let mut input_phase: [i64; 650] = [0; 650];
        let mut output_phase: [i64; 650] = [0; 650];
        if let Ok(input) = readresult {
            // populate input_phase
            for (i, c) in input.chars().enumerate() {
                input_phase[i] = c.to_string().parse::<i64>().unwrap();
            }
            let output_phase = compute_output_phase(input_phase, 100);
            let result = &output_phase[0..8];
            println!("THE ANSWER TO DAY SIXTEEN PART ONE IS {:?}", result);
        }
    }

    // #[test]
    fn day_sixteen_part_two_challenge() {
        // let readresult = fs::read_to_string("adventinputs/year19day16.txt");
        // let mut input_phase: [i64; 650_0000] = [0; 650_0000];
        // let mut output_phase: [i64; 650_0000] = [0; 650_0000];
        // if let Ok(input) = readresult {
        //     // populate input_phase
        //     for j in 0..10000 {
        //         for (i, c) in input.chars().enumerate() {
        //             input_phase[i + 10000 * j] = c.to_string().parse::<i64>().unwrap();
        //         }
        //     }
        //     let output_phase = compute_output_phase_for_real(input_phase, 1);
        //     let result = &output_phase[0..8];
        //     println!("INTERMEDIATE VALUE IS {:?}", result);
        // }
    }
}
