mod year19day19 {
    #[test]
    fn day_nineteen_part_one_challenge() {
        use crate::intcode::intcode;
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/year19day19.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let mut tractor_beam_points = 0;
            for i in 0..50 {
                for j in 0..50 {
                    let mut ctx = intcode::build_program_context(program.clone(), vec![]);
                    ctx.inputs.push(i);
                    ctx.inputs.push(j);
                    ctx.run_to_next_input_or_done();
                    if ctx.outputs[ctx.outputs.len() - 1] == 1 {
                        tractor_beam_points += 1;
                    }
                }
            }

            println!(
                "THE ANSWER TO DAY NINETEEN PART ONE IS {}",
                tractor_beam_points
            )
        }
    }

    #[derive(PartialEq, Debug, Hash, Clone)]
    struct TractorRow {
        minx: i64,
        maxx: i64,
    }
    impl Eq for TractorRow {}

    #[test]
    fn day_nineteen_part_two_challenge() {
        use crate::intcode::intcode;
        use std::fs;
        let readresult = fs::read_to_string("adventinputs/year19day19.txt");
        if let Ok(input) = readresult {
            let program = intcode::read_tokens(&input);
            let mut tractor_beam = Vec::<TractorRow>::new();
            // The first few rows are kind of wonky, with some of them empty, so we skip ahead a few rows.
            let mut i = 0;
            let initj = 10;
            let mut j = initj;
            loop {
                let mut minx = -1;
                loop {
                    let mut ctx = intcode::build_program_context(program.clone(), vec![i, j]);
                    ctx.run_to_next_input_or_done();
                    if ctx.outputs[0] == 1 {
                        if minx == -1 {
                            minx = i;
                            // We don't care if the width of the tractor row is less than 100,
                            // so we skip ahead a bit.
                            i += 98;
                        }
                    };
                    if ctx.outputs[0] == 0 {
                        if minx != -1 {
                            let maxx = i;
                            let row = TractorRow {
                                minx: minx,
                                maxx: maxx,
                            };
                            tractor_beam.push(row.clone());
                            println!("Row {}: {:?}", j, row);
                            break;
                        }
                    };
                    i += 1;
                }
                // Analyze tractor beam rows for first square
                if tractor_beam[tractor_beam.len() - 1].maxx
                    - tractor_beam[tractor_beam.len() - 1].minx
                    >= 100
                {
                    let left_x = tractor_beam[tractor_beam.len() - 1].minx;
                    // let bottom_y = tractor_beam.len() - 1;
                    let right_x = left_x + 100;
                    let top_y = tractor_beam.len() - 100 + initj as usize;
                    if tractor_beam[top_y - initj as usize].minx <= left_x
                        && tractor_beam[top_y - initj as usize].maxx >= right_x
                    {
                        println!(
                            "Found first 100 by 100 square with top left corner {}, {}",
                            left_x, top_y
                        );
                        println!(
                            "THE ANSWER TO DAY NINETEEN PART TWO IS {}",
                            left_x * 10000 + top_y as i64
                        );
                        break;
                    } else {
                        println!(
                            "Gap at top right corner: {} - {} = {}",
                            right_x,
                            tractor_beam[top_y].maxx,
                            right_x - tractor_beam[top_y].maxx
                        )
                    }
                }

                // Tractor beam rows always progress to the right as you go down, so
                // we can just set i to be the beginning of the previous beam
                i = tractor_beam[tractor_beam.len() - 1].minx;
                j += 1;
            }
        }
    }
}
