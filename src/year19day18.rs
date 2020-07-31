mod year19day18 {
    use std::collections::{HashMap, HashSet, VecDeque};

    fn letters() -> HashSet<char> {
        vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ]
        .into_iter()
        .collect()
    }

    fn read_input(s: &str) -> Vec<Vec<char>> {
        let mut result = Vec::<Vec<char>>::new();
        for line in (*s).split("\n") {
            // let foo: Vec<char> = line.to_owned().chars().map(|c| c).collect();
            let line_chars: Vec<char> = line.to_owned().chars().collect();
            result.push(line_chars);
        }
        result
    }

    #[derive(Debug, PartialEq, Hash, Clone, Copy)]
    struct DistancedPosition {
        x: usize,
        y: usize,
        steps: i64,
    }
    impl Eq for DistancedPosition {}

    #[derive(Debug, PartialEq, Hash, Clone, Copy)]
    struct Position {
        x: usize,
        y: usize,
    }
    impl Eq for Position {}

    fn minimize(field: Vec<Vec<char>>, num_steps: i64) -> i64 {
        let distance_map = distance_to_letters(field.clone());
        if distance_map.len() == 0 {
            return num_steps;
        }

        let mut result = i64::max_value();
        for (key, distance) in &distance_map {
            let new_field = collect_key(field.clone(), *key);
            let candidate_value = minimize(new_field.clone(), num_steps + distance);
            println!("Collecting {}", *key);
            println!("Distance so far: {}", num_steps + distance);
            println!("Min steps: {}", candidate_value);
            for line in new_field {
                println!("{:?}", line);
            }
            if candidate_value < result {
                result = candidate_value;
            }
        }
        result
    }

    fn distance_to_letters(field: Vec<Vec<char>>) -> HashMap<char, i64> {
        let mut result = HashMap::<char, i64>::new();
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
                    });
                    seen_positions.insert(new_pos);
                }
                if let Some(letter) = letters().get(&c) {
                    result.insert(*letter, node.steps + 1);
                }
            }
        }

        result
    }

    fn collect_key(field: Vec<Vec<char>>, key: char) -> Vec<Vec<char>> {
        let mut result = Vec::<Vec<char>>::new();
        let upper_case_key = key.to_uppercase().next().unwrap();
        for (_j, _line) in field.iter().enumerate() {
            let mut line = Vec::<char>::new();
            for (_i, c) in _line.iter().enumerate() {
                if c == &'@' {
                    line.push('.');
                } else if c == &key {
                    line.push('@');
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
            let field = read_input(input);
            let new_field = collect_key(field, 'c');
        }
        #[test]
        fn day_eighteen_part_one_example() {
            let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
            let field = read_input(input);
            minimize(field, 0);
        }
    }
}
