#[test]
fn test_run_program(){
    assert_eq!(
        vec![2,0,0,0,99],
        run_program(vec![1,0,0,0,99])
    );
    assert_eq!(
        vec![2,4,4,5,99,9801],
        run_program(vec![2,4,4,5,99,0])
    );
    assert_eq!(
        vec![30,1,1,4,2,5,6,0,99],
        run_program(vec![1,1,1,4,99,5,6,0,99])
    );
}
fn run_program(input: &Vec<i32>) -> Vec<i32> {
    // Runs an Intcode program and returns its final state as Vec<i32>
    let mut program = input.clone();
    let mut pos = 0;
    let mut opcode = program[pos];
    while opcode != 99 {
        // Arguments must be usize since we will use them as indices into slice
        let arg1_addr = program[pos + 1] as usize;
        let arg2_addr = program[pos + 2] as usize;
        let dest_addr = program[pos + 3] as usize;

        match opcode {
            1 => { program[dest_addr] = program[arg1_addr] + program [arg2_addr] },
            2 => { program[dest_addr] = program[arg1_addr] * program [arg2_addr] },
            _ => panic!("Not a recognised opcode {}", pos)
        }
        pos += 4;
        opcode = program[pos];
    }
    program
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut program = input.clone();
    program[1] = 12;
    program[2] = 2;
    run_program(&program)[0]
}

fn part2(input: &Vec<i32>) -> i32 {
    // We don't technically need to clone() here as we're always modifying the
    // same part of the input, so don't care what it was originally
    // ...this is still cleaner
    let mut program = input.clone();
    let mut result = 0;
    'outer: for a in 0..99 {
        for b in 0..99 {
            program[1] = a;
            program[2] = b;
            if run_program(&program)[0] == 19690720 {
                result = (100 * a) + b;
                break 'outer
            }
        }
    }
    result
}

fn main() {
    let tape: &str = include_str!("input");
    let input: Vec<i32> = tape.trim()
                              .split(",")
                              .map(|x| x.parse::<i32>().unwrap())
                              .collect::<Vec<i32>>();

    println!("Part 1: Answer is {:?}", part1(&input));
    println!("Part 2: Answer is {:?}", part2(&input));

}
