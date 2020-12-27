#[cfg(test)]
mod year19day02 {
    use std::collections::HashSet;
    use std::fs;
    const ADD_OP_CODE: i64 = 1;
    const MULT_OP_CODE: i64 = 2;
    const HALT_OP_CODE: i64 = 99;
    fn process_opcodes(mut opcodes: Vec<i64>) -> Option<Vec<i64>> {
        let mut active_op_codes = HashSet::new();
        active_op_codes.insert(ADD_OP_CODE);
        active_op_codes.insert(MULT_OP_CODE);
        let mut cursor: usize = 0;
        let mut done = false;
        while !done {
            if opcodes.len() <= cursor {
                return None;
            }
            let opcode = opcodes[cursor];
            if active_op_codes.contains(&opcode) {
                if opcodes.len() <= cursor + 3 {
                    return None;
                }
                let read1 = opcodes[cursor + 1] as usize;
                let read2 = opcodes[cursor + 2] as usize;
                let write = opcodes[cursor + 3] as usize;
                if opcode == ADD_OP_CODE {
                    opcodes[write] = opcodes[read1] + opcodes[read2]
                } else {
                    opcodes[write] = opcodes[read1] * opcodes[read2]
                }
            } else if opcode == HALT_OP_CODE {
                done = true;
            } else {
                return None;
            }
            cursor += 4;
        }
        Some(opcodes)
    }

    fn read_opcodes(input: &str) -> Vec<i64> {
        let mut opcodes = Vec::new();

        for token in (*input).split(",") {
            let parsed_int = token.parse::<i64>();
            match parsed_int {
                Ok(int) => opcodes.push(int),
                Err(_e) => println!("WARNING COULDN'T PARSE TOKEN {}", token),
            }
        }
        opcodes
    }

    fn initialize_opcodes(mut opcodes: Vec<i64>, init1: i64, init2: i64) -> Vec<i64> {
        opcodes[1] = init1;
        opcodes[2] = init2;
        opcodes
    }

    #[test]
    fn day_two_part_one_examples() {
        assert_eq!(
            process_opcodes(read_opcodes(&"1,0,0,0,99")).unwrap(),
            vec![2, 0, 0, 0, 99]
        );
        assert_eq!(
            process_opcodes(read_opcodes(&"2,3,0,3,99")).unwrap(),
            vec![2, 3, 0, 6, 99]
        );
        assert_eq!(
            process_opcodes(read_opcodes(&"2,4,4,5,99,0")).unwrap(),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            process_opcodes(read_opcodes(&"1,1,1,4,99,5,6,0,99")).unwrap(),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
        assert_eq!(
            process_opcodes(read_opcodes(&"1,9,10,3,2,3,11,0,99,30,40,50")).unwrap(),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn day_two_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day02.txt");
        match readresult {
            Ok(input) => {
                let mut opcodes = read_opcodes(&input);
                opcodes = initialize_opcodes(opcodes, 12, 2);
                opcodes = process_opcodes(opcodes).unwrap();
                println!("THE ANSWER TO DAY TWO PART ONE IS {}", opcodes[0])
            }
            Err(_e) => panic!("COULD NOT OPEN FILE"),
        }
    }

    #[test]
    fn pop_vec() {
        let mut foo = Vec::<i64>::new();
        assert_eq!(foo.pop(), None)
    }

    #[test]
    fn range() {
        for i in 3..5 {
            println!("{}", i)
        }
    }

    #[test]
    fn day_two_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day02.txt");
        match readresult {
            Ok(input) => {
                let init_opcodes = read_opcodes(&input);
                for i in 0..99 {
                    for j in 0..99 {
                        let opcodes = initialize_opcodes(init_opcodes.clone(), i, j);
                        let result = process_opcodes(opcodes).unwrap();
                        if result[0] == 19690720 {
                            println!("THE ANSWER TO DAY TWO PART TWO IS {}", i * 100 + j);
                            return;
                        }
                    }
                }
            }
            Err(_e) => panic!("COULD NOT OPEN FILE"),
        }
    }
}
