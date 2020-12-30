mod y20d02 {
    #[derive(Debug, Eq, PartialEq)]
    enum Continuation {
        Inputs(Vec<PasswordPolicy>),
        Done(i64),
    }

    #[derive(Debug, Eq, PartialEq)]
    struct PasswordPolicy {
        min: i64,
        max: i64,
        ref_char: String,
        password: String,
    }

    fn is_valid_appearance(policy: PasswordPolicy) -> bool {
        let count = policy.password.as_str().matches(&policy.ref_char).count() as i64;
        count >= policy.min && count <= policy.max
    }

    fn count_valid_appearance(k: Continuation) -> Continuation {
        let mut count = 0;
        let policies = match k {
            Continuation::Inputs(k_policies) => k_policies,
            _ => panic!("No policies given"),
        };
        for policy in policies {
            if is_valid_appearance(policy) {
                count += 1;
            }
        }
        return Continuation::Done(count);
    }

    fn is_valid_index(policy: PasswordPolicy) -> bool {
        let min_match = policy.password.as_bytes()[(policy.min - 1) as usize] as char
            == policy.ref_char.parse::<char>().unwrap();
        let max_match = policy.password.as_bytes()[(policy.max - 1) as usize] as char
            == policy.ref_char.parse::<char>().unwrap();
        if min_match && max_match {
            return false;
        }
        return min_match || max_match;
    }

    fn count_valid_index(k: Continuation) -> Continuation {
        let mut count = 0;
        let policies = match k {
            Continuation::Inputs(k_policies) => k_policies,
            _ => panic!("No policies given"),
        };
        for policy in policies {
            if is_valid_index(policy) {
                count += 1;
            }
        }
        return Continuation::Done(count);
    }

    fn parse_text() -> Continuation {
        use regex::Regex;
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/y20d02.txt");
        if let Ok(input) = readresult {
            let mut password_policies: Vec<PasswordPolicy> = vec![];
            let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<refchar>\w): (?P<password>\w+)")
                .unwrap();
            for token in (*input).split("\n") {
                let caps = re.captures(token).unwrap();
                password_policies.push(PasswordPolicy {
                    min: caps.name("min").unwrap().as_str().parse::<i64>().unwrap(),
                    max: caps.name("max").unwrap().as_str().parse::<i64>().unwrap(),
                    ref_char: caps.name("refchar").unwrap().as_str().to_string(),
                    password: caps.name("password").unwrap().as_str().to_string(),
                });
            }
            return Continuation::Inputs(password_policies);
        } else {
            panic!("Could not read file")
        }
    }

    #[test]
    fn y20d02ch01() {
        let k = parse_text();
        let k = count_valid_appearance(k);
        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 2 CHALLENGE 1 --------------------> {:?}",
            answer,
        );
    }

    #[test]
    fn y20d02ch02() {
        let k = parse_text();
        let k = count_valid_index(k);
        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 2 CHALLENGE 2 --------------------> {:?}",
            answer,
        );
    }
}
