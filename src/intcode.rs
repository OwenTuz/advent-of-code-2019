pub mod intcode {
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Opcode {
        Stop,
        Add,
        Mul,
        Mov,
        Out,
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
            _  => { panic!("Invalid/unimplemented opcode {:?}", raw_value) }
        }
    }

    pub fn run_program(input: &Vec<i32>) -> i32 {
        // Runs an Intcode program and returns its final state as Vec<i32>
        let mut program = input.clone();

        let mut pos: usize = 0;
        let mut output: i32 = 0;
        let mut opcode = get_opcode(&program[pos]);

        while opcode != Opcode::Stop {
            if output != 0 {
                panic!("TEST failed: invalid output {:?}", output)
            }

            let mut args: Vec<i32> = get_args(&program, pos);
            let dest: usize = args.pop().unwrap() as usize;

            match opcode {
                Opcode::Add => { program[dest] = args.pop().unwrap() + args.pop().unwrap() },
                Opcode::Mul => { program[dest] = args.pop().unwrap() * args.pop().unwrap() },
                Opcode::Mov => { program[dest] = args.pop().unwrap() },
                Opcode::Out => { output = program[dest] },
                _           => { panic!("Invalid opcode value {:?}", opcode) },
            }
            pos += num_args(opcode) + 1;
            opcode = get_opcode(&program[pos]);
        }
        output
    }

    fn num_args(opcode: Opcode) -> usize {
        match opcode {
            Opcode::Stop => 0,
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Mov => 1,
            Opcode::Out => 1,
        }
    }

    #[test]
    fn test_get_args() {
        let program = &vec![1002,4,3,4,33];
        let expected = vec![33,3,4];
        assert_eq!(get_args(program, 0), expected);
    }
    fn get_args(program: &Vec<i32>, pos: usize) -> Vec<i32> {
        let raw_value = program[pos];
        let opcode = get_opcode(&raw_value);
        if opcode == Opcode::Stop {
            panic!("Called get_args() for opcode STOP, this should never happen")
        }
        let num_args = num_args(opcode);

        let mut mode_digits = raw_value / 100;
        let mut args = Vec::new();

        for i in 1..(num_args + 1) {
            let mode: ParameterMode = match i {
                // Last parameter is destination and is always an address
                // The spec says this is "position mode" since we treat it as an
                // address but we never read the value, so it behaves like
                // immediate mode for our purposes and we override it
                x if x == num_args         => ParameterMode::Immediate,
                _ if mode_digits % 10 == 0 => ParameterMode::Position,
                _ if mode_digits % 10 == 1 => ParameterMode::Immediate,
                _                          => {
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
