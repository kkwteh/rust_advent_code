#[cfg(test)]
mod year19day08 {
    use std::fs;
    use std::str::Chars;
    struct LayerChunker<'a> {
        data_iter: Chars<'a>,
        layer_s: i64,
    }

    #[derive(Debug)]
    struct Layer {
        zero_count: i64,
        one_count: i64,
        two_count: i64,
        chars: Vec<char>,
    }

    impl Iterator for LayerChunker<'_> {
        type Item = Layer;
        // Here, we define the sequence using `.curr` and `.next`.
        // The return type is `Option<T>`:
        //     * When the `Iterator` is finished, `None` is returned.
        //     * Otherwise, the next value is wrapped in `Some` and returned.
        fn next(&mut self) -> Option<Layer> {
            let mut zero_count = 0;
            let mut one_count = 0;
            let mut two_count = 0;
            let mut chars = Vec::<char>::new();
            for _i in 0..self.layer_s {
                match self.data_iter.next() {
                    Some(c) => {
                        match c {
                            '0' => zero_count += 1,
                            '1' => one_count += 1,
                            '2' => two_count += 1,
                            _ => panic!("Unexpected char {}", c),
                        }
                        chars.push(c)
                    }
                    None => return None,
                };
            }
            Some(Layer {
                zero_count,
                one_count,
                two_count,
                chars,
            })
        }
    }

    #[test]
    fn day_eight_part_one_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day08.txt");
        if let Ok(input) = readresult {
            let mut min = i64::max_value();
            let mut argmax: Option<Layer> = None;
            let layer_s = 25 * 6;
            let lc = LayerChunker {
                data_iter: input.chars(),
                layer_s,
            };
            for stats in lc {
                println!("Zero Count {}", stats.zero_count);
                if stats.zero_count < min {
                    min = stats.zero_count;
                    argmax = Some(stats);
                }
            }
            if let Some(someargmax) = argmax {
                println!(
                    "The ANSWER TO 2019 DAY EIGHT PART ONE IS One count x Two count {}",
                    someargmax.one_count * someargmax.two_count
                );
            }
        }
    }

    fn first_non_two(layers: &Vec<Layer>, index: usize) -> Option<char> {
        for layer in layers {
            if layer.chars[index] != '2' {
                return Some(layer.chars[index]);
            }
        }
        None
    }

    #[test]
    fn day_eight_part_two_challenge() {
        let readresult = fs::read_to_string("adventinputs/year19day08.txt");
        if let Ok(input) = readresult {
            let layer_size = 25 * 6;
            let lc = LayerChunker {
                data_iter: input.chars(),
                layer_s: layer_size,
            };
            let layers: Vec<Layer> = lc.collect();
            let mut display_chars = Vec::<char>::new();
            for i in 0..layer_size {
                match first_non_two(&layers, i as usize) {
                    Some(c) => display_chars.push(c),
                    None => panic!("No non-two character found"),
                }
            }
            let display: String = display_chars
                .iter()
                .map(|c| match c {
                    '0' => ' ',
                    '1' => 'X',
                    _ => panic!("Unexpected char found"),
                })
                .collect();
            println!("THE ANSWER TO DAY EIGHT PART TWO IS:");
            for i in 0..6 {
                println!("{}", &display[25 * i..25 * (i + 1)]);
            }
        }
    }
}
