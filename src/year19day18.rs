mod year19day18 {
    use std::collections::{HashMap, HashSet, VecDeque};
    fn read_input(s: &str) -> Vec<Vec<char>> {
        let mut result = Vec::<Vec<char>>::new();
        for line in (*s).split("\n") {
            // let foo: Vec<char> = line.to_owned().chars().map(|c| c).collect();
            let line_chars: Vec<char> = line.to_owned().chars().collect();
            println!("{:?}", line_chars);
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

    fn distance_to_letters(field: Vec<Vec<char>>) -> HashMap<char, i64> {
        let mut result = HashMap::<char, i64>::new();
        let mut seen_positions = HashSet::<Position>::new();
        let mut work_queue = VecDeque::<DistancedPosition>::new();
        let mut letters: HashSet<char> = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ]
        .into_iter()
        .collect();
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
                if let Some(letter) = letters.get(&c) {
                    result.insert(*letter, node.steps + 1);
                }
            }
        }

        println!("Distance to letters: {:?}", result);
        result
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn day_eighteen_part_one_example() {
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
            distance_to_letters(field);
        }
    }
}
