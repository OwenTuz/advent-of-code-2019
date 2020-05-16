#[test]
fn test_get_initial_fuel_requirements() {
    assert_eq!(2, get_initial_fuel_requirements(12));
    assert_eq!(2, get_initial_fuel_requirements(14));
    assert_eq!(654, get_initial_fuel_requirements(1969));
    assert_eq!(33583, get_initial_fuel_requirements(100756));
}
fn get_initial_fuel_requirements(mass: i32) -> i32 {
    // Fuel required to launch a given module is based on its mass.
    // Specifically, to find the fuel required for a module, take its mass,
    // divide by three, round down, and subtract 2.

    ( mass / 3 ) - 2
}

#[test]
fn test_get_total_fuel_requirements(){
    assert_eq!(966, get_total_fuel_requirements(1969));
    assert_eq!(2, get_total_fuel_requirements(14));
}
fn get_total_fuel_requirements(mass: i32) -> i32 {
    // Part 2 of this puzzle requires us to perform the same fuel/mass
    // calculation as in part 1, but accounting for the extra mass that the
    // added fuel provides

    let mut total = 0;
    let mut fuel = get_initial_fuel_requirements(mass);

    while fuel > 0 {
        total = total + fuel;
        fuel = get_initial_fuel_requirements(fuel);
    }
    total
}

fn part1(input: &Vec<i32>) -> i32 {
    let res: i32 = input.into_iter().map(|x| get_initial_fuel_requirements(*x)).sum();
    res
}

fn part2(input: &Vec<i32>) -> i32 {
    let res: i32 = input.into_iter().map(|x| get_total_fuel_requirements(*x)).sum();
    res
}

fn main() {
    let input: Vec<i32> = util::input_to_vec_t_fromstr(include_str!("input"), '\n');
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));

}
