use std::str::FromStr;
pub mod intcode;

#[test]
fn test_input_to_str_vec(){
    assert_eq!(
        vec!["first","second","third"],
        input_to_str_vec(&"first\nsecond\nthird\n")
    );
}
pub fn input_to_str_vec(input: &str) -> Vec<&str> {
    input.trim()
        .split("\n")
        .collect::<Vec<&str>>()
}

#[test]
fn test_input_to_vec_t_fromstr(){
    let result: Vec<i32> = input_to_vec_t_fromstr("0\n1\n1\n2\n3\n5\n", '\n');
    assert_eq!(
        vec![0,1,1,2,3,5],
        result
    );
    let result: Vec<u64> = input_to_vec_t_fromstr("0\n1\n1\n2\n3\n5\n", '\n');
    assert_eq!(
        vec![0,1,1,2,3,5],
        result
    );
}
// Slightly awkwardly named: this will parse a multi-line input file into
// a vector of any type that implements FromStr
//
// In the context of Advent of Code, this is mostly useful for parsing lists of
// numbers from a string into vectors of appropriate numeric types.
//
// However we can also use it for parsing input directly to our own types,
// provided they implement the FromStr trait
pub fn input_to_vec_t_fromstr<T>(input: &str, separator: char) -> Vec<T>
    where
        T: FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input.trim()
        .split(separator)
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
