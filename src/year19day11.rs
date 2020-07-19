#[cfg(test)]
mod year19day11 {
    use crate::intcode::intcode;
    use std::cmp::Ordering;
    use std::collections::BTreeMap;
    use std::fs;

    #[derive(Copy, Clone, Debug)]
    enum Color {
        Black,
        White,
    }

    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    struct Robot {
        location: Location,
        direction: Direction,
    }

    impl Robot {
        fn turn_left(&mut self) {
            match self.direction {
                Direction::Up => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Down,
                Direction::Down => self.direction = Direction::Right,
                Direction::Right => self.direction = Direction::Up,
            }
        }

        fn turn_right(&mut self) {
            match self.direction {
                Direction::Up => self.direction = Direction::Right,
                Direction::Right => self.direction = Direction::Down,
                Direction::Down => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Up,
            }
        }

        fn move_forward(&mut self) {
            match self.direction {
                Direction::Up => self.location.y += 1,
                Direction::Right => self.location.x += 1,
                Direction::Down => self.location.y -= 1,
                Direction::Left => self.location.x -= 1,
            }
        }
    }

    #[derive(Hash, PartialEq, Clone, Debug)]
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
                other.y.cmp(&self.y)
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

    #[test]
    fn test_location_ord() {
        assert_eq!(Location { x: 1, y: 1 } < Location { x: 2, y: 1 }, true);
        assert_eq!(Location { x: 2, y: 2 } < Location { x: 1, y: 1 }, true);
    }

    fn run_paint_program(
        program: Vec<String>,
        robot: &mut Robot,
        // BTreeMap wasn't actually helpful because of gaps in the paint.
        paint: &mut BTreeMap<Location, Color>,
    ) {
        let mut ctx = intcode::build_program_context(program, vec![]);
        let mut color: Color;
        const BLACK_STRING: &str = "0";
        const WHITE_STRING: &str = "1";
        const LEFT_STRING: &str = "0";
        const RIGHT_STRING: &str = "1";
        loop {
            match paint.get(&robot.location) {
                None => color = Color::Black,
                Some(value) => color = *value,
            }
            match color {
                Color::Black => ctx.inputs.push((*BLACK_STRING).to_string()),
                Color::White => ctx.inputs.push((*WHITE_STRING).to_string()),
            }
            ctx.run_to_next_input_or_done();
            let paint_instruction = &ctx.outputs[ctx.outputs.len() - 2][..];
            let move_instruction = &ctx.outputs[ctx.outputs.len() - 1][..];
            match paint_instruction {
                BLACK_STRING => {
                    paint.insert(robot.location.clone(), Color::Black);
                }
                WHITE_STRING => {
                    paint.insert(robot.location.clone(), Color::White);
                }
                _ => panic!("Unexepcted paint instruction {}", paint_instruction),
            };
            match move_instruction {
                LEFT_STRING => robot.turn_left(),
                RIGHT_STRING => robot.turn_right(),
                _ => panic!("Unexepcted move instruction {}", move_instruction),
            };
            robot.move_forward();
            if ctx.done {
                break;
            }
        }
    }

    #[test]
    fn day_eleven_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day11.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let mut paint = BTreeMap::<Location, Color>::new();
            let mut robot = Robot {
                location: Location { x: 0, y: 0 },
                direction: Direction::Up,
            };
            run_paint_program(program, &mut robot, &mut paint);

            println!("THE ANSWER TO DAY 11 PART ONE IS {}", paint.len());
        }
    }

    #[test]
    fn day_eleven_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day11.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let mut paint = BTreeMap::<Location, Color>::new();
            paint.insert(Location { x: 0, y: 0 }, Color::White);
            let mut robot = Robot {
                location: Location { x: 0, y: 0 },
                direction: Direction::Up,
            };
            run_paint_program(program, &mut robot, &mut paint);

            let mut minx = 0;
            let mut maxx = 0;
            let mut miny = 0;
            let mut maxy = 0;
            for location in paint.keys() {
                if location.x < minx {
                    minx = location.x;
                }
                if location.x > maxx {
                    maxx = location.x;
                }
                if location.y < miny {
                    miny = location.y;
                }
                if location.y > maxy {
                    maxy = location.y;
                }
            }

            for j in (miny..=maxy).rev() {
                println!("");
                for i in minx..=maxx {
                    match paint.get(&Location { x: i, y: j }) {
                        None => print!(" "),
                        Some(Color::Black) => print!(" "),
                        Some(Color::White) => print!("X"),
                    }
                }
            }
        }
    }
}
