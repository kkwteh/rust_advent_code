mod year19day18 {
    use lazy_static::lazy_static;
    use std::collections::{HashMap, HashSet, VecDeque};

    use std::sync::Mutex;

    lazy_static! {
        static ref COMPUTED_MINIMA: Mutex<HashMap<String, i64>> = Mutex::new(HashMap::new());
    }

    lazy_static! {
        static ref CURRENT_MINIMUM: Mutex<Vec<i64>> = Mutex::new(vec![999999999999]);
    }

    fn letters() -> HashSet<char> {
        vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ]
        .into_iter()
        .collect()
    }

    fn string_to_field(s: &str) -> Vec<Vec<char>> {
        let mut result = Vec::<Vec<char>>::new();
        for line in (*s).split("\n") {
            let line_chars: Vec<char> = line.to_owned().chars().collect();
            result.push(line_chars);
        }
        result
    }

    fn field_to_string(field: Vec<Vec<char>>) -> String {
        let mut field_as_string = "".to_owned();
        for line in field.clone() {
            let line_as_string: String = line.into_iter().collect();
            field_as_string.push_str(&line_as_string);
            field_as_string.push_str("\n");
        }
        field_as_string
    }

    #[derive(Debug, PartialEq, Hash, Clone, Copy)]
    struct DistancedPosition {
        x: usize,
        y: usize,
        steps: i64,
        robot: char,
    }
    impl Eq for DistancedPosition {}

    #[derive(Debug, PartialEq, Hash, Clone, Copy)]
    struct Position {
        x: usize,
        y: usize,
    }
    impl Eq for Position {}

    #[derive(Debug, PartialEq, Hash, Clone, Copy)]
    struct RobotKey {
        robot: char,
        key: char,
    }
    impl Eq for RobotKey {}

    fn minimize(input: String, steps_so_far: i64) -> i64 {
        if let Some(value) = COMPUTED_MINIMA.lock().unwrap().get(&input.clone()) {
            // println!("Already computed minima {}", value);
            // println!("{}", field_as_string.clone());
            return *value;
        }

        let field = string_to_field(&input[..]);
        let distance_map = distance_to_letters(field.clone());
        if distance_map.len() == 0 {
            COMPUTED_MINIMA.lock().unwrap().insert(input.clone(), 0);
            println!(
                "Cached solutions: {}",
                COMPUTED_MINIMA.lock().unwrap().len()
            );
            return 0;
        }
        let mut sorted_distances: Vec<(&RobotKey, &i64)> = distance_map.iter().collect();
        let mut result = 999999999999;
        sorted_distances.sort_by(|a, b| a.1.cmp(b.1));
        for (robot_key, distance) in sorted_distances.iter() {
            let new_field = collect_key(field.clone(), robot_key.key, robot_key.robot);
            let candidate_value = minimize(field_to_string(new_field), steps_so_far + **distance);
            if steps_so_far + **distance + candidate_value < CURRENT_MINIMUM.lock().unwrap()[0] {
                println!(
                    "Setting new minimum {} + {} + {} = {}",
                    steps_so_far,
                    **distance,
                    candidate_value,
                    steps_so_far + **distance + candidate_value
                );
                println!("Collecting {}", robot_key.key);
                for line in input.split("\n") {
                    println!("{}", line);
                }
                CURRENT_MINIMUM.lock().unwrap()[0] = steps_so_far + **distance + candidate_value;
            }
            if candidate_value + **distance < result {
                result = candidate_value + **distance;
            }
        }
        if result < 999999999999 {
            // println!("Caching minimum value {}", result);
            // println!("{}", field_as_string);
            COMPUTED_MINIMA
                .lock()
                .unwrap()
                .insert(input.clone(), result);
            println!(
                "Cached solutions: {}",
                COMPUTED_MINIMA.lock().unwrap().len()
            );
        }
        result
    }

    fn distance_to_letters(field: Vec<Vec<char>>) -> HashMap<RobotKey, i64> {
        let mut result = HashMap::<RobotKey, i64>::new();
        let mut seen_positions = HashSet::<Position>::new();
        let mut work_queue = VecDeque::<DistancedPosition>::new();
        let mut xpos = 0;
        let mut ypos = 0;
        for (j, _line) in field.iter().enumerate() {
            for (i, c) in _line.iter().enumerate() {
                if c == &'@' {
                    xpos = i;
                    ypos = j;
                }
            }
        }
        work_queue.push_back(DistancedPosition {
            x: xpos,
            y: ypos,
            steps: 0,
            robot: '@',
        });
        seen_positions.insert(Position { x: xpos, y: ypos });

        loop {
            if work_queue.len() == 0 {
                break;
            }
            let node = work_queue.pop_front().unwrap();
            let new_positions = vec![
                Position {
                    x: node.x - 1,
                    y: node.y,
                },
                Position {
                    x: node.x + 1,
                    y: node.y,
                },
                Position {
                    x: node.x,
                    y: node.y - 1,
                },
                Position {
                    x: node.x,
                    y: node.y + 1,
                },
            ];
            for new_pos in new_positions {
                let c = field[new_pos.y][new_pos.x];
                if c == '.' && seen_positions.get(&new_pos) == None {
                    work_queue.push_back(DistancedPosition {
                        x: new_pos.x,
                        y: new_pos.y,
                        steps: node.steps + 1,
                        robot: '@',
                    });
                    seen_positions.insert(new_pos);
                }
                if let Some(letter) = letters().get(&c) {
                    result.insert(
                        RobotKey {
                            robot: '@',
                            key: *letter,
                        },
                        node.steps + 1,
                    );
                }
            }
        }

        result
    }

    fn collect_key(field: Vec<Vec<char>>, key: char, robot: char) -> Vec<Vec<char>> {
        let mut result = Vec::<Vec<char>>::new();
        let upper_case_key = key.to_uppercase().next().unwrap();
        for (_j, _line) in field.iter().enumerate() {
            let mut line = Vec::<char>::new();
            for (_i, c) in _line.iter().enumerate() {
                if c == &robot {
                    line.push('.');
                } else if c == &key {
                    line.push(robot);
                } else if c == &upper_case_key {
                    line.push('.');
                } else {
                    line.push(*c);
                }
            }
            result.push(line);
        }
        result
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_collect_key() {
            let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
            let field = string_to_field(input);
            let new_field = collect_key(field, 'c', '@');
        }
        #[test]
        fn day_eighteen_part_one_example() {
            let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
            assert_eq!(86, minimize(input.to_owned(), 0));
        }

        #[test]
        fn day_eighteen_part_one_second_example() {
            let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
            let field = string_to_field(input);
            assert_eq!(81, minimize(input.to_owned(), 0));
        }

        #[test]
        fn day_eighteen_part_one_third_example() {
            let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
            assert_eq!(136, minimize(input.to_owned(), 0));
        }

        #[test]
        fn day_eighteen_part_one_fourth_example() {
            let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
            assert_eq!(132, minimize(input.to_owned(), 0));
        }

        #[test]
        fn day_eighteen_part_one_challenge() {
            let input =
                "#################################################################################
#.........#.............#.......#.....#.#.....................#...#..c#.........#
#.###.#####.#######.#####.###.#.#.###.#.#.#######.###########.#.#.###.#.#####.#.#
#.#.#.#e..#...#.#...#.....#.#.#...#.....#..b#...#.#.........#...#.....#.#.T..r#.#
#.#.#.#.#.###.#.#.#.#.#####.#.#############.#.###.#.#####.#.#.#######.#.#.#######
#.#...#.#...#...#.#.#.#.....#.........K.#.#.#...#.#.#...#.#...#...#...#.#.......#
#.###.#.###.#.###.#.#.#.###.###########.#.#.###.#.###.#.#.#####.#.#.###.#######.#
#...#.#...#...#...#.#...#.#...#.....#...#.#.#...#.....#.#.#.....#.#...#.#.#.....#
###.#.###.#####.#########.###.#.###.#.#.#.#.#.#.#######.#.#O#####.#####.#.#.###.#
#...#.#.#...#.#.............#.#.#...#.#.#...#x#.#.....#.#.#...#z#.....#.#.#.#...#
#.###.#R###.#.#####.#.#######.#.#.###.#.#.###.#.#.###.#.#.###.#.#####.#.#.#.#####
#.#...#...#.#.....#.#.#.....#.#.#.#...#.#...#.#...#...#.#.#.......#...#...#.....#
#.#.#####.#.###.#.#.#.#.###.#.#.#.#.#######.#.#####L###.#.#.#####.#.#####.#####.#
#.#.......#...#.#.#.#.#...#...#.#.#...Y.#.#.#...#.....#.#.#.#...#.#.......#....q#
#.###########.#.#.#.#####.#######.#####.#.#.###.#######.###.#.#.#.#########.###.#
#...#.......#.#.#.#...#...#..h..#.....#.#.......#.....#...#.#.#.#.#.......#.#...#
#.#.#.#####.#.#.#.###.#.###.###.###.###.#######.#.###.###.#.#.#.#.#.#######.#.###
#.#.....#...#.#.#...#.#.#...#.....#.#...#...#...#...#...#...#.#.#.#.#.......#.#.#
#########.###.#####.#.#.#F#.#####.#.#.###.#.###.###.###.#.###.#.#.#.#.#######.#.#
#.........#...#.....#...#.#.#...#...#.#s#.#...#.#...#...#.#...#.#.#.#.#...#...#.#
#.#####.###.###.#.#####.#.###.#.###.#.#.#.###.###.###.#####.###.#.#.#.#.###.###.#
#..d#...#...#...#.#...#...#...#.#...#.#.#.#.#.....#.#.......#...#...#.#...#.#...#
#.#.#####.###.#.###.#######.###.#####.#.#.#.#######.#############.###.#.#.#.###.#
#.#.....#...#.#...#.........#.#.......#.#.......#.....#.....#.....#...#.#.#.....#
#.#####.###.#####.###########.#########.#######.#.###.###.#.#####.#.#####.#####.#
#.#...#...#...#...#.......#...........#.#.....#...#...#...#.....#.#...#.....#...#
#.#.###.#.###.#.#.#.#####.###.#######.#.#.###.#####.###.#######.#####.#.#####.###
#.#.#...#.#.#.#.#...#.....#...#.......#.#.#.#.....#...#.#.....#.#.....#.........#
#.#.#.###.#.#.#.#####.#####.#####.#####.#.#.###.#.###.#.#.#.###.#.#.#######.#####
#...#.#.#...#.#...#.#.#...#.....#.#.....#.....#.#...#...#.#.#...#.#j#.....#.#.W.#
#####.#.###.#.###.#.#.#.#.#####.#.###.#.#####.#.#########.###.###.###.###.###.#.#
#...#.#.....#.#...#.#.#.#.......#.....#.#.#...#.........#.....#.....#...#...#.#.#
#.#.#.#####.#.#.###.#.#.###############.#.#.#########.###.#########.#.#####.#.#.#
#.#...#...#.#.#w..#...#...#...........#.#.#.#.......#...#...#.....#...#.V.#...#.#
#.#####.#.###.###.#.###.#.#.###########.#.#.#####.#.###.###.###.#M#####.#.#####.#
#.....#.#...#.....#.#.#.#.#.A.#...#.....#.#.......#...#...#...#.#.......#.......#
#####.#.###.#######.#.#.#.#.#.#.#.#.#####.###########.###.###.#.###############.#
#.....#.#.#.......#.#...#.#.#...#.#.....#...#.......#.#.#.#...#.....#.#.....#...#
#.#####.#.#######.#.#####.#.#####.#####.#.#.#.###.###.#.#.#.#######.#.#.###.#.###
#............v..#.........#.....#.........#.....#.......#...........#.....#.....#
#######################################.@.#######################################
#.#.....G.....#.....#.......#.........#.........#.......#...........#.......#...#
#.#.###.#####.#####.#.#####.#.###.###.#.#.#####.#####.#.#######.###.#.###.#.###.#
#.#.#.#.....#.......#.#...#...#.#.#.....#.#...#......a#.......#...#.#.#...#.#...#
#.#.#.#####.#######.#.#.#.#####.#.#####.#.#.#.###############.###.#.#.#.###.#.#.#
#.#..n....#.#.......#.#.#...#...#.....#.#.#.#...#.....#.....#...#.#...#...#.#.#.#
#.#######.#.#########.#.###.#.#######.#.#.#.#####.#.###.#.#.###.#########P#.#.###
#...#.#...#...#u..#...#.#...#.......#.#.#.#...#...#.....#.#...#.....#...#.#.#...#
#.#.#.#.#####.#.#.#.###.###.###.###.#.#.#.###.#.#########.###.#####.#.#U#.#.###.#
#.#...#p..#.#...#...#.....#.....#...#.#.#...#.#.#.....#...#...#...#...#.#.#...#.#
#.###.###.#.###########.#.###########.#####.#.#.###.#.#.###.###.#.#####.#.###.#.#
#...#.#...#...#.......#.#.#..k..#...#...#...#.#.#...#.#.#...#...#...#...#...#.#.#
#.#.###.###.#.###.###.###.#.###.#.#.###.#.###.#.#.###C#Z###.#.#####.#.###.#.#.#.#
#.#.........#.....#.#.#...#.#...#.#...#.#...#.#.#.#...#...#.#.....#.#...#.#.#.#.#
#.#################.#.#.###.#.###.###.#.#.#.#.#.#.#.#####.#.###.###.###.###.#.#.#
#m....#...........#.#...#...#...#...#.#.#.#.#.#.#.#.#.....#...#.#...#..g#.N.#...#
#.#####.#########.#.###.#.#####.###.#.#.###.#.#.#B#.#.#########.#.###.###.#####.#
#.#.....#.....#...#.....#.#...#.#...#...#...#.....#.#.......#...#...#...#.#i..#.#
###.#######.###.#########.#.###.#.#######.#########.#######.#.#####.###.#.#.#.#.#
#...#.......#...#....y....#...#.#.#.....#.#...S.#...#.....#.......#.#...#.#.#.#.#
#.###.#####.#.#.#.#########.#.#.#.#.###.#.#.###.#.###.###.#####.###.#.###.#.#.#.#
#...#.#.#...#.#.#.....#...#.#...#.....#.#.....#.#...#.#.....#.#.#...#.....#.#.#.#
###.#.#.#.###.#########.#.#.#####.#####.#######.###X#.#######.###.#.#######.###.#
#...#.#.#...#...........#.#...#...#.....#...#...#...#.#.......#...#.....#.......#
#Q###.#.###.#####.###########.#.###.###.#.#.#.#####.#.#.#######.#########.#######
#.....#...#.#...#.........#...#.#...#.#.#.#.#.....#...#...#.....#.......#.......#
#######.###.#.###########.#.#####.###.#.#.#.#####.###.###.#####.#D#####.#######.#
#.....#...#.#.......#...#...#.....#.....#.#.#...#.#....l#...#...#.#...#.......#.#
###.#.###.#.###.###.###.#####.#.#########.#.#.###.#########.#.#.#.###.#######.#.#
#...#...#.#.....#...#.......#.#.........#.#.#.#...#...#...#.#.#.#...#.......#...#
#.#####.#.#######.###.#######.#########.#.#.#.#.###.#.#.#.#.#.#.###.#.#####.#####
#.#..t#.#.....#...#.............#.....#.#.#.#.#.....#...#...#.#.#...#.#...#.....#
#.###.#.#.#####.###.###########.#.#.###.#.#.#.###############.#.#.#####.#.#####.#
#...#.....#...#.#.....#.......#...#.#...#.#.........#.........#.#.......#.#.....#
#E#.#.#####.#.#.#####.#.#####.#####.#.###.###.#######.#####.#############.#.#####
#.#.#...#...#.#...#...#.#...#...#...#...#...#.#.....#.#...#.........#.....#.#...#
###.#####.###I###.#.###.#.#.###.#####.#.#.#.###.###.#.#.#######.#####.#####.#.#.#
#...#...#...#...#.#.#...#.#...#.#...#.#.#.#..f..#.#...#.....#...#.....#...#...#.#
#.###.#.###.#.###.###.###.#####.#.#.###.#.#######.#########.#.###.#####.#.#####.#
#.J...#.....#.........#...........#.....#.............H....o#...........#.......#
#################################################################################";
            println!(
                "THE ANSWER TO DAY EIGHTEEN PART ONE IS {}",
                minimize(input.to_owned(), 0)
            );
        }
    }
}
