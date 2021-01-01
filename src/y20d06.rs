mod y20d06 {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fmt;

    #[derive(Debug, Eq, PartialEq)]
    struct IndivAnsweredYes {
        group_size: i64,
        count_map: HashMap<char, i64>,
    }

    #[derive(Debug, Eq, PartialEq)]
    enum Continuation {
        Responses(Vec<String>),
        AnsweredYes(Vec<HashSet<char>>),
        IndivAnsweredYes(Vec<IndivAnsweredYes>),
        Done(i64),
    }
    fn parse_text() -> Continuation {
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/y20d06.txt");
        let input = readresult.unwrap();

        let mut response_vec: Vec<String> = vec![];
        for response in (*input).split("\n\n") {
            response_vec.push(response.to_string());
        }
        return Continuation::Responses(response_vec);
    }

    fn extract_answered_yes(k: Continuation) -> Continuation {
        let response_vec = match k {
            Continuation::Responses(k_response_vec) => k_response_vec,
            _ => panic!("No responses given"),
        };
        let mut answered_yes_vec: Vec<HashSet<char>> = vec![];

        for response in response_vec {
            let mut answered_yes = HashSet::<char>::new();
            for c in response.chars() {
                match c {
                    'a'..='z' => {
                        answered_yes.insert(c);
                    }
                    _ => (),
                }
            }
            answered_yes_vec.push(answered_yes);
        }

        return Continuation::AnsweredYes(answered_yes_vec);
    }

    fn extract_indiv_answered_yes(k: Continuation) -> Continuation {
        let response_vec = match k {
            Continuation::Responses(k_response_vec) => k_response_vec,
            _ => panic!("No responses given"),
        };
        let mut indiv_answered_yes_vec = vec![];
        for response in response_vec {
            let mut count_map = HashMap::<char, i64>::new();
            let mut group_size = 0;
            for line in response.split('\n') {
                for c in line.chars() {
                    *count_map.entry(c).or_insert(0) += 1;
                }
                group_size += 1;
            }
            indiv_answered_yes_vec.push(IndivAnsweredYes {
                group_size: group_size,
                count_map: count_map,
            });
        }
        return Continuation::IndivAnsweredYes(indiv_answered_yes_vec);
    }

    fn sum_set_lengths(k: Continuation) -> Continuation {
        let answered_yes_vec = match k {
            Continuation::AnsweredYes(k_answered_yes_vec) => k_answered_yes_vec,
            _ => panic!("No answered yes vector given"),
        };
        let mut res = 0;
        for answered_yes in answered_yes_vec {
            res += answered_yes.len();
        }
        return Continuation::Done(res as i64);
    }

    fn sum_all_yeses(k: Continuation) -> Continuation {
        let answered_yes_vec = match k {
            Continuation::IndivAnsweredYes(k_answered_yes_vec) => k_answered_yes_vec,
            _ => panic!("No answered yes vector given"),
        };
        let mut res = 0;
        for answered_yes in answered_yes_vec {
            for (_, count) in answered_yes.count_map.iter() {
                if *count == answered_yes.group_size {
                    res += 1;
                }
            }
        }
        return Continuation::Done(res as i64);
    }

    #[test]
    fn y20d06ch01() {
        let k = parse_text();
        let k = extract_answered_yes(k);
        let k = sum_set_lengths(k);

        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 6 CHALLENGE 1 --------------------> {:?}",
            answer,
        );
    }

    #[test]
    fn y20d06ch02() {
        let k = parse_text();
        let k = extract_indiv_answered_yes(k);
        let k = sum_all_yeses(k);

        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 6 CHALLENGE 2 --------------------> {:?}",
            answer,
        );
    }
}
