#[cfg(test)]
mod year19day06 {
    use std::collections::hash_map::Entry;
    use std::collections::{HashMap, VecDeque};
    use std::fs;

    struct OrbitData {
        orbits: HashMap<String, String>,
        neighbors: HashMap<String, Vec<String>>,
    }

    fn load_orbits(input: String) -> OrbitData {
        let mut orbits = HashMap::<String, String>::new();
        let mut neighbors = HashMap::<String, Vec<String>>::new();
        for orbit in (*input).split("\n") {
            let relation: Vec<&str> = orbit.split(")").collect();
            orbits.insert(relation[1].to_string(), relation[0].to_string());
            match neighbors.entry(relation[1].to_string()) {
                Entry::Vacant(e) => {
                    e.insert(vec![relation[0].to_string()]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(relation[0].to_string());
                }
            }
            match neighbors.entry(relation[0].to_string()) {
                Entry::Vacant(e) => {
                    e.insert(vec![relation[1].to_string()]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(relation[1].to_string());
                }
            }
        }
        OrbitData { orbits, neighbors }
    }

    fn find_distance_between_nodes(orbit_data: &OrbitData, start: String, end: String) -> i64 {
        let mut distances = HashMap::<String, i64>::new();
        let mut nodes_to_process = VecDeque::<String>::new();
        distances.insert(start.clone(), 0);
        nodes_to_process.push_back(start.clone());
        loop {
            match nodes_to_process.pop_front() {
                Some(node) => {
                    if let Some(kv) = orbit_data.neighbors.get_key_value(&node) {
                        let neighbors = kv.1;
                        for neighbor in neighbors {
                            if !distances.contains_key(neighbor) {
                                match distances.get(&node) {
                                    Some(distance) => {
                                        if *neighbor == end {
                                            return distance + 1;
                                        }
                                        let neighbor_distance = distance + 1;
                                        distances.insert(neighbor.clone(), neighbor_distance);
                                        nodes_to_process.push_back(neighbor.clone());
                                    }
                                    None => panic!("nodes in process should already have their distances calculated.")
                                }
                            }
                        }
                    }
                }
                None => panic!("nodes_to_process emptied before finding the distance"),
            };
        }
    }

    fn count_orbits_of_planet(orbits: &HashMap<String, String>, key: &String) -> i64 {
        let mut count = 0;
        let mut done = false;
        let mut current_key = key;
        while !done {
            match orbits.get_key_value(current_key) {
                Some(value) => {
                    count += 1;
                    current_key = value.1;
                }
                None => done = true,
            }
        }
        count
    }

    #[test]
    fn day_six_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day06.txt");
        match readresult {
            Ok(input) => {
                let orbit_data = load_orbits(input);
                let mut count = 0;
                for key in orbit_data.orbits.keys() {
                    count += count_orbits_of_planet(&orbit_data.orbits, &key);
                }
                println!("THE ANSWER TO DAY SIX PART ONE IS {}", count);
            }
            Err(_e) => panic!("COULD NOT OPEN FILE"),
        }
    }

    #[test]
    fn day_six_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day06.txt");
        match readresult {
            Ok(input) => {
                let orbit_data = load_orbits(input);
                let start = orbit_data.orbits.get("YOU").unwrap();
                let end = orbit_data.orbits.get("SAN").unwrap();
                let result = find_distance_between_nodes(&orbit_data, start.clone(), end.clone());
                println!("THE ANSWER TO DAY SIX PART TWO IS {}", result);
            }
            Err(_e) => panic!("COULD NOT OPEN FILE"),
        }
    }
}
