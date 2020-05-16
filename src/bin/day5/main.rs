use util::intcode::intcode;

fn part1(input: &Vec<i32>) -> i32 {
    let mut program = input.clone();
    intcode::run_program(&mut program, Some(1))
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut program = input.clone();
    intcode::run_program(&mut program, Some(5))
}

fn main(){
    let input: Vec<i32> = util::input_to_vec_t_fromstr(include_str!("input"),',');
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}
