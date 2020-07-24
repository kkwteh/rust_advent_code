pub mod intcode {
    enum OpType {
        Add,
        Mult,
        _Copy,
        Output,
        Halt,
        JumpIfTrue,
        JumpIfFalse,
        LessThan,
        Equals,
        AddRelativeBase,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum ParameterMode {
        Position,
        Immediate,
        Relative,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramContext {
        program: Vec<String>,
        pub inputs: Vec<String>,
        pub outputs: Vec<String>,
        pcursor: usize,
        icursor: usize,
        pub done: bool,
        parameter_modes: Vec<ParameterMode>,
        relative_base: i64,
    }

    impl ProgramContext {
        pub fn run_to_next_input_or_done(&mut self) {
            while !self.done {
                let opcode = unpack_opcode(&self.program[self.pcursor]);
                self.parameter_modes = opcode.parameter_modes;
                if let Err(_e) = (opcode.operator)(self) {
                    return;
                };
            }
            return;
        }
    }

    pub fn build_program_context(program: Vec<String>, input: Vec<String>) -> ProgramContext {
        ProgramContext {
            program: program,
            inputs: input,
            outputs: Vec::<String>::new(),
            pcursor: 0,
            icursor: 0,
            done: false,
            parameter_modes: Vec::<ParameterMode>::new(),
            relative_base: 0,
        }
    }

    struct OpCode {
        operator: fn(&mut ProgramContext) -> Result<(), i64>,
        parameter_modes: Vec<ParameterMode>,
        op_type: OpType,
    }

    fn read_parameter(ctx: &ProgramContext, ix: usize) -> i64 {
        match ctx.parameter_modes[ix] {
            ParameterMode::Position => {
                let position = ctx.program[ctx.pcursor + ix + 1].parse::<usize>().unwrap();
                match position >= ctx.program.len() {
                    true => 0,
                    false => ctx.program[position].parse::<i64>().unwrap(),
                }
            }
            ParameterMode::Immediate => match ctx.pcursor + ix + 1 >= ctx.program.len() {
                true => 0,
                false => ctx.program[ctx.pcursor + ix + 1].parse::<i64>().unwrap(),
            },
            ParameterMode::Relative => {
                let position = ctx.program[ctx.pcursor + ix + 1].parse::<i64>().unwrap();
                match position + ctx.relative_base >= ctx.program.len() as i64 {
                    true => 0,
                    false => ctx.program[(position + ctx.relative_base) as usize]
                        .parse::<i64>()
                        .unwrap(),
                }
            }
        }
    }

    fn get_write_position(ctx: &ProgramContext, ix: usize) -> usize {
        match ctx.parameter_modes[ix] {
            ParameterMode::Position => ctx.program[ctx.pcursor + ix + 1].parse::<usize>().unwrap(),
            ParameterMode::Immediate => panic!("Encountered immediate mode for _Copy operator"),
            ParameterMode::Relative => {
                (ctx.program[ctx.pcursor + ix + 1].parse::<i64>().unwrap() + ctx.relative_base)
                    as usize
            }
        }
    }

    fn add(ctx: &mut ProgramContext) -> Result<(), i64> {
        let operand_a = read_parameter(&ctx, 0);
        let operand_b = read_parameter(&ctx, 1);
        let operand_c = get_write_position(&ctx, 2);
        write_program(
            &mut ctx.program,
            operand_c,
            (operand_a + operand_b).to_string(),
        );
        ctx.pcursor += 4;
        Ok(())
    }

    fn mult(ctx: &mut ProgramContext) -> Result<(), i64> {
        let operand_a = read_parameter(&ctx, 0);
        let operand_b = read_parameter(&ctx, 1);
        let operand_c = get_write_position(&ctx, 2);
        write_program(
            &mut ctx.program,
            operand_c,
            (operand_a * operand_b).to_string(),
        );
        ctx.pcursor += 4;
        Ok(())
    }

    fn _copy(ctx: &mut ProgramContext) -> Result<(), i64> {
        if ctx.icursor >= ctx.inputs.len() {
            return Err(1);
        }
        let write_position = get_write_position(&ctx, 0);
        write_program(
            &mut ctx.program,
            write_position,
            ctx.inputs[ctx.icursor].clone(),
        );
        ctx.icursor += 1;
        ctx.pcursor += 2;
        Ok(())
    }

    fn output(ctx: &mut ProgramContext) -> Result<(), i64> {
        let operand = read_parameter(&ctx, 0);
        ctx.outputs.push(operand.to_string());
        ctx.pcursor += 2;
        Ok(())
    }

    fn halt(ctx: &mut ProgramContext) -> Result<(), i64> {
        ctx.done = true;
        Ok(())
    }

    fn jump_if_true(ctx: &mut ProgramContext) -> Result<(), i64> {
        let operand_a = read_parameter(&ctx, 0);
        let operand_b = read_parameter(&ctx, 1) as usize;
        if operand_a != 0 {
            ctx.pcursor = operand_b;
        } else {
            ctx.pcursor += 3;
        }
        Ok(())
    }
    fn jump_if_false(ctx: &mut ProgramContext) -> Result<(), i64> {
        let operand_a = read_parameter(&ctx, 0);
        let operand_b = read_parameter(&ctx, 1) as usize;
        if operand_a == 0 {
            ctx.pcursor = operand_b;
        } else {
            ctx.pcursor += 3;
        }
        Ok(())
    }
    fn less_than(ctx: &mut ProgramContext) -> Result<(), i64> {
        let operand_a = read_parameter(&ctx, 0);
        let operand_b = read_parameter(&ctx, 1);
        let operand_c = get_write_position(&ctx, 2);
        if operand_a < operand_b {
            write_program(&mut ctx.program, operand_c, "1".to_string())
        } else {
            write_program(&mut ctx.program, operand_c, "0".to_string())
        }
        ctx.pcursor += 4;
        Ok(())
    }
    fn equals(ctx: &mut ProgramContext) -> Result<(), i64> {
        let operand_a = read_parameter(&ctx, 0);
        let operand_b = read_parameter(&ctx, 1);
        let operand_c = get_write_position(&ctx, 2);
        if operand_a == operand_b {
            write_program(&mut ctx.program, operand_c, "1".to_string())
        } else {
            write_program(&mut ctx.program, operand_c, "0".to_string())
        }
        ctx.pcursor += 4;
        Ok(())
    }

    fn write_program(program: &mut Vec<String>, index: usize, value: String) {
        if index >= program.len() {
            program.resize(index + 1, "0".to_string());
        }
        program[index] = value;
    }

    fn add_relative_base(ctx: &mut ProgramContext) -> Result<(), i64> {
        let operand = read_parameter(&ctx, 0);
        ctx.relative_base += operand;
        ctx.pcursor += 2;
        Ok(())
    }

    pub fn read_tokens(tokens: &str) -> Vec<String> {
        let mut result = Vec::new();

        for token in (*tokens).split(",") {
            result.push(token.to_string());
        }
        result
    }

    pub fn run_program(program: Vec<String>, input: Vec<String>) -> Vec<String> {
        let mut ctx = ProgramContext {
            program: program,
            inputs: input,
            outputs: Vec::<String>::new(),
            pcursor: 0,
            icursor: 0,
            done: false,
            parameter_modes: Vec::<ParameterMode>::new(),
            relative_base: 0,
        };
        while !ctx.done {
            let opcode = unpack_opcode(&ctx.program[ctx.pcursor]);
            ctx.parameter_modes = opcode.parameter_modes;
            if let Err(_e) = (opcode.operator)(&mut ctx) {
                panic!("Unexpected error during operation.")
            };
        }
        ctx.outputs.clone()
    }

    pub fn run_multiprogram(program: Vec<String>, inputs: Vec<Vec<String>>) -> i64 {
        let mut ctxs = Vec::<ProgramContext>::new();
        for input in inputs {
            ctxs.push(ProgramContext {
                program: program.clone(),
                inputs: input,
                outputs: Vec::<String>::new(),
                pcursor: 0,
                icursor: 0,
                done: false,
                parameter_modes: Vec::<ParameterMode>::new(),
                relative_base: 0,
            });
        }

        let mut ctx_cursor = 0;
        loop {
            let opcode = unpack_opcode(&ctxs[ctx_cursor].program[ctxs[ctx_cursor].pcursor]);
            ctxs[ctx_cursor].parameter_modes = opcode.parameter_modes;
            match (opcode.operator)(&mut ctxs[ctx_cursor]) {
                Ok(()) => {
                    if let OpType::Output = opcode.op_type {
                        // Operation was an output, need to add it to input of next program
                        let latest_output = ctxs[ctx_cursor].outputs.last().unwrap().clone();
                        let next_ctx_cursor: usize;
                        if ctx_cursor < 4 {
                            next_ctx_cursor = ctx_cursor + 1;
                        } else {
                            next_ctx_cursor = 0;
                        }
                        ctxs[next_ctx_cursor].inputs.push(latest_output);
                    }
                    if let OpType::Halt = opcode.op_type {
                        // Program halted, need to advance to next machine
                        if ctx_cursor < 4 {
                            ctx_cursor = ctx_cursor + 1;
                        } else {
                            break;
                        }
                    }
                }
                Err(_err_val) => {
                    // Can't continue without a new input, moving to run next program
                    if ctx_cursor < 4 {
                        ctx_cursor += 1;
                    } else {
                        ctx_cursor = 0;
                    }
                }
            }
        }
        ctxs[4]
            .outputs
            .last()
            .unwrap()
            .to_string()
            .parse::<i64>()
            .unwrap()
    }

    fn unpack_opcode(opcode_string: &String) -> OpCode {
        // lpad with 0's to length 5
        let padded_opcode = format!("{:0>5}", opcode_string);
        let mut parameter_modes = Vec::<ParameterMode>::new();
        for c in padded_opcode[..3].chars().rev() {
            match c {
                '0' => parameter_modes.push(ParameterMode::Position),
                '1' => parameter_modes.push(ParameterMode::Immediate),
                '2' => parameter_modes.push(ParameterMode::Relative),
                _ => panic!("Unexpected parameter mode {}", c),
            }
        }
        let opcode_number = padded_opcode[3..].parse::<i64>().unwrap();
        match opcode_number {
            1 => OpCode {
                operator: add,
                parameter_modes: parameter_modes,
                op_type: OpType::Add,
            },
            2 => OpCode {
                operator: mult,
                parameter_modes: parameter_modes,
                op_type: OpType::Mult,
            },
            3 => OpCode {
                operator: _copy,
                parameter_modes: parameter_modes,
                op_type: OpType::_Copy,
            },
            4 => OpCode {
                operator: output,
                parameter_modes: parameter_modes,
                op_type: OpType::Output,
            },
            5 => OpCode {
                operator: jump_if_true,
                parameter_modes: parameter_modes,
                op_type: OpType::JumpIfTrue,
            },
            6 => OpCode {
                operator: jump_if_false,
                parameter_modes: parameter_modes,
                op_type: OpType::JumpIfFalse,
            },
            7 => OpCode {
                operator: less_than,
                parameter_modes: parameter_modes,
                op_type: OpType::LessThan,
            },
            8 => OpCode {
                operator: equals,
                parameter_modes: parameter_modes,
                op_type: OpType::Equals,
            },
            9 => OpCode {
                operator: add_relative_base,
                parameter_modes: parameter_modes,
                op_type: OpType::AddRelativeBase,
            },
            99 => OpCode {
                operator: halt,
                parameter_modes: parameter_modes,
                op_type: OpType::Halt,
            },
            _ => panic!("Unexpected opcode input {}", opcode_string),
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_run_to_next_input_or_done() {
            let tokens = "3,9,8,9,10,9,4,9,99,-1,8";
            let program = read_tokens(tokens);
            let input = vec!["8".to_string()];
            let mut ctx = build_program_context(program, input);
            ctx.run_to_next_input_or_done();
            assert_eq!(ctx.outputs[0], "1".to_string());
            assert_eq!(ctx.done, true);
        }
        #[test]
        fn test_run_to_next_input_or_done_need_input() {
            let tokens = "3,9,8,9,10,9,4,9,99,-1,8";
            let program = read_tokens(tokens);
            let mut ctx = build_program_context(program, Vec::<String>::new());
            ctx.run_to_next_input_or_done();
            assert_eq!(ctx.done, false);
            ctx.inputs.push("8".to_string());
            ctx.run_to_next_input_or_done();
            assert_eq!(ctx.done, true);
            assert_eq!(ctx.outputs[0], "1".to_string());
        }

        #[test]
        fn module_path() {
            println!(module_path!())
        }

        #[test]
        fn test_unpack_opcode() {
            let result = unpack_opcode(&"1102".to_string());
            assert_eq!(
                result.parameter_modes,
                vec![
                    ParameterMode::Immediate,
                    ParameterMode::Immediate,
                    ParameterMode::Position,
                ]
            );
        }

        #[test]
        fn test_unpack_opcode_weird_case() {
            let result = unpack_opcode(&"104".to_string());
            assert_eq!(
                result.parameter_modes,
                vec![
                    ParameterMode::Immediate,
                    ParameterMode::Position,
                    ParameterMode::Position,
                ]
            );
        }
    }
}
