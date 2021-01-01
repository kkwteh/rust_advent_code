mod y20d05 {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fmt;

    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    struct SeatData {
        min_row: i64,
        max_row: i64,
        min_column: i64,
        max_column: i64,
    }
    impl SeatData {
        fn id(&self) -> i64 {
            assert_eq!(self.min_row, self.max_row);
            assert_eq!(self.min_column, self.max_column);
            self.min_row * 8 + self.min_column
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    enum Continuation {
        Inputs(Vec<String>),
        SeatData(Vec<SeatData>),
        Done(i64),
    }
    fn parse_text() -> Continuation {
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/y20d05.txt");
        let input = readresult.unwrap();

        let mut inputs: Vec<String> = vec![];
        for line in (*input).split("\n") {
            inputs.push(line.to_string());
        }
        return Continuation::Inputs(inputs);
    }

    fn extract_seat_data(k: Continuation) -> Continuation {
        let mut seat_data_vec: Vec<SeatData> = vec![];
        let inputs = match k {
            Continuation::Inputs(k_inputs) => k_inputs,
            _ => panic!("No inputs given"),
        };

        for input in inputs {
            let mut seat_data = SeatData {
                min_row: 0,
                max_row: 127,
                min_column: 0,
                max_column: 7,
            };
            for c in input.chars() {
                seat_data = process_character(c, seat_data)
            }
            seat_data_vec.push(seat_data)
        }

        return Continuation::SeatData(seat_data_vec);
    }

    fn process_character(c: char, seat_data: SeatData) -> SeatData {
        let mut res = seat_data.clone();
        // powers of 2
        let half_row_delta: i64 = (res.max_row - res.min_row + 1) / 2;
        let half_column_delta: i64 = (res.max_column - res.min_column + 1) / 2;
        match c {
            'F' => res.max_row = res.max_row - half_row_delta,
            'B' => res.min_row = res.min_row + half_row_delta,
            'L' => res.max_column = res.max_column - half_column_delta,
            'R' => res.min_column = res.min_column + half_column_delta,
            _ => panic!("Unknown character found"),
        }
        return res;
    }

    fn max_seat_id(k: Continuation) -> Continuation {
        let seat_data_list = match k {
            Continuation::SeatData(k_seat_data_list) => k_seat_data_list,
            _ => panic!("No inputs given"),
        };
        let mut max_id = 0;
        for seat_data in seat_data_list {
            if seat_data.id() > max_id {
                max_id = seat_data.id();
            }
        }
        return Continuation::Done(max_id);
    }

    fn missing_seat_id(k: Continuation) -> Continuation {
        let mut seat_id_set = HashSet::<i64>::new();
        let seat_data_list = match k {
            Continuation::SeatData(k_seat_data_list) => k_seat_data_list,
            _ => panic!("No inputs given"),
        };
        // if seen_points.get(&leveled_point) == None {
        //     seen_points.insert(leveled_point);
        let mut min_id = 99999999;
        let mut max_id = 0;
        for seat_data in seat_data_list {
            if seat_data.id() > max_id {
                max_id = seat_data.id();
            }
            if seat_data.id() < min_id {
                min_id = seat_data.id();
            }
            seat_id_set.insert(seat_data.id());
        }
        let mut current_id = min_id;
        println!("seat id set {:?}", seat_id_set);
        loop {
            if seat_id_set.contains(&(current_id - 1))
                && seat_id_set.contains(&(current_id + 1))
                && !seat_id_set.contains(&current_id)
            {
                return Continuation::Done(current_id);
            }
            current_id += 1;
            if current_id == max_id {
                panic!("Could not find missing seat id");
            }
        }
    }
    #[test]
    fn y20d05ch01() {
        let k = parse_text();
        let k = extract_seat_data(k);
        let k = max_seat_id(k);

        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 5 CHALLENGE 1 --------------------> {:?}",
            answer,
        );
    }

    #[test]
    fn y20d05ch02() {
        let k = parse_text();
        let k = extract_seat_data(k);
        let k = missing_seat_id(k);

        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 5 CHALLENGE 2 --------------------> {:?}",
            answer,
        );
    }
}
