mod year19day17 {
    use crate::intcode::intcode;
    use std::convert::TryFrom;
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    #[derive(Debug)]
    enum Tile {
        Empty,
        Scaffold,
    }

    #[derive(Debug)]
    enum Direction {
        U,
        D,
        L,
        R,
    }

    #[derive(Debug)]
    struct Ship {
        x: usize,
        y: usize,
        direction: Direction,
    }

    fn load_scaffold(mut ctx: intcode::ProgramContext) {
        let mut field: Vec<Vec<Tile>> = Vec::<Vec<Tile>>::new();
        let mut ship = Ship {
            x: 0,
            y: 0,
            direction: Direction::D,
        };
        ctx.run_to_next_input_or_done();
        let outputs = ctx.outputs;
        let mut current_line = Vec::<Tile>::new();
        let mut x = 0;
        let mut y = 0;
        for out in outputs {
            match out {
                46 => {
                    current_line.push(Tile::Empty);
                    x += 1;
                }
                35 => {
                    current_line.push(Tile::Scaffold);
                    x += 1;
                }
                118 => {
                    current_line.push(Tile::Scaffold);
                    ship = Ship {
                        x,
                        y,
                        direction: Direction::D,
                    };
                    x += 1;
                }
                60 => {
                    current_line.push(Tile::Scaffold);
                    ship = Ship {
                        x,
                        y,
                        direction: Direction::L,
                    };
                    x += 1;
                }
                62 => {
                    current_line.push(Tile::Scaffold);
                    ship = Ship {
                        x,
                        y,
                        direction: Direction::R,
                    };
                    x += 1;
                }
                94 => {
                    current_line.push(Tile::Scaffold);
                    ship = Ship {
                        x,
                        y,
                        direction: Direction::U,
                    };
                    x += 1;
                }
                10 => {
                    field.push(current_line);
                    current_line = Vec::<Tile>::new();
                    x = 0;
                    y += 1;
                }
                out => panic!("Unexpected string {}", out),
            }
        }
        field.pop(); // pop last empty line
        let line_length = field[0].len();
        println!("Line length {}", line_length);
        let num_lines = field.len();
        println!("Num lines {}", num_lines);
        let mut alignment_sum = 0;
        for (y, line) in field.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if (x, y) == (ship.x, ship.y) {
                    print!("{:?}", ship.direction);
                } else {
                    match tile {
                        Tile::Empty => print!("."),
                        Tile::Scaffold => print!("#"),
                    }
                }
                if x == 0 || y == 0 || x == line_length - 1 || y == num_lines - 1 {
                    continue;
                }
                if let Tile::Scaffold = field[y][x] {
                    if let Tile::Scaffold = field[y - 1][x] {
                        if let Tile::Scaffold = field[y + 1][x] {
                            if let Tile::Scaffold = field[y][x - 1] {
                                if let Tile::Scaffold = field[y][x + 1] {
                                    alignment_sum += x * y;
                                }
                            }
                        }
                    }
                }
            }
            println!("")
        }
        println!("Alignment sum {}", alignment_sum);
        println!("Ship location {:?}", ship);
    }

    // A,B,A,B,C,C,B,A,B,C
    // A 76,44,49,50,44,76,44,49,48,44,82,44,56,44,76,44,49,50,44,10
    // B 82,44,56,44,82,44,49,48,44,82,44,49,50,44,10
    // C 76,44,49,48,44,82,44,49,50,44,82,44,56,44,10
    fn navigate_scaffold(mut ctx: intcode::ProgramContext) {
        // let input_tokens = "65,44,66,44,65,44,66,44,67,44,67,44,66,44,65,44,66,44,67,10,\
        // 76,44,49,50,44,76,44,49,48,44,82,44,56,44,76,44,49,50,44,10,\
        // 82,44,56,44,82,44,49,48,44,82,44,49,50,44,10,\
        // 76,44,49,48,44,82,44,49,50,44,82,44,56,44,10,\
        // 121,10"
        let input_tokens = "65,44,66,44,65,44,66,44,67,44,67,44,66,44,65,44,66,44,67,10,\
        76,44,49,50,44,76,44,49,48,44,82,44,56,44,76,44,49,50,10,\
        82,44,56,44,82,44,49,48,44,82,44,49,50,10,\
        76,44,49,48,44,82,44,49,50,44,82,44,56,10,\
        121,10"
            .to_owned();
        println!("Input {}", input_tokens);
        ctx.inputs = intcode::read_tokens(&input_tokens);
        ctx.run_to_next_input_or_done();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("adventoutputs/year19day17parttwo.txt")
            .unwrap();
        for output in ctx.outputs.iter() {
            match u8::try_from(*output) {
                Ok(c) => {
                    let _result = write!(file, "{}", c as char);
                }
                Err(_e) => {
                    let _result = write!(file, "{}", output);
                }
            };
        }
        println!("Output length: {}", ctx.outputs.len());
    }

    #[test]
    fn day_seventeen_challenge_part_one() {
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/year19day17.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let ctx = intcode::build_program_context(program, vec![]);
            load_scaffold(ctx);
        }
    }

    #[test]
    fn day_seventeen_challenge_part_two() {
        use std::fs;
        let mut readresult = fs::read_to_string("adventinputs/year19day17.txt");
        if let Ok(input) = readresult.as_mut() {
            input.replace_range(..1, "2");
            let program = intcode::read_tokens(&input);
            let ctx = intcode::build_program_context(program, vec![]);
            navigate_scaffold(ctx);
        }
    }

    #[test]
    fn replace_range() {
        let mut s = String::from("123456");
        s.replace_range(..1, "X");
        assert_eq!(&s[..], "X23456");
    }

    #[test]
    fn string_across_multiple_lines() {
        let string = "one line \
                          written over \
                          several";
        assert_eq!(string, "one line written over several");
    }
}
