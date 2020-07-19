#[cfg(test)]
mod year19day10 {
    use fraction::GenericFraction::{self, Infinity, NaN, Rational};
    use fraction::Sign;
    use std::cmp::Ord;
    use std::cmp::Ordering::{self, Equal};
    use std::collections::{BTreeMap, HashSet};
    use std::f64::consts::{FRAC_PI_2, PI};
    use std::fs;

    type F = GenericFraction<i64>;

    // for the special case of a vertical line
    // Right will mean Up and Left will mean Down
    #[derive(PartialEq, Hash, Debug)]
    enum Direction {
        Left,
        Right,
    }

    impl Eq for Direction {}

    #[derive(PartialEq, Hash, Debug)]
    struct SightLine {
        slope: GenericFraction<i64>,
        direction: Direction,
    }

    impl SightLine {
        fn get_angle(&self) -> f64 {
            // Higher angles means that asteroid gets shot first
            let atan_slope: f64;
            match self.slope {
                Rational(sign, ratio) => {
                    let float_sign: f64;
                    match sign {
                        Sign::Plus => float_sign = 1.0,
                        Sign::Minus => float_sign = -1.0,
                    }
                    let float_fraction: f64 = *ratio.numer() as f64 / *ratio.denom() as f64;
                    atan_slope = -1.0 * (float_sign * float_fraction).atan();
                }
                Infinity(_sign) => {
                    atan_slope = FRAC_PI_2;
                }
                NaN => panic!("Angle not defined for NaN slope"),
            }
            match self.direction {
                Direction::Right => atan_slope,
                Direction::Left => atan_slope - PI,
            }
        }
    }

    impl Eq for SightLine {}

    impl Ord for SightLine {
        // Ordering is defined so that the min SightLine gets shot first.
        // This way we can use BTree pop_min to sort SightLines.
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .get_angle()
                .partial_cmp(&self.get_angle())
                .unwrap_or(Equal)
        }
    }
    impl PartialOrd for SightLine {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Location {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    struct RangedLocation {
        location: Location,
        distance: f64,
    }

    #[test]
    fn test_angle() {
        let sight_line = SightLine {
            slope: F::new(-1, 0),
            direction: Direction::Right,
        };
        match sight_line.slope {
            Rational(sign, ratio) => {
                let float_fraction: f64 = *ratio.numer() as f64 / *ratio.denom() as f64;
                println!("{} {}", sign, float_fraction);
            }
            Infinity(sign) => {
                println!("Infinity {}", sign);
            }
            NaN => {}
        }
    }

    #[test]
    fn test_angle_ordering() {
        // loc above reference
        let loc = Location { x: 4, y: 4 };
        let sight_line_up = get_sight_line(&Location { x: 4, y: 3 }, &loc);
        let sight_line_up_right = get_sight_line(&Location { x: 5, y: 3 }, &loc);
        let sight_line_right = get_sight_line(&Location { x: 5, y: 4 }, &loc);
        let sight_line_down_right = get_sight_line(&Location { x: 5, y: 5 }, &loc);
        let sight_line_down = get_sight_line(&Location { x: 4, y: 5 }, &loc);
        let sight_line_down_left = get_sight_line(&Location { x: 3, y: 5 }, &loc);
        let sight_line_left = get_sight_line(&Location { x: 3, y: 4 }, &loc);
        let sight_line_up_left = get_sight_line(&Location { x: 3, y: 3 }, &loc);
        assert_eq!(
            sight_line_up.get_angle() > sight_line_up_right.get_angle(),
            true
        );
        assert_eq!(
            sight_line_up_right.get_angle() > sight_line_right.get_angle(),
            true
        );
        assert_eq!(
            sight_line_right.get_angle() > sight_line_down_right.get_angle(),
            true
        );
        assert_eq!(
            sight_line_down_right.get_angle() > sight_line_down.get_angle(),
            true
        );
        assert_eq!(
            sight_line_down_left.get_angle() > sight_line_left.get_angle(),
            true
        );
        assert_eq!(
            sight_line_left.get_angle() > sight_line_up_left.get_angle(),
            true
        );

        assert_eq!(sight_line_up < sight_line_up_right, true);
        assert_eq!(sight_line_up_right < sight_line_right, true);
        assert_eq!(sight_line_right < sight_line_down_right, true);
        assert_eq!(sight_line_down_right < sight_line_down, true);
        assert_eq!(sight_line_down_left < sight_line_left, true);
        assert_eq!(sight_line_left < sight_line_up_left, true);
    }

    #[test]
    fn test_infinity_slope() {
        // loc above reference
        let sight_line = get_sight_line(&Location { x: 4, y: 4 }, &Location { x: 4, y: 5 });
        if let Infinity(sign) = sight_line.slope {
            assert_eq!(sign, Sign::Plus);
        }
        assert_eq!(sight_line.direction, Direction::Right);

        // loc below reference
        let sight_line = get_sight_line(&Location { x: 4, y: 4 }, &Location { x: 4, y: 3 });
        if let Infinity(sign) = sight_line.slope {
            assert_eq!(sign, Sign::Plus);
        }
        assert_eq!(sight_line.direction, Direction::Left);
    }

    #[test]
    fn atan() {
        let result = 0.0_f64.atan();
        println!("atan 0.0 {}", result);
        let result = -9999.0_f64.atan();
        println!("atan -9999.0 {}", result);
        let result = 9999.0_f64.atan();
        println!("atan 9999.0 {}", result);
    }

    fn read_locations(input: String) -> Vec<Location> {
        let mut locations = Vec::<Location>::new();
        for (y, line) in input.split("\n").enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => locations.push(Location {
                        x: x as i64,
                        y: y as i64,
                    }),
                    _ => panic!("Found unexpected character {}", c),
                }
            }
        }
        locations
    }

    fn shoot_ordering(locations: Vec<Location>, ref_loc: Location) -> Vec<RangedLocation> {
        let mut result = Vec::<RangedLocation>::new();
        let num_targets = locations.len() - 1;
        // mapping for sight line to vector of locations with that sight line
        let mut sight_lines = BTreeMap::<SightLine, Vec<RangedLocation>>::new();
        for location in locations {
            // We do not shoot the asteroid we are on.
            if location == ref_loc {
                continue;
            }
            let sight_line = get_sight_line(&location, &ref_loc);
            let distance = get_distance(&location, &ref_loc);
            let ranged_loc = RangedLocation {
                distance: distance,
                location: location,
            };
            if !sight_lines.contains_key(&sight_line) {
                sight_lines.insert(sight_line, vec![ranged_loc]);
            } else {
                if let Some(ranged_loc_vec) = sight_lines.get_mut(&sight_line) {
                    ranged_loc_vec.push(ranged_loc);
                };
            }
        }

        // Sorts sightline asteroids in descending order by distance so that they can be
        // popped off the end when destroyed
        for ranged_loc_vec in sight_lines.values_mut() {
            ranged_loc_vec.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap_or(Equal));
        }

        loop {
            for value in sight_lines.values_mut() {
                if let Some(ranged_loc) = value.pop() {
                    result.push(ranged_loc);
                }
            }
            if result.len() == num_targets {
                break;
            }
        }
        result
    }

    #[test]
    fn test_shoot_ordering_complex_example() {
        let locations = read_locations(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".to_string());
        let result = shoot_ordering(locations, Location { x: 11, y: 13 });
        assert_eq!(result[0].location, Location { x: 11, y: 12 });
        assert_eq!(result[1].location, Location { x: 12, y: 1 });
        assert_eq!(result[2].location, Location { x: 12, y: 2 });
        assert_eq!(result[9].location, Location { x: 12, y: 8 });
        assert_eq!(result[19].location, Location { x: 16, y: 0 });
        assert_eq!(result[49].location, Location { x: 16, y: 9 });
        assert_eq!(result[99].location, Location { x: 10, y: 16 });
        assert_eq!(result[198].location, Location { x: 9, y: 6 });
        assert_eq!(result[199].location, Location { x: 8, y: 2 });
        assert_eq!(result[200].location, Location { x: 10, y: 9 });
        assert_eq!(result[298].location, Location { x: 11, y: 1 });
    }

    #[test]
    fn test_shoot_ordering() {
        let readresult = fs::read_to_string("adventinputs/year19day10.txt");
        if let Ok(input) = readresult {
            let locations = read_locations(input);
            let result = shoot_ordering(locations, Location { x: 23, y: 19 });
            println!(
                "THE ANSWER TO 2019 DAY 10 PART TWO IS {:?}",
                result[199].location.x * 100 + result[199].location.y
            )
        }
    }

    fn get_distance(loc: &Location, ref_loc: &Location) -> f64 {
        // Manhattan distance, chosen for simplicity.
        // The important thing is that distances along the same sight line are in the correct order
        ((ref_loc.x - loc.x).abs() + (ref_loc.y - loc.y).abs()) as f64
    }

    fn get_sight_line(loc: &Location, ref_loc: &Location) -> SightLine {
        if (ref_loc.x < loc.x) || (ref_loc.x == loc.x && ref_loc.y > loc.y) {
            SightLine {
                slope: F::new(ref_loc.y - loc.y, ref_loc.x - loc.x),
                direction: Direction::Right,
            }
        } else {
            SightLine {
                slope: F::new(ref_loc.y - loc.y, ref_loc.x - loc.x),
                direction: Direction::Left,
            }
        }
    }
    fn num_sight_lines(locations: &Vec<Location>, ref_loc: &Location) -> i64 {
        let mut sight_lines = HashSet::<SightLine>::new();
        for loc in locations.iter() {
            sight_lines.insert(get_sight_line(loc, ref_loc));
        }
        (sight_lines.len() - 1) as i64
    }

    struct Solution<'a> {
        max: i64,
        argmax: &'a Location,
    }

    fn optimize_sight_lines(locations: &Vec<Location>) -> Solution {
        let mut max = 0;
        let mut argmax = &Location { x: 0, y: 0 };
        for loc in locations.iter() {
            let sight_lines = num_sight_lines(locations, loc);
            if sight_lines > max {
                max = sight_lines;
                argmax = loc;
            }
        }
        Solution { max, argmax }
    }

    // TODO: test arcsin

    #[test]
    fn year19day10partonechallenge() {
        let readresult = fs::read_to_string("adventinputs/year19day10.txt");
        if let Ok(input) = readresult {
            let locations = read_locations(input);
            let solution = optimize_sight_lines(&locations);
            println!(
                "THE ANSWER TO YEAR 2019 DAY 10 PART ONE IS {}",
                solution.max
            );
            println!("THE LOCATION OF THE ASTEROID IS {:?}", solution.argmax);
        }
    }

    #[test]
    fn test_optimize_sightlines() {
        let locations = read_locations(".#..#\n.....\n#####\n....#\n...##".to_string());
        let result = optimize_sight_lines(&locations).max;
        assert_eq!(result, 8);
    }

    #[test]
    fn test_optimize_sightlines_complex_example() {
        let locations = read_locations(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".to_string());
        let result = optimize_sight_lines(&locations).max;
        assert_eq!(result, 210);
    }

    #[test]
    fn test_num_sightlines() {
        let locations = read_locations(".#..#\n.....\n#####\n....#\n...##".to_string());
        let result = num_sight_lines(&locations, &Location { x: 3, y: 4 });
        assert_eq!(result, 8);

        let result = num_sight_lines(&locations, &Location { x: 4, y: 2 });
        assert_eq!(result, 5);
    }

    #[test]
    fn test_read_locations() {
        let result = read_locations(".#\n#.\n".to_string());
        assert_eq!(result[0], Location { x: 1, y: 0 });
        assert_eq!(result[1], Location { x: 0, y: 1 });
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn hash_map_fraction() {
        let mut sight_lines = HashSet::<SightLine>::new();
        sight_lines.insert(SightLine {
            slope: F::new(1, 2),
            direction: Direction::Left,
        });
    }

    #[test]
    fn test_fraction() {
        let first = F::new(1, 2);
        let second = F::new(2, 8);
        assert_eq!(first + second, F::new(3, 4));
    }

    #[test]
    fn test_zero_denominator() {
        let first = F::new(1, 0);
        let second = F::new(-1, 0);
        let third = F::new(1, 1);
        let fourth = F::new(0, 0);
        let fifth = F::new(0, 1);
        assert_eq!(first, second);
        assert_ne!(first, third);
        assert_ne!(first, fourth);
        assert_ne!(fourth, fifth);
    }
}
