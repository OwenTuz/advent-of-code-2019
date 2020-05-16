pub mod intcode {
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Opcode {
        Stop,
        Add,
        Mul,
        Mov,
        Out,
        Jnz,
        Jz,
        Lt,
        Eq,
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum ParameterMode {
        Position,
        Immediate,
    }

    #[test]
    fn test_get_opcode() {
        assert_eq!(get_opcode(&12304), Opcode::Out);
        assert_eq!(get_opcode(&12301), Opcode::Add);
        assert_eq!(get_opcode(&12399), Opcode::Stop);
    }
    fn get_opcode(raw_value: &i32) -> Opcode {
        match raw_value % 100 {
            99 => Opcode::Stop,
            1  => Opcode::Add,
            2  => Opcode::Mul,
            3  => Opcode::Mov,
            4  => Opcode::Out,
            5  => Opcode::Jnz,
            6  => Opcode::Jz,
            7  => Opcode::Lt,
            8  => Opcode::Eq,
            _  => { panic!("Invalid/unimplemented opcode {:?}", raw_value) }
        }
    }

    pub fn run_program(tape: &Vec<i32>, input: Option<i32>) -> i32 {
        // Runs an Intcode program and returns its final state as Vec<i32>
        let mut program = tape.clone();

        let mut pos: usize = 0;
        let mut output: i32 = 0;
        let mut opcode = get_opcode(&program[pos]);

        while opcode != Opcode::Stop {
            if output != 0 {
                panic!("TEST failed: invalid output {:?}", output)
            }

            match run_instruction(&opcode, &mut program, &mut pos, input) {
                Some(x) => { output = x },
                None    => {},
            }
            opcode = get_opcode(&program[pos]);
        }
        output
    }

    // Given an opcode, mutable reference to a program, and current position within that program:
    // mutates the program according the instruction given
    // If there is an output from the instruction, returns Some(output)
    // otherwise returns None
    fn run_instruction(opcode: &Opcode, program: &mut Vec<i32>, pos: &mut usize, input: Option<i32>) -> Option<i32> {
        let mut args: Vec<i32> = get_args(&program, &pos);

        let dest: usize = match writes_to_program(opcode) {
            true  => args.pop().unwrap() as usize,
            false => 0,
        };
        let mut pos_changed = false;

        let mut output: Option<i32> = None;
        match opcode {
            Opcode::Add => { program[dest] = args[0] + args[1] },
            Opcode::Mul => { program[dest] = args[0] * args[1] },
            Opcode::Mov => { program[dest] = input.unwrap() },
            Opcode::Out => { output = args.pop() },
            Opcode::Jnz => {
                if args[0] != 0 {
                    *pos = args[1] as usize;
                    pos_changed = true;
                }
            },
            Opcode::Jz  => {
                if args[0] == 0 {
                    *pos = args[1] as usize;
                    pos_changed = true;
                }
            },
            Opcode::Lt  => {
                if args[0] < args[1] {
                    program[dest] = 1
                } else {
                    program[dest] = 0
                }
            },
            Opcode::Eq  => {
                if args[0] == args[1] {
                    program[dest] = 1
                } else {
                    program[dest] = 0
                }
            },
            _ => { panic!("Invalid opcode value {:?}", opcode) },
        }
        if !pos_changed {
            *pos = *pos + num_args(&opcode) + 1;
        }
        return output
    }

    fn num_args(opcode: &Opcode) -> usize {
        match opcode {
            Opcode::Stop => 0,
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Mov => 1,
            Opcode::Out => 1,
            Opcode::Jnz => 2,
            Opcode::Jz  => 2,
            Opcode::Lt  => 3,
            Opcode::Eq  => 3,
        }
    }

    fn writes_to_program(opcode: &Opcode) -> bool {
        let writers = vec![
            Opcode::Add,
            Opcode::Mul,
            Opcode::Mov,
            Opcode::Lt,
            Opcode::Eq,
        ];
        if writers.contains(opcode) {
            return true
        }
        false
    }

    #[test]
    fn test_get_args() {
        let program = &vec![1002,4,3,4,33];
        let expected = vec![33,3,4];
        assert_eq!(get_args(program, &0), expected);
    }
    fn get_args(program: &Vec<i32>, pos: &usize) -> Vec<i32> {
        let raw_value = program[*pos];
        let opcode = get_opcode(&raw_value);
        if opcode == Opcode::Stop {
            panic!("Called get_args() for opcode STOP, this should never happen")
        }
        let num_args = num_args(&opcode);

        let mut mode_digits = raw_value / 100;
        let mut args = Vec::new();

        for i in 1..(num_args + 1) {
            let mode: ParameterMode = match i {
                // Any instruction that writes to a location simply treats the
                // last argument as a destination address.
                // The spec says this is "position" mode but since we use it as
                // an index after reading, we actually treat it as immediate
                x if x == num_args && writes_to_program(&opcode) => ParameterMode::Immediate,
                _ if mode_digits % 10 == 0                       => ParameterMode::Position,
                _ if mode_digits % 10 == 1                       => ParameterMode::Immediate,
                _                                                => {
                    panic!("Unsupported parameter mode indicator {:?}", mode_digits % 10)
                },
            };
            match mode {
                ParameterMode::Position => {
                    let arg_addr = program[pos + i] as usize;
                    args.push(program[arg_addr]);
                },
                ParameterMode::Immediate => {
                    args.push(program[pos + i]);
                },
            }
            mode_digits = mode_digits / 10
        }
        args
    }
}
