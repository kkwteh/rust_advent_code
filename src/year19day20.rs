mod year19day20 {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::iter::FromIterator;
    #[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
    struct DistancedPoint {
        point: Point,
        steps: i64,
    }

    #[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
    struct LeveledPoint {
        point: Point,
        level: i64,
    }

    #[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
    struct DistancedLeveledPoint {
        leveled_point: LeveledPoint,
        steps: i64,
    }

    #[derive(Debug, Clone)]
    struct MapData {
        field: Vec<Vec<char>>,
        portals: HashMap<Point, Point>,
        entrance: Option<Point>,
        exit: Option<Point>,
    }

    fn letters() -> HashSet<char> {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().into_iter().collect()
    }

    fn string_to_field(s: &str) -> Vec<Vec<char>> {
        let mut result = Vec::<Vec<char>>::new();
        for line in (*s).split("\n") {
            let line_chars: Vec<char> = line.to_owned().chars().collect();
            result.push(line_chars);
        }
        result
    }

    fn extract_map_data(field: Vec<Vec<char>>) -> MapData {
        // Scan input for '.'
        // For each '.' probe around for (unique) adjacent letter, there will be at most one.
        // If adjacent letter found, probe around the letter for adjacent letter. There will be exactly one.
        // Sort characters, add portal point to HashMap mapping portal id to length-2 array of portal coordinates
        let mut entrance: Option<Point> = None;
        let mut exit: Option<Point> = None;
        let mut portal_points = HashMap::<String, [Point; 2]>::new();
        for (j, line) in field.iter().enumerate() {
            for (i, c) in line.iter().enumerate() {
                if c == &'.' {
                    for adjacent in vec![
                        Point { x: i - 1, y: j },
                        Point { x: i + 1, y: j },
                        Point { x: i, y: j - 1 },
                        Point { x: i, y: j + 1 },
                    ] {
                        if let Some(letter1) = letters().get(&field[adjacent.y][adjacent.x]) {
                            for adadjacent in vec![
                                Point {
                                    x: adjacent.x - 1,
                                    y: adjacent.y,
                                },
                                Point {
                                    x: adjacent.x + 1,
                                    y: adjacent.y,
                                },
                                Point {
                                    x: adjacent.x,
                                    y: adjacent.y - 1,
                                },
                                Point {
                                    x: adjacent.x,
                                    y: adjacent.y + 1,
                                },
                            ] {
                                if let Some(letter2) =
                                    letters().get(&field[adadjacent.y][adadjacent.x])
                                {
                                    if letter1 == &'A' && letter2 == &'A' {
                                        entrance = Some(Point { x: i, y: j });
                                        continue;
                                    }

                                    if letter1 == &'Z' && letter2 == &'Z' {
                                        exit = Some(Point { x: i, y: j });
                                        continue;
                                    }

                                    let mut portal_chars = vec![letter1, letter2];
                                    portal_chars.sort_by(|a, b| a.cmp(b));
                                    let portal_id = String::from_iter(portal_chars);
                                    match portal_points.get_mut(&portal_id) {
                                        Some(point_array) => point_array[1] = Point { x: i, y: j },
                                        None => {
                                            let array: [Point; 2] =
                                                [Point { x: i, y: j }, Point { x: 0, y: 0 }];
                                            portal_points.insert(portal_id, array);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut portals = HashMap::<Point, Point>::new();
        for pair_array in portal_points.values() {
            portals.insert(pair_array[0].clone(), pair_array[1].clone());
            portals.insert(pair_array[1].clone(), pair_array[0].clone());
        }
        MapData {
            field: field.clone(),
            portals: portals,
            entrance: entrance,
            exit: exit,
        }
    }

    fn compute_naive_distance(map_data: MapData) -> i64 {
        let mut work_queue = VecDeque::<DistancedPoint>::new();
        let mut seen_points = HashSet::<Point>::new();
        let exit = map_data.exit.unwrap();

        work_queue.push_back(DistancedPoint {
            point: map_data.entrance.clone().unwrap(),
            steps: 0,
        });
        seen_points.insert(map_data.entrance.clone().unwrap());
        loop {
            if let Some(node) = work_queue.pop_front() {
                let mut adjacent_points = vec![
                    Point {
                        x: node.point.x - 1,
                        y: node.point.y,
                    },
                    Point {
                        x: node.point.x + 1,
                        y: node.point.y,
                    },
                    Point {
                        x: node.point.x,
                        y: node.point.y - 1,
                    },
                    Point {
                        x: node.point.x,
                        y: node.point.y + 1,
                    },
                ];
                if let Some(destination) = map_data.portals.get(&node.point) {
                    adjacent_points.push(destination.clone());
                }
                for point in adjacent_points {
                    if point == exit {
                        return node.steps + 1;
                    }
                    if map_data.field[point.y][point.x] == '.' {
                        if seen_points.get(&point) == None {
                            seen_points.insert(point);
                            work_queue.push_back(DistancedPoint {
                                point: point.clone(),
                                steps: node.steps + 1,
                            });
                        }
                    }
                }
            }
        }
    }

    fn is_outer(point: &Point, maze_width: usize, maze_height: usize) -> bool {
        point.x == 2 || point.y == 2 || point.x == maze_width - 3 || point.y == maze_height - 3
    }

    fn compute_distance(map_data: MapData) -> i64 {
        let mut work_queue = VecDeque::<DistancedLeveledPoint>::new();
        let mut seen_points = HashSet::<LeveledPoint>::new();
        let maze_height = map_data.field.len();
        // Take the third row because to avoid leading new lines
        let maze_width = map_data.field[2].len();
        let exit = map_data.exit.unwrap();

        work_queue.push_back(DistancedLeveledPoint {
            leveled_point: LeveledPoint {
                point: map_data.entrance.clone().unwrap(),
                level: 0,
            },
            steps: 0,
        });
        seen_points.insert(LeveledPoint {
            point: map_data.entrance.clone().unwrap(),
            level: 0,
        });
        loop {
            if let Some(node) = work_queue.pop_front() {
                let mut adjacent_points = vec![
                    LeveledPoint {
                        point: Point {
                            x: node.leveled_point.point.x - 1,
                            y: node.leveled_point.point.y,
                        },
                        level: node.leveled_point.level,
                    },
                    LeveledPoint {
                        point: Point {
                            x: node.leveled_point.point.x + 1,
                            y: node.leveled_point.point.y,
                        },
                        level: node.leveled_point.level,
                    },
                    LeveledPoint {
                        point: Point {
                            x: node.leveled_point.point.x,
                            y: node.leveled_point.point.y - 1,
                        },
                        level: node.leveled_point.level,
                    },
                    LeveledPoint {
                        point: Point {
                            x: node.leveled_point.point.x,
                            y: node.leveled_point.point.y + 1,
                        },
                        level: node.leveled_point.level,
                    },
                ];
                if let Some(destination) = map_data.portals.get(&node.leveled_point.point) {
                    if is_outer(&node.leveled_point.point, maze_width, maze_height)
                        && node.leveled_point.level > 0
                    {
                        println!("Outer portal at {:?}", node.leveled_point);
                        adjacent_points.push(LeveledPoint {
                            point: destination.clone(),
                            level: node.leveled_point.level - 1,
                        });
                    }
                    if !is_outer(&node.leveled_point.point, maze_width, maze_height) {
                        println!("Inner portal at {:?}", node.leveled_point);
                        adjacent_points.push(LeveledPoint {
                            point: destination.clone(),
                            level: node.leveled_point.level + 1,
                        });
                    }
                }
                for leveled_point in adjacent_points {
                    if leveled_point.point == exit && leveled_point.level == 0 {
                        return node.steps + 1;
                    }
                    if map_data.field[leveled_point.point.y][leveled_point.point.x] == '.' {
                        if seen_points.get(&leveled_point) == None {
                            seen_points.insert(leveled_point);
                            work_queue.push_back(DistancedLeveledPoint {
                                leveled_point: leveled_point.clone(),
                                steps: node.steps + 1,
                            });
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn day_twenty_part_one_example() {
        let input = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";
        let field = string_to_field(input);
        let map_data = extract_map_data(field.clone());
        assert_eq!(23, compute_naive_distance(map_data));
    }

    #[test]
    fn day_twenty_part_one_example_two() {
        let input = "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P   
";
        let field = string_to_field(input);
        let map_data = extract_map_data(field.clone());
        assert_eq!(58, compute_naive_distance(map_data));
    }

    #[test]
    fn day_twenty_part_one_challenge() {
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/year19day20.txt");
        if let Ok(input) = readresult {
            let field = string_to_field(&input[..]);
            let map_data = extract_map_data(field.clone());
            let result = compute_naive_distance(map_data);
            println!("THE ANSWER TO DAY TWENTY PART ONE IS {}", result);
        };
    }

    #[test]
    fn day_twenty_part_two_challenge() {
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/year19day20.txt");
        if let Ok(input) = readresult {
            let field = string_to_field(&input[..]);
            let map_data = extract_map_data(field.clone());
            let result = compute_distance(map_data);
            println!("THE ANSWER TO DAY TWENTY PART TWO IS {}", result);
        };
    }

    #[test]
    fn day_twenty_part_two_example() {
        let input = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
        let field = string_to_field(input);
        let map_data = extract_map_data(field.clone());
        let result = compute_distance(map_data);
        assert_eq!(396, result);
    }

    fn sort_string(input: String) -> String {
        let s_slice: &str = &input[..];
        let mut chars: Vec<char> = s_slice.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        String::from_iter(chars)
    }

    #[test]
    fn test_sort_string() {
        let wordy = "I am a hello world example".to_owned();
        println!("sorting '{}' gives '{}'", wordy.clone(), sort_string(wordy));
    }
}
