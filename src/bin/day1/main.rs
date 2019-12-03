#[test]
fn test_get_fuel_requirements() {
    assert_eq!(2, get_fuel_requirements(12));
    assert_eq!(2, get_fuel_requirements(14));
    assert_eq!(654, get_fuel_requirements(1969));
    assert_eq!(33583, get_fuel_requirements(100756));
}

fn get_fuel_requirements(mass: i32) -> i32 {
    // Fuel required to launch a given module is based on its mass.
    // Specifically, to find the fuel required for a module, take its mass,
    // divide by three, round down, and subtract 2.

    ( mass / 3 ) - 2
}

fn part1(input: &Vec<i32>) -> i32 {
    let res: i32 = input.into_iter().map(|x| get_fuel_requirements(*x)).sum();
    res
}

fn main() {
    let input: Vec<i32> = util::input_to_vec_t_fromstr(include_str!("input"));
    println!("Part 1: Answer is {}", part1(&input));

}
