#[cfg(test)]
mod year19day05 {
    use crate::intcode::intcode;
    use std::fs;

    #[test]
    fn day_five_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day05.txt");
        match readresult {
            Ok(input) => {
                let program = intcode::read_tokens(&input);
                let input = vec![1];
                let output = intcode::run_program(program, input);
                println!("THE ANSWER TO DAY 5 PART 1 IS {}", output.last().unwrap())
            }
            Err(_e) => panic!("COULD NOT OPEN FILE"),
        }
    }

    #[test]
    fn day_five_part_two_example_01() {
        let tokens = "3,9,8,9,10,9,4,9,99,-1,8";
        let program = intcode::read_tokens(tokens);
        let input = vec![8];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn day_five_part_two_example_02() {
        let tokens = "3,9,8,9,10,9,4,9,99,-1,8";
        let program = intcode::read_tokens(tokens);
        let input = vec![9999];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn day_five_part_two_example_03() {
        let tokens = "3,3,1108,-1,8,3,4,3,99";
        let program = intcode::read_tokens(tokens);
        let input = vec![8];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn day_five_part_two_example_04() {
        let tokens = "3,3,1108,-1,8,3,4,3,99";
        let program = intcode::read_tokens(tokens);
        let input = vec![9999];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn day_five_part_two_example_05() {
        let tokens = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let program = intcode::read_tokens(tokens);
        let input = vec![0];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn day_five_part_two_example_06() {
        let tokens = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let program = intcode::read_tokens(tokens);
        let input = vec![2];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn day_five_part_two_example_07() {
        let tokens = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let program = intcode::read_tokens(tokens);
        let input = vec![0];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn day_five_part_two_example_08() {
        let tokens = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let program = intcode::read_tokens(tokens);
        let input = vec![2];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn day_five_part_two_example_09() {
        let tokens = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let program = intcode::read_tokens(tokens);
        let input = vec![7];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 999);
    }

    #[test]
    fn day_five_part_two_example_10() {
        let tokens = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let program = intcode::read_tokens(tokens);
        let input = vec![8];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 1000);
    }

    #[test]
    fn day_five_part_two_example_11() {
        let tokens = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let program = intcode::read_tokens(tokens);
        let input = vec![9];
        let output = intcode::run_program(program, input);
        assert_eq!(output[0], 1001);
    }

    #[test]
    fn day_five_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day05.txt");
        match readresult {
            Ok(input) => {
                let program = intcode::read_tokens(&input);
                let input = vec![5];
                let output = intcode::run_program(program, input);
                println!("THE ANSWER TO DAY FIVE PART TWO IS {}", output[0])
            }
            Err(_e) => panic!("COULD NOT OPEN FILE"),
        }
    }

    #[test]
    fn test_modular_math() {
        assert_eq!(125 - 125 / 100 * 100, 25)
    }

    #[test]
    fn test_parse_int() {
        assert_eq!("01".parse::<u32>().unwrap(), 1)
    }

    #[test]
    fn test_concat_str() {
        let a = "Hello";
        let b = "World";
        let c = format!("{} {}", a, b);
        assert_eq!(c, "Hello World")
    }

    #[test]
    fn test_push_str() {
        let mut a = "Hello".to_string();
        a.push_str(" World");
        assert_eq!(a, "Hello World");
    }

    #[test]
    fn test_sub_string() {
        let a = "Hello".to_string();
        assert_eq!(&a[..3], "Hel");
    }

    #[test]
    fn test_lpad() {
        let padded_opcode = format!("{:0>5}", "3");
        assert_eq!(padded_opcode, "00003")
    }

    #[test]
    fn test_reverse_chars() {
        let opcode = "12302";
        for c in opcode[..3].chars().rev() {
            println!("{}", c);
        }
    }

    #[test]
    fn test_vec_str() {
        let mut foo: Vec<&str> = vec!["foo", "bar", "baz"];
        foo[2] = "cor";
        println!("{:?}", foo)
    }

    #[test]
    fn test_vec_str_computed_value() {
        let mut foo: Vec<String> = vec!["foo".to_string()];
        let computed_value: &str;
        {
            computed_value = "asdf";
            foo[0] = computed_value.to_string();
        }
        println!("{:?}", foo)
    }
}
