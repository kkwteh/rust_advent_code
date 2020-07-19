#[cfg(test)]
mod year19day12 {
    use smallvec::{smallvec, SmallVec};
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Physics {
        position: SmallVec<[i16; 3]>,
        velocity: SmallVec<[i16; 3]>,
    }
    impl Eq for Physics {}

    fn energy(physics: &SmallVec<[Physics; 4]>) -> i64 {
        let mut result = 0;
        for physics_elem in physics {
            let mut kin = 0;
            let mut pot = 0;
            for dim in 0..3 {
                pot += physics_elem.position[dim].abs() as i64;
                kin += physics_elem.velocity[dim].abs() as i64;
            }
            result += kin * pot;
            println!("kin {} pot {} total {}", kin, pot, kin * pot);
        }
        result
    }

    fn update_physics(physics: &mut SmallVec<[Physics; 4]>) {
        for i in 0..physics.len() {
            for j in i + 1..physics.len() {
                for dim in 0..3 {
                    if physics[i].position[dim] < physics[j].position[dim] {
                        physics[i].velocity[dim] += 1;
                        physics[j].velocity[dim] -= 1;
                    } else if physics[i].position[dim] > physics[j].position[dim] {
                        physics[i].velocity[dim] -= 1;
                        physics[j].velocity[dim] += 1;
                    }
                }
            }
        }

        for i in 0..physics.len() {
            for dim in 0..3 {
                physics[i].position[dim] += physics[i].velocity[dim];
            }
        }
    }

    fn reverse_physics(physics: &mut SmallVec<[Physics; 4]>) {
        // Since the physics are reversible, the first looped point should match the original state.
        // Therefore there shouldn't be any need to use this function.
        for i in 0..physics.len() {
            for dim in 0..3 {
                physics[i].position[dim] -= physics[i].velocity[dim];
            }
        }

        for i in 0..physics.len() {
            for j in 0..physics.len() {
                if i < j {
                    for dim in 0..3 {
                        if physics[i].position[dim] < physics[j].position[dim] {
                            physics[i].velocity[dim] -= 1;
                            physics[j].velocity[dim] += 1;
                        } else if physics[i].position[dim] > physics[j].position[dim] {
                            physics[i].velocity[dim] += 1;
                            physics[j].velocity[dim] -= 1;
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn day_twelve_example() {
        let mut physics = smallvec![
            Physics {
                position: smallvec![-8, -10, 0],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![5, 5, 10],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![2, -7, 3],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![9, -8, -3],
                velocity: smallvec![0, 0, 0],
            },
        ];
        for _step in 0..100 {
            if _step % 10 == 0 {
                println!("Step {}", _step);
                println!("{:?}", physics);
            }
            update_physics(&mut physics);
            if _step == 99 {
                println!("Step {}", _step);
                println!("{:?}", physics);
                assert_eq!(1940, energy(&physics));
            }
        }
    }

    #[test]
    fn day_twelve_part_one_challenge() {
        let mut physics = smallvec![
            Physics {
                position: smallvec![13, -13, -2],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![16, 2, -15],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![7, -18, -12],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![-3, -8, -8],
                velocity: smallvec![0, 0, 0],
            },
        ];
        for _step in 0..1000 {
            update_physics(&mut physics);
        }
        println!("THE ANSWER TO DAY TWELVE PART ONE IS {}", energy(&physics));
    }

    // Comment out test because it is super long
    // #[test]
    fn day_twelve_part_two() {
        let original_physics: smallvec::SmallVec<[Physics; 4]> = smallvec![
            Physics {
                position: smallvec![13, -13, -2],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![16, 2, -15],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![7, -18, -12],
                velocity: smallvec![0, 0, 0],
            },
            Physics {
                position: smallvec![-3, -8, -8],
                velocity: smallvec![0, 0, 0],
            },
        ];
        // Small example
        // let mut physics = smallvec![
        //     Physics {
        //         position: smallvec![-1, 0, 2],
        //         velocity: smallvec![0, 0, 0],
        //     },
        //     Physics {
        //         position: smallvec![2, -10, -7],
        //         velocity: smallvec![0, 0, 0],
        //     },
        //     Physics {
        //         position: smallvec![4, -8, 8],
        //         velocity: smallvec![0, 0, 0],
        //     },
        //     Physics {
        //         position: smallvec![3, 5, -1],
        //         velocity: smallvec![0, 0, 0],
        //     },
        // ];
        // Very long example
        // let mut physics = smallvec![
        //     Physics {
        //         position: smallvec![-8, -10, 0],
        //         velocity: smallvec![0, 0, 0],
        //     },
        //     Physics {
        //         position: smallvec![5, 5, 10],
        //         velocity: smallvec![0, 0, 0],
        //     },
        //     Physics {
        //         position: smallvec![2, -7, 3],
        //         velocity: smallvec![0, 0, 0],
        //     },
        //     Physics {
        //         position: smallvec![9, -8, -3],
        //         velocity: smallvec![0, 0, 0],
        //     },
        // ];
        // 4686774924
        // let mut key_step_physics = SmallVec::<[SmallVec<[Physics; 4]>; 4]>::new();
        let mut physics: smallvec::SmallVec<[Physics; 4]> = smallvec![
            Physics {
                position: smallvec![206, 446, 84],
                velocity: smallvec![-19, 29, -23]
            },
            Physics {
                position: smallvec![-748, 129, 280],
                velocity: smallvec![-27, -48, -23]
            },
            Physics {
                position: smallvec![322, -206, 111],
                velocity: smallvec![16, 4, -8]
            },
            Physics {
                position: smallvec![253, -406, -512],
                velocity: smallvec![30, 15, 54]
            }
        ];
        let mut step: i64 = 46_000_000_000;
        while step < 100_000_000_001 {
            if step % 1_000_000_000 == 0 {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open("stepreached.txt")
                    .unwrap();
                writeln!(
                    file,
                    "reached step index {}, physics is {:?}",
                    step, physics
                );
            }
            update_physics(&mut physics);
            if physics == original_physics {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open("day12part2.txt")
                    .unwrap();
                writeln!(file, "found duplicate physics at step {}", step + 1);
                return;
            }
            step += 1;
        }
    }
}
