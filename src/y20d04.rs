mod y20d04 {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fmt;

    #[derive(Debug)]
    struct ValidationError {
        details: String,
    }

    impl ValidationError {
        fn new(msg: &str) -> ValidationError {
            ValidationError {
                details: msg.to_string(),
            }
        }
    }

    impl fmt::Display for ValidationError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    struct PassportData {
        birth_year: Option<String>,
        issue_year: Option<String>,
        expiration_year: Option<String>,
        height: Option<String>,
        hair_color: Option<String>,
        eye_color: Option<String>,
        passport_id: Option<String>,
        country_id: Option<String>,
    }

    impl PassportData {
        fn has_required_fields(&self) -> bool {
            self.birth_year.is_some()
                && self.issue_year.is_some()
                && self.expiration_year.is_some()
                && self.height.is_some()
                && self.hair_color.is_some()
                && self.eye_color.is_some()
                && self.passport_id.is_some()
        }
    }

    impl PassportData {
        fn is_valid(&self) -> Result<(), ValidationError> {
            if !self.has_required_fields() {
                return Err(ValidationError::new("missing required field"));
            }

            let eye_colors: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .into_iter()
                .collect();

            let hex_chars: HashSet<u8> = vec![
                b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd',
                b'e', b'f',
            ]
            .into_iter()
            .collect();

            let digits: HashSet<u8> =
                vec![b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']
                    .into_iter()
                    .collect();

            match self.birth_year.as_ref().unwrap().parse::<i64>() {
                Ok(result) => {
                    if result < 1920 || result > 2002 {
                        return Err(ValidationError::new("birth year out of range"));
                    }
                }
                Err(error) => {
                    return Err(ValidationError::new("birth year unparseable"));
                }
            }

            match self.issue_year.as_ref().unwrap().parse::<i64>() {
                Ok(result) => {
                    if result < 2010 || result > 2020 {
                        return Err(ValidationError::new("issue year out of range"));
                    }
                }
                Err(_error) => {
                    return Err(ValidationError::new("issue year unparseable"));
                }
            }

            match self.expiration_year.as_ref().unwrap().parse::<i64>() {
                Ok(result) => {
                    if result < 2020 || result > 2030 {
                        return Err(ValidationError::new("expiration year out of range"));
                    }
                }
                Err(_error) => {
                    return Err(ValidationError::new("expiration year unparseable"));
                }
            }

            let height_ref = &self.height.as_ref().unwrap();
            let unit = &height_ref[height_ref.len() - 2..];
            match height_ref[..height_ref.len() - 2].parse::<i64>() {
                Ok(result) => {
                    if unit == "cm" {
                        if result < 150 || result > 193 {
                            return Err(ValidationError::new("height cm out of range"));
                        }
                    } else if unit == "in" {
                        if result < 59 || result > 76 {
                            return Err(ValidationError::new("height in out of range"));
                        }
                    } else {
                        return Err(ValidationError::new("height units invalid"));
                    }
                }
                Err(_error) => {
                    return Err(ValidationError::new("height unparseable"));
                }
            }

            let hair_color = &self.hair_color.as_ref().unwrap().as_bytes();

            if hair_color[0] != b'#' {
                return Err(ValidationError::new("hair color has no leading #"));
            }
            for byte in &hair_color[1..] {
                if !hex_chars.contains(byte) {
                    return Err(ValidationError::new("hair color hex has invalid character"));
                }
            }

            let eye_color = self.eye_color.as_ref().unwrap().as_str();
            if !eye_colors.contains(eye_color) {
                return Err(ValidationError::new("eye color is not a valid value"));
            }

            let passport_number = &self.passport_id.as_ref().unwrap().as_bytes();

            for byte in &passport_number[..] {
                if !digits.contains(byte) {
                    return Err(ValidationError::new("passport id has invalid character"));
                }

                if passport_number.len() != 9 {
                    return Err(ValidationError::new(
                        "passport id is not the correct length",
                    ));
                }
            }

            return Ok(());
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    enum Continuation {
        Inputs(Vec<String>),
        PassportData(Vec<PassportData>),
        Done(i64),
    }
    fn parse_text() -> Continuation {
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/y20d04.txt");
        let input = readresult.unwrap();

        let mut inputs: Vec<String> = vec![];
        for passport_data in (*input).split("\n\n") {
            inputs.push(passport_data.to_string());
        }
        return Continuation::Inputs(inputs);
    }

    fn extract_passport_data(k: Continuation) -> Continuation {
        use regex::Regex;
        let re = Regex::new(r"(?P<name>\w+):(?P<value>[\w#]+)").unwrap();

        let raw_inputs = match k {
            Continuation::Inputs(k_inputs) => k_inputs,
            _ => panic!("No inputs given"),
        };

        let mut passport_hash = HashMap::<String, String>::new();
        let mut passport_data: Vec<PassportData> = vec![];
        for raw_input in raw_inputs {
            for cap in re.captures_iter(&raw_input) {
                passport_hash.insert(
                    cap.name("name").unwrap().as_str().to_string(),
                    cap.name("value").unwrap().as_str().to_string(),
                );
            }
            passport_data.push(PassportData {
                birth_year: passport_hash.remove("byr"),
                issue_year: passport_hash.remove("iyr"),
                expiration_year: passport_hash.remove("eyr"),
                height: passport_hash.remove("hgt"),
                hair_color: passport_hash.remove("hcl"),
                eye_color: passport_hash.remove("ecl"),
                passport_id: passport_hash.remove("pid"),
                country_id: passport_hash.remove("cid"),
            });
        }
        return Continuation::PassportData(passport_data);
    }

    fn count_required_fields(k: Continuation) -> Continuation {
        let passport_data = match k {
            Continuation::PassportData(k_passport_data) => k_passport_data,
            _ => panic!("No passport data given"),
        };
        println!("Number of passports {:?}", passport_data.len());
        let mut answer = 0;
        for data in passport_data {
            if data.has_required_fields() {
                answer += 1;
            // println!("VALID {:?}", data)
            } else {
                // println!("INVALID {:?}", data)
            }
        }
        return Continuation::Done(answer);
    }

    fn count_valid(k: Continuation) -> Continuation {
        let passport_data = match k {
            Continuation::PassportData(k_passport_data) => k_passport_data,
            _ => panic!("No passport data given"),
        };
        println!("Number of passports {:?}", passport_data.len());
        let mut answer = 0;
        for data in passport_data {
            match data.is_valid() {
                Ok(()) => {
                    answer += 1;
                    println!("VALID {:?}", data);
                }
                Err(error) => {
                    println!("INVALID {:?}", data);
                    println!("ERROR {:?}", error);
                }
            }
        }
        return Continuation::Done(answer);
    }

    #[test]
    fn y20d04ch01() {
        let k = parse_text();
        let k = extract_passport_data(k);
        let k = count_required_fields(k);
        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 4 CHALLENGE 1 --------------------> {:?}",
            answer,
        );
    }

    #[test]
    fn y20d04ch02() {
        let k = parse_text();
        let k = extract_passport_data(k);
        let k = count_valid(k);
        let answer = match k {
            Continuation::Done(k_answer) => k_answer,
            _ => panic!("No answer received"),
        };
        println!(
            "ANSWER TO 2020 DAY 4 CHALLENGE 1 --------------------> {:?}",
            answer,
        );
    }
}
