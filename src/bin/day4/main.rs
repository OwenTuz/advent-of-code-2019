use std::ops::Range;

#[test]
fn test_is_valid_password_part1(){
    assert!(is_valid_password_part1(&111111));
    assert!(is_valid_password_part1(&111123));

    // Not valid - final digit is less than preceding
    assert!(!is_valid_password_part1(&223450));

    // Not valid - no doubled digit
    assert!(!is_valid_password_part1(&123789));
}
fn is_valid_password_part1(pw: &i32) -> bool {
    let digits: Vec<u32> = pw.to_string()
                             .chars()
                             .map(|x| x.to_digit(10).unwrap())
                             .collect();

    // Bit of a hack, but all() and any() require mutable items on which to
    // operate, so they can't share
    let mut window1 = digits.windows(2);
    let mut window2 = digits.windows(2);

    window1.all(|x| x[0] <= x[1]) && window2.any(|x| x[0] == x[1])
}

#[test]
fn test_is_valid_password_part2(){
    assert!(is_valid_password_part2(&112233));

    // Not valid - more than two adjacent '4's, no other doubled numbers
    assert!(!is_valid_password_part2(&123444));

    // Valid - more than two adjacent '1's but the '2's are only doubled
    assert!(is_valid_password_part2(&111122));
}
fn is_valid_password_part2(pw: &i32) -> bool {
    let digits: Vec<u32> = pw.to_string()
                             .chars()
                             .map(|x| x.to_digit(10).unwrap())
                             .collect();

    digits.windows(2).all(|x| x[0] <= x[1]) &&
    has_any_entry_repeated_exactly_once(digits)
}

#[test]
fn test_has_any_entry_repeated_exactly_once() {
    assert!(has_any_entry_repeated_exactly_once(vec![1,1,2,3,4,5]));
    assert!(has_any_entry_repeated_exactly_once(vec![1,2,3,4,5,5]));
    assert!(has_any_entry_repeated_exactly_once(vec![1,2,3,3,4,5]));

    assert!(!has_any_entry_repeated_exactly_once(vec![1,2,3,4,5,6]));
    assert!(!has_any_entry_repeated_exactly_once(vec![1,1,1,2,3,4]));
    assert!(!has_any_entry_repeated_exactly_once(vec![1,2,3,4,4,4]));

    assert!(has_any_entry_repeated_exactly_once(vec![1,1,2,2,3,4]));
    assert!(has_any_entry_repeated_exactly_once(vec![1,1,1,2,2,5]));
}
fn has_any_entry_repeated_exactly_once(digits: Vec<u32>) -> bool{
    let mut count = 0;
    for i in 1..6 {
        if digits[i] == digits[i-1] {
            count += 1;
        } else {
            if count == 1 {
                return true
            }
            count = 0;
        }
    }
    count == 1
}

fn part1(range: Range<i32>) -> usize {
    range.filter(|x| is_valid_password_part1(x)).count()
}

fn part2(range: Range<i32>) -> usize {
    range.filter(|x| is_valid_password_part2(x)).count()
}

fn main(){
    println!("Part 1: Answer is {}", part1(206938..679128));
    println!("Part 2: Answer is {}", part2(206938..679128));
}
