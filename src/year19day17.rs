mod year19day17 {
    use crate::intcode::intcode;

    #[derive(Debug)]
    enum Tile {
        Empty,
        Scaffold,
    }

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

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
            direction: Direction::Down,
        };
        ctx.run_to_next_input_or_done();
        let outputs = ctx.outputs;
        let mut current_line = Vec::<Tile>::new();
        let mut x = 0;
        let mut y = 0;
        for s in outputs {
            match &s[..] {
                "46" => current_line.push(Tile::Empty),
                "35" => current_line.push(Tile::Scaffold),
                "118" => {
                    current_line.push(Tile::Scaffold);
                    ship = Ship {
                        x,
                        y,
                        direction: Direction::Down,
                    };
                }
                "60" => {
                    current_line.push(Tile::Scaffold);
                    ship = Ship {
                        x,
                        y,
                        direction: Direction::Left,
                    };
                }
                "62" => {
                    current_line.push(Tile::Scaffold);
                    ship = Ship {
                        x,
                        y,
                        direction: Direction::Right,
                    };
                }
                "94" => {
                    current_line.push(Tile::Scaffold);
                    ship = Ship {
                        x,
                        y,
                        direction: Direction::Up,
                    };
                }
                "10" => {
                    field.push(current_line);
                    current_line = Vec::<Tile>::new();
                    x = 0;
                    y += 1;
                }
                st => panic!("Unexpected string {}", st),
            }
            x += 1;
        }
        field.pop(); // pop last empty line
        let line_length = field[0].len();
        let num_lines = field.len();
        let mut alignment_sum = 0;
        for (y, line) in field.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if x == 0 || y == 0 || x == line_length - 1 || y == num_lines - 1 {
                    continue;
                }
                if let Tile::Scaffold = field[y][x] {
                    if let Tile::Scaffold = field[y - 1][x] {
                        if let Tile::Scaffold = field[y + 1][x] {
                            if let Tile::Scaffold = field[y][x - 1] {
                                if let Tile::Scaffold = field[y][x + 1] {
                                    println!("Found intersection at ({}, {})", x, y);
                                    alignment_sum += x * y;
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("Alignment sum {}", alignment_sum);
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use std::fs;
        #[test]
        fn day_seventeen_challenge_part_one() {
            let readresult = fs::read_to_string("adventinputs/year19day17.txt");
            if let Ok(input) = readresult {
                let program = intcode::read_tokens(&input);
                let ctx = intcode::build_program_context(program, vec![]);
                load_scaffold(ctx);
            }
        }

        #[test]
        fn enum_equality() {
            let foo = Tile::Empty;
        }
    }
}
