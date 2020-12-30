mod y20d03 {

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Field {
        tiles: Vec<Vec<Tile>>,
        width: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Slope {
        dx: usize,
        dy: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Continuation {
        Inputs(Field),
        TreeCounts(Vec<i64>),
        Done(i64),
    }
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Tile {
        Tree,
        Empty,
    }

    fn parse_text() -> Continuation {
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/y20d03.txt");
        if let Ok(input) = readresult {
            let mut tiles: Vec<Vec<Tile>> = vec![];
            let mut width: usize = 0;
            for line in (*input).split("\n") {
                let mut tree_line = vec![];
                for char in line.chars() {
                    match char {
                        '.' => tree_line.push(Tile::Empty),
                        '#' => tree_line.push(Tile::Tree),
                        _ => panic!("Unexpected char found"),
                    }
                }
                width = tree_line.len();
                tiles.push(tree_line);
            }
            return Continuation::Inputs(Field {
                tiles: tiles,
                width: width,
            });
        } else {
            panic!("Could not read file")
        }
    }

    fn count_trees(slopes: Vec<Slope>, k: Continuation) -> Continuation {
        let field = match k {
            Continuation::Inputs(k_field) => k_field,
            _ => panic!("No field given"),
        };
        let mut results: Vec<i64> = vec![];
        for slope in slopes {
            let mut ix: usize = 0;
            let mut tree_count = 0;
            loop {
                let x = ix * slope.dx % field.width;
                let y = ix * slope.dy;
                if y >= field.tiles.len() {
                    break;
                }
                match field.tiles[y][x] {
                    Tile::Tree => {
                        tree_count += 1;
                    }
                    Tile::Empty => {}
                }
                ix += 1;
            }
            results.push(tree_count);
        }

        return Continuation::TreeCounts(results);
    }

    fn multiply_counts(k: Continuation) -> Continuation {
        let counts = match k {
            Continuation::TreeCounts(k_counts) => k_counts,
            _ => panic!("No counts given"),
        };
        let mut answer = 1;
        for count in counts {
            answer *= count;
        }
        return Continuation::Done(answer);
    }

    #[test]
    fn y20d03ch01() {
        let k = parse_text();
        let k = count_trees(vec![Slope { dx: 3, dy: 1 }], k);
        let k = multiply_counts(k);
        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 3 CHALLENGE 1 --------------------> {:?}",
            answer,
        );
    }

    #[test]
    fn y20d03ch02() {
        let k = parse_text();
        let k = count_trees(
            vec![
                Slope { dx: 1, dy: 1 },
                Slope { dx: 3, dy: 1 },
                Slope { dx: 5, dy: 1 },
                Slope { dx: 7, dy: 1 },
                Slope { dx: 1, dy: 2 },
            ],
            k,
        );
        let k = multiply_counts(k);
        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 3 CHALLENGE 2 --------------------> {:?}",
            answer,
        );
    }
}
