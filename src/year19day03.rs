#[cfg(test)]
mod year19day03 {
    use std::collections::HashMap;
    #[cfg(test)]
    use std::fs;

    #[derive(PartialEq, Eq, Debug, Hash)]
    struct Point {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    struct Segment {
        distance: i64,
        direction: String,
    }

    #[cfg(test)]
    fn populate_points(
        mut points_traversed: HashMap<Point, i64>,
        current: &Point,
        steps: i64,
        dx: i64,
        dy: i64,
        distance: i64,
    ) -> HashMap<Point, i64> {
        for i in 1..(distance + 1) {
            let new_point = Point {
                x: current.x + i * dx,
                y: current.y + i * dy,
            };
            if !points_traversed.contains_key(&new_point) {
                points_traversed.insert(
                    Point {
                        x: current.x + i * dx,
                        y: current.y + i * dy,
                    },
                    steps + i,
                );
            }
        }
        points_traversed
    }

    #[cfg(test)]
    fn update_current(mut current: Point, dx: i64, dy: i64, distance: i64) -> Point {
        current.x += distance * dx;
        current.y += distance * dy;
        current
    }

    #[cfg(test)]
    fn find_min_cross_point_dist(patha: &str, pathb: &str) -> i64 {
        let points_a = extract_points(patha);
        let points_b = extract_points(pathb);
        let mut min = i64::max_value();
        for point in points_a.keys() {
            if points_b.contains_key(point) {
                let dist = point.x.abs() + point.y.abs();
                if 0 < dist && dist < min {
                    min = dist;
                }
            }
        }
        min
    }

    #[cfg(test)]
    fn find_min_walk_dist(patha: &str, pathb: &str) -> i64 {
        let points_a = extract_points(patha);
        let points_b = extract_points(pathb);
        let mut min = i64::max_value();
        for point in points_a.keys() {
            if points_b.contains_key(point) {
                let dist = points_a[point] + points_b[point];
                if 0 < dist && dist < min {
                    min = dist;
                }
            }
        }
        min
    }

    #[cfg(test)]
    fn extract_points(path: &str) -> HashMap<Point, i64> {
        let segments = parse_segments(path);
        let mut points_traversed = HashMap::new();
        let mut current = Point { x: 0, y: 0 };
        let mut steps = 0;
        for segment in segments {
            if segment.direction == "U" {
                let dx = 0;
                let dy = 1;
                points_traversed =
                    populate_points(points_traversed, &current, steps, dx, dy, segment.distance);
                current = update_current(current, dx, dy, segment.distance);
                steps += segment.distance;
            } else if segment.direction == "D" {
                let dx = 0;
                let dy = -1;
                points_traversed =
                    populate_points(points_traversed, &current, steps, dx, dy, segment.distance);
                current = update_current(current, dx, dy, segment.distance);
                steps += segment.distance;
            } else if segment.direction == "L" {
                let dx = -1;
                let dy = 0;
                points_traversed =
                    populate_points(points_traversed, &current, steps, dx, dy, segment.distance);
                current = update_current(current, dx, dy, segment.distance);
                steps += segment.distance;
            } else if segment.direction == "R" {
                let dx = 1;
                let dy = 0;
                points_traversed =
                    populate_points(points_traversed, &current, steps, dx, dy, segment.distance);
                current = update_current(current, dx, dy, segment.distance);
                steps += segment.distance;
            } else {
                panic!("Unexpected segment direction {}", segment.direction)
            }
        }

        points_traversed
    }

    #[cfg(test)]
    fn parse_segments(input: &str) -> Vec<Segment> {
        let mut segments = Vec::new();

        for token in (*input).split(",") {
            let segment = Segment {
                distance: token[1..token.len()].to_string().parse::<i64>().unwrap(),
                direction: token.chars().nth(0).unwrap().to_string(),
            };
            segments.push(segment);
        }
        segments
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn day_three_part_one_examples() {
            assert_eq!(find_min_cross_point_dist(&"R8,U5,L5,D3", &"U7,R6,D4,L4"), 6);
            assert_eq!(
                find_min_cross_point_dist(
                    &"R75,D30,R83,U83,L12,D49,R71,U7,L72",
                    &"U62,R66,U55,R34,D71,R55,D58,R83"
                ),
                159
            );
            assert_eq!(
                find_min_cross_point_dist(
                    &"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                    &"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                ),
                135
            );
        }

        #[test]
        fn test_points_traversed() {
            let points_traversed = extract_points(&"U2");
            println!("{:?}", points_traversed)
        }

        #[test]
        fn test_parse_segments() {
            let result = parse_segments(&"U2,L2,R2,D2");
            assert_eq!(result[0].distance, 2);
            assert_eq!(result[2].direction, "R".to_string());
        }

        #[test]
        fn day_three_part_one_challenge() {
            let readresult = fs::read_to_string("adventinputs/year19day03.txt");
            match readresult {
                Ok(input) => {
                    let paths: Vec<&str> = (*input).split("\n").collect();
                    let result = find_min_cross_point_dist(paths[0], paths[1]);
                    println!("THE ANSWER TO DAY THREE PART ONE IS {}", result);
                }
                Err(_e) => panic!("COULD NOT OPEN FILE"),
            }
        }

        #[test]
        fn day_three_part_two_examples() {
            assert_eq!(find_min_walk_dist(&"R8,U5,L5,D3", &"U7,R6,D4,L4"), 30);
            assert_eq!(
                find_min_walk_dist(
                    &"R75,D30,R83,U83,L12,D49,R71,U7,L72",
                    &"U62,R66,U55,R34,D71,R55,D58,R83"
                ),
                610
            );
            assert_eq!(
                find_min_walk_dist(
                    &"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                    &"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                ),
                410
            );
        }

        #[test]
        fn day_three_part_two_challenge() {
            let readresult = fs::read_to_string("adventinputs/year19day03.txt");
            match readresult {
                Ok(input) => {
                    let paths: Vec<&str> = (*input).split("\n").collect();
                    let result = find_min_walk_dist(paths[0], paths[1]);
                    println!("THE ANSWER TO DAY THREE PART TWO IS {}", result);
                }
                Err(_e) => panic!("COULD NOT OPEN FILE"),
            }
        }

        #[test]
        fn day_two_part_two_challenge() {}

        fn incr_point(mut current: Point) -> Point {
            current.x += 1;
            current.y += 1;
            current
        }
        #[test]
        fn mutate_struct() {
            let mut current = Point { x: 0, y: 0 };
            current = incr_point(current);
            assert_eq!(current.x, 1);
        }

        #[test]
        fn max_int() {
            let min = u32::max_value();
            println!("{}", min)
        }
    }
}
