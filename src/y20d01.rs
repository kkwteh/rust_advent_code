mod y20d01 {
    #[derive(Debug, Eq, PartialEq)]
    enum Continuation {
        Inputs(Vec<i64>),
        NoMatchFound,
        Done(i64),
    }

    fn parse_text() -> Continuation {
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/y20d01.txt");
        if let Ok(input) = readresult {
            let inputs: Vec<i64> = (*input)
                .split("\n")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            return Continuation::Inputs(inputs);
        } else {
            panic!("Could not read file")
        }
    }

    fn find_pair(k: Continuation) -> Continuation {
        let inputs: Vec<i64>;
        match k {
            Continuation::Inputs(k_inputs) => {
                inputs = k_inputs;
            }
            _ => {
                panic!("No inputs provided");
            }
        }
        for i in 0..inputs.len() {
            for j in (i + 1)..inputs.len() {
                if inputs[i] + inputs[j] == 2020 {
                    return Continuation::Done(inputs[i] * inputs[j]);
                }
            }
        }
        return Continuation::NoMatchFound;
    }

    fn find_triple(k: Continuation) -> Continuation {
        let inputs: Vec<i64>;
        match k {
            Continuation::Inputs(k_inputs) => {
                inputs = k_inputs;
            }
            _ => {
                panic!("No inputs provided");
            }
        }
        for i in 0..inputs.len() {
            for j in (i + 1)..inputs.len() {
                for l in (j + 1)..inputs.len() {
                    if inputs[i] + inputs[j] + inputs[l] == 2020 {
                        return Continuation::Done(inputs[i] * inputs[j] * inputs[l]);
                    }
                }
            }
        }
        return Continuation::NoMatchFound;
    }

    #[test]
    fn y20d01ch01() {
        let k = parse_text();
        let k = find_pair(k);
        match k {
            Continuation::Done(answer) => {
                println!(
                    "ANSWER TO 2020-12-01 CHALLENGE 1 ---------------> {:?}",
                    answer
                );
            }
            _ => panic!("No pair found"),
        }
    }

    #[test]
    fn y20d01ch02() {
        let k = parse_text();
        let k = find_triple(k);
        match k {
            Continuation::Done(answer) => {
                println!(
                    "ANSWER TO 2020-12-01 CHALLENGE 2 ---------------> {:?}",
                    answer
                );
            }
            _ => panic!("No match found"),
        }
    }
}
