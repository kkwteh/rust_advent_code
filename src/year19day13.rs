#[cfg(test)]
mod year19day13 {
    use crate::intcode::intcode;
    use console::Term;
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, VecDeque};
    use std::fs;
    use std::io;

    #[derive(Copy, Clone, Debug)]
    enum Tile {
        Empty,
        Wall,
        Block,
        HorPaddle,
        Ball,
    }

    #[derive(Hash, PartialEq, Clone, Debug, Copy)]
    struct Location {
        x: i64,
        y: i64,
    }
    impl Eq for Location {}

    impl Ord for Location {
        // Ordering is defined so that the min SightLine gets shot first.
        // This way we can use BTree pop_min to sort SightLines.
        fn cmp(&self, other: &Self) -> Ordering {
            if self.y != other.y {
                self.y.cmp(&other.y)
            } else {
                self.x.cmp(&other.x)
            }
        }
    }
    impl PartialOrd for Location {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Clone)]
    struct ScreenPrinter {
        output: Vec<i64>,
        cursor: usize,
        screen: BTreeMap<Location, Tile>,
        score: i64,
    }

    impl ScreenPrinter {
        fn update_screen(&mut self) {
            while self.output.len() > self.cursor + 2 {
                let mut x: i64;
                let mut y: i64;
                let mut tile = Tile::Empty;
                let mut score: i64;
                x = self.output[self.cursor];
                y = self.output[self.cursor + 1];
                if x != -1 {
                    match self.output[self.cursor + 2] {
                        0 => tile = Tile::Empty,
                        1 => tile = Tile::Wall,
                        2 => tile = Tile::Block,
                        3 => tile = Tile::HorPaddle,
                        4 => tile = Tile::Ball,
                        _ => panic!("Unexpected tile type"),
                    };
                } else {
                    self.score = self.output[self.cursor + 2];
                }
                self.cursor += 3;
                if x != -1 {
                    self.screen.insert(Location { x, y }, tile);
                }
            }
            self.print();
        }

        fn count_blocks(&self) -> i64 {
            let mut res = 0;
            for (_location, tile) in &self.screen {
                match tile {
                    Tile::Block => res += 1,
                    _ => {}
                }
            }
            res
        }

        fn print(&self) {
            println!("Score: {}", self.score);
            let mut current_y: Option<i64> = None;
            for (location, tile) in &self.screen {
                if Some(location.y) != current_y {
                    println!("");
                    current_y = Some(location.y)
                }
                match tile {
                    Tile::Empty => print!(" "),
                    Tile::Wall => print!("|"),
                    Tile::Block => print!("X"),
                    Tile::HorPaddle => print!("_"),
                    Tile::Ball => print!("o"),
                }
            }
        }
    }

    // Commenting this test because beating the game takes a really long time.
    // #[test]
    fn day_thirteen_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day13.txt");
        if let Ok(input) = readresult {
            let term = Term::stdout();
            let mut program = intcode::read_tokens(&input);
            program[0] = 2;
            let mut ctx = intcode::build_program_context(program, vec![]);
            let mut screen_printer = ScreenPrinter {
                output: ctx.outputs.clone(),
                cursor: 0,
                screen: BTreeMap::<Location, Tile>::new(),
                score: 0,
            };
            let mut prev_ctxs: VecDeque<intcode::ProgramContext> = VecDeque::new();
            let mut prev_screen_printers: VecDeque<ScreenPrinter> = VecDeque::new();
            let mut step = 0;
            prev_ctxs.push_back(ctx.clone());
            prev_screen_printers.push_back(screen_printer.clone());
            loop {
                ctx.run_to_next_input_or_done();
                screen_printer.output = ctx.outputs.clone();
                screen_printer.update_screen();
                match term.read_char() {
                    // left
                    Ok('j') => ctx.inputs.push(-1),
                    // neutral
                    Ok('k') => ctx.inputs.push(0),
                    //right
                    Ok('l') => ctx.inputs.push(1),
                    Ok('r') => {
                        ctx = prev_ctxs.pop_back().unwrap();
                        screen_printer = prev_screen_printers.pop_back().unwrap();
                    }
                    Ok('s') => {
                        prev_ctxs.push_back(ctx.clone());
                        prev_screen_printers.push_back(screen_printer.clone());
                        println!("GAME SAVED !!!")
                    }
                    Ok('q') => break,
                    Err(_e) => panic!("Error while reading keyboard input"),
                    _ => {}
                }
                step += 1;
            }
        }
    }

    #[test]
    fn day_thirteen_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day13.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let mut ctx = intcode::build_program_context(program, vec![]);
            ctx.run_to_next_input_or_done();
            let mut screen_printer = ScreenPrinter {
                output: ctx.outputs,
                cursor: 0,
                screen: BTreeMap::<Location, Tile>::new(),
                score: 0,
            };
            screen_printer.update_screen();
            println!(
                "THE ANSWER TO DAY THIRTEEN PART ONE IS {}",
                screen_printer.count_blocks()
            );
        }
    }
}
