#[cfg(test)]
mod year19day15 {
    use crate::intcode::intcode;
    use std::collections::{HashMap, VecDeque};
    use std::fs;

    #[derive(Debug)]
    enum Tile {
        Empty,
        Wall,
        OxygenSystem,
        Oxygen(i64),
    }

    #[derive(Debug, PartialEq, Hash, Clone, Copy)]
    struct Location {
        x: i64,
        y: i64,
    }

    impl Location {
        fn north(&self) -> (Location, i64) {
            (
                Location {
                    x: self.x,
                    y: self.y - 1,
                },
                1,
            )
        }

        fn south(&self) -> (Location, i64) {
            (
                Location {
                    x: self.x,
                    y: self.y + 1,
                },
                2,
            )
        }

        fn west(&self) -> (Location, i64) {
            (
                Location {
                    x: self.x - 1,
                    y: self.y,
                },
                3,
            )
        }

        fn east(&self) -> (Location, i64) {
            (
                Location {
                    x: self.x + 1,
                    y: self.y,
                },
                4,
            )
        }
    }

    #[derive(Clone)]
    struct Path {
        ctx: intcode::ProgramContext,
        location: Location,
        steps: i64,
        prev_locations: Vec<Location>,
    }

    impl Eq for Location {}

    fn explore_dungeon(ctx: intcode::ProgramContext) {
        let mut oxygen_system_location: Option<Location> = None;
        let mut known_locs = HashMap::<Location, Tile>::new();
        let start = Location { x: 0, y: 0 };
        known_locs.insert(start, Tile::Empty);
        let mut paths = VecDeque::<Path>::new();
        for (location, command) in [start.north(), start.south(), start.west(), start.east()].iter()
        {
            if !known_locs.contains_key(&location) {
                let mut new_ctx = ctx.clone();
                new_ctx.inputs.push(command.clone());
                paths.push_back(Path {
                    ctx: new_ctx,
                    location: *location,
                    steps: 1,
                    prev_locations: vec![start],
                })
            }
        }
        loop {
            if paths.len() == 0 {
                println!("Dungeon fully explored!");
                break;
            }
            let mut path = paths.pop_front().unwrap();
            path.ctx.run_to_next_input_or_done();
            match path.ctx.outputs[path.ctx.outputs.len() - 1] {
                0 => {
                    known_locs.insert(path.location, Tile::Wall);
                    continue;
                }
                1 => {
                    known_locs.insert(path.location, Tile::Empty);
                }
                2 => {
                    known_locs.insert(path.location, Tile::OxygenSystem);
                    oxygen_system_location = Some(path.location);
                    println!(
                        "OxygenSystem found at {},{} in {} steps",
                        path.location.x, path.location.y, path.steps
                    );
                    println!("Path taken {:?}", path.prev_locations);
                }
                _ => panic!(
                    "Unexpected output found {}",
                    path.ctx.outputs[path.ctx.outputs.len() - 1]
                ),
            }
            for (location, command) in [
                path.location.north(),
                path.location.south(),
                path.location.west(),
                path.location.east(),
            ]
            .iter()
            {
                if !known_locs.contains_key(&location) {
                    let mut new_ctx = path.ctx.clone();
                    new_ctx.inputs.push(command.clone());
                    let mut prev_locations = path.prev_locations.clone();
                    prev_locations.push(path.location);
                    paths.push_back(Path {
                        ctx: new_ctx,
                        location: *location,
                        steps: path.steps + 1,
                        prev_locations: prev_locations,
                    })
                }
            }
        }

        let mut oxygen_sources = VecDeque::<(Location, i64)>::new();
        oxygen_sources.push_back((oxygen_system_location.unwrap(), 0));
        let mut oxygen_spread_time = 0;
        while oxygen_sources.len() > 0 {
            let (source_location, time_minutes) = oxygen_sources.pop_front().unwrap();
            for (location, _) in [
                source_location.north(),
                source_location.south(),
                source_location.west(),
                source_location.east(),
            ]
            .iter()
            {
                match known_locs.get_key_value(&location) {
                    Some((location, Tile::Empty)) => {
                        if time_minutes + 1 > oxygen_spread_time {
                            oxygen_spread_time = time_minutes + 1;
                        }
                        let oxygen_location = location.clone();
                        known_locs.insert(oxygen_location.clone(), Tile::Oxygen(time_minutes + 1));
                        oxygen_sources.push_back((oxygen_location.clone(), time_minutes + 1));
                    }
                    _ => {}
                }
            }
        }

        println!(
            "All spaces filled with oxygen after {} minutes",
            oxygen_spread_time
        );
    }
    #[test]
    fn day_fifteen_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day15.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let ctx = intcode::build_program_context(program, vec![]);
            explore_dungeon(ctx);
        }
    }
}
