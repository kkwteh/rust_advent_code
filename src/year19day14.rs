#[cfg(test)]
mod year19day14 {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Clone)]
    struct Transform {
        output: Chemical,
        inputs: Vec<Chemical>,
    }
    #[derive(Debug, Clone, PartialEq, Hash)]
    struct Chemical {
        symbol: &'static str,
        quantity: i64,
    }
    impl Eq for Chemical {}

    fn topological_sort(
        mut graph: HashMap<&'static str, HashSet<&'static str>>,
    ) -> Vec<&'static str> {
        // Topological sort algorithm https://en.wikipedia.org/wiki/Topological_sorting
        // L ← Empty list that will contain the sorted elements
        // S ← Set of all nodes with no incoming edge

        // while S is not empty do
        //     remove a node n from S
        //     add n to tail of L
        //     for each node m with an edge e from n to m do
        //         remove edge e from the graph
        //         if m has no other incoming edges then
        //             insert m into S

        // if graph has edges then
        //     return error   (graph has at least one cycle)
        // else
        //     return L   (a topologically sorted order)
        let mut result = Vec::<&'static str>::new();
        let mut no_incoming: Vec<&'static str> = vec!["FUEL"];
        while no_incoming.len() > 0 {
            let symbol = no_incoming.pop().unwrap();
            result.push(symbol);
            for (node, incoming_set) in &mut graph {
                if incoming_set.remove(symbol) {
                    if incoming_set.len() == 0 {
                        no_incoming.push(node);
                    }
                }
            }
        }
        result
    }

    fn read_input(
        input: &'static str,
    ) -> (
        HashMap<&'static str, HashSet<&'static str>>,
        HashMap<&'static str, Transform>,
    ) {
        let mut graph = HashMap::<&str, HashSet<&str>>::new();
        let mut transforms = HashMap::<&str, Transform>::new();
        for line in (*input).split("\n") {
            let v: Vec<&str> = line.split("=>").collect();
            let left = v[0].trim();
            let right = v[1].trim();

            let output_v: Vec<&str> = right.split(' ').collect();
            let output = Chemical {
                symbol: output_v[1],
                quantity: output_v[0].parse::<i64>().unwrap(),
            };

            let left_v: Vec<&str> = left.split(", ").collect();
            let mut inputs = Vec::<Chemical>::new();
            for token in left_v {
                let input_v: Vec<&str> = token.split(' ').collect();
                let input = Chemical {
                    symbol: input_v[1],
                    quantity: input_v[0].parse::<i64>().unwrap(),
                };
                inputs.push(input.clone());
                if !graph.contains_key(input.symbol) {
                    graph.insert(input.symbol, HashSet::<&str>::new());
                }
                if let Some(nodes) = graph.get_mut(input.symbol) {
                    nodes.insert(output.symbol);
                }
            }
            transforms.insert(output.symbol, Transform { output, inputs });
        }
        (graph, transforms)
    }

    #[test]
    fn int_division() {
        let x: i64 = 5;
        let y: i64 = 2;
        assert_eq!(x / y, 2);
    }

    fn compute_ore_needed(problem_text: &'static str) -> i64 {
        // let readresult: &'static str = year19day14input;
        let (graph, transforms) = read_input(&problem_text[..]);
        let sorted_symbols = topological_sort(graph);
        let mut needed = HashMap::<&str, i64>::new();
        needed.insert("FUEL", 1);
        println!("sorted symbols {:?}", sorted_symbols);
        for symbol in sorted_symbols {
            if symbol == "ORE" {
                return needed.remove(symbol).unwrap();
            }
            match transforms.get(symbol) {
                Some(transform) => match needed.get(symbol) {
                    Some(needed_quantity) => {
                        println!("Need {} more of {}", needed_quantity, symbol);
                        let coefficient = (*needed_quantity as f64
                            / transform.output.quantity as f64)
                            .ceil() as i64;
                        for input in &transform.inputs {
                            if needed.get(input.symbol) == None {
                                needed.insert(input.symbol, 0);
                            }
                            println!(
                                "=> Need {} more of {}",
                                input.quantity * coefficient,
                                input.symbol
                            );
                            *needed.get_mut(input.symbol).unwrap() += input.quantity * coefficient;
                        }
                        needed.insert(symbol, 0);
                    }
                    None => panic!("Couldn't find needed chemical for {}", symbol),
                },
                None => panic!("Couldn't find transform for {}", symbol),
            }
        }
        panic!("Shouldn't make it to the end of the function body");
    }

    #[test]
    fn example_one() {
        let problem_text: &'static str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        assert_eq!(compute_ore_needed(problem_text), 31);
    }

    #[test]
    fn example_two() {
        let problem_text: &'static str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        assert_eq!(compute_ore_needed(problem_text), 165);
    }

    #[test]
    fn example_three() {
        let problem_text: &'static str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(compute_ore_needed(problem_text), 13312);
    }

    #[test]
    fn example_four() {
        let problem_text: &'static str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        assert_eq!(compute_ore_needed(problem_text), 180697);
    }

    #[test]
    fn example_five() {
        let problem_text: &'static str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        assert_eq!(compute_ore_needed(problem_text), 2210736);
    }

    #[test]
    fn day_fourteen_part_one_challenge() {
        let problem_text: &'static str = "11 BNMWF, 1 MRVFT, 10 PBNSF => 7 XSFVQ
149 ORE => 4 SMSB
1 XHQDX, 1 SVSTJ, 2 LDHX => 7 JMWQG
12 MJCLX => 9 PBNSF
132 ORE => 7 XPTXL
15 TZMWG, 1 LDHX, 1 PDVR => 7 LBQB
1 HJTD, 8 VFXHC => 2 SVSTJ
5 LBHQ, 6 MTQCB => 4 MHBZ
1 PRXT, 1 FWZN => 2 PBMPL
1 XPTXL => 1 HMRGM
10 XHPHR => 6 NSVJL
3 QZQLZ, 3 MTQCB => 4 TZMWG
5 LBHQ, 2 VPSDV => 3 ZFCD
13 WPFP => 6 ZXMGK
10 MHJMX, 75 LDHX, 52 JMWQG, 4 QWRB, 1 SVNVJ, 17 BNMWF, 18 GHVN => 1 FUEL
4 PFQRG, 14 XVNL => 5 PDCV
11 JMWQG, 10 ZBNCP => 6 NTJZH
14 PBMPL, 12 PRXT, 9 MJQS => 9 XVNL
9 GDNG, 13 LBQB => 9 QWRB
1 CXNM => 6 PFQRG
9 NTJZH, 7 BNMWF, 11 JCHP, 1 MHBZ, 1 SVSTJ, 9 XRDN => 5 SVNVJ
1 XHPHR, 1 GSMP => 4 THRVR
26 FWZN => 4 WPFP
35 VJTFJ, 2 XSFVQ, 6 HJVN, 1 NSVJL, 1 JCHP, 3 MJCLX, 1 QZNCK => 6 GHVN
1 WPFP, 3 XHPHR => 2 HJVN
5 SMSB => 7 HNCDS
111 ORE => 4 GSMP
6 LBHQ => 8 GDNG
2 GDNG, 5 MHBZ => 1 RNMKC
15 THRVR, 4 NWNSH, 1 NSVJL => 7 FDVH
2 HMRGM => 9 FWZN
6 MJQS, 5 JRZXM => 5 NWNSH
14 ZXMGK, 1 JTXWX => 6 DLWT
1 MJQS, 3 FWZN, 2 PRXT => 1 JTXWX
1 GSMP, 4 CXNM => 3 JRZXM
151 ORE => 9 ZNPRL
2 NTJZH, 1 DLWT, 3 ZBNCP => 9 MRVFT
14 SWZCB, 1 VPSDV => 7 XRDN
14 LBHQ, 16 FDVH, 9 PFQRG => 4 PRXT
22 CXNM => 9 HJTD
1 VFXHC, 1 MTQCB => 6 QZQLZ
6 SWZCB, 2 PDCV, 17 RNMKC => 9 LTHFW
4 ZNPRL => 6 CXNM
2 CXNM => 3 LBHQ
8 MHBZ, 2 QZQLZ, 2 LBQB => 3 VJTFJ
3 ZFCD => 1 XHQDX
1 VJTFJ, 7 MHBZ => 8 ZBNCP
5 CXNM => 2 VPSDV
7 MJQS => 9 VFXHC
2 LTHFW, 11 HJVN, 4 XRDN, 8 MRVFT, 3 NSVJL, 3 SVSTJ, 5 XSFVQ, 13 RNMKC => 8 MHJMX
2 HMRGM => 3 XHPHR
1 GDNG, 19 PDVR => 3 SWZCB
18 HMRGM, 10 HNCDS => 2 MJQS
6 HNCDS, 2 HMRGM, 1 LBHQ => 3 MTQCB
16 VJTFJ, 1 WPFP, 6 JMWQG => 6 BNMWF
3 TZMWG, 1 FWZN => 7 PDVR
10 ZXMGK => 4 QZNCK
32 LBQB, 1 ZBNCP => 1 JCHP
27 PDVR, 7 QZQLZ, 7 PBMPL => 3 MJCLX
5 MHBZ, 12 ZFCD => 4 LDHX";
        println!(
            "THE ANSWER TO DAY FOURTEEN PART ONE IS {}",
            compute_ore_needed(problem_text)
        );
    }

    fn returns_str_reference() -> &'static str {
        "asdf"
    }
    #[test]
    fn test_str_reference() {
        assert_eq!(returns_str_reference(), "asdf");
    }

    #[test]
    fn day_fourteen_part_one_example() {
        // Algorithm:
        // Create graph linking outputs to each input
        // For compound in topological sort convert output to inputs (starting with 1 FUEL)
        // Count how much ORE there is
    }
}
