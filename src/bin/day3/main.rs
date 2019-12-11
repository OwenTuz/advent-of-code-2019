// Today's answer contains a lot of unncessary boilerplate
// But I don't get to write Rust very often so it was fun to try out the various
// traits etc

#[macro_use]
extern crate simple_error;

use simple_error::SimpleError;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone,Copy,Debug,PartialEq)]
enum Direction {
    U,
    D,
    L,
    R
}

impl FromStr for Direction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => bail!("Invalid direction")
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
struct GridVector {
    // A vector with magnitude (distance) and direction
    // Name to avoid confusion with std::vec::Vec
    origin: Point,
    direction: Direction,
    distance: i32,
    _iter_state: Point
}

impl GridVector {
    fn new(origin: Point, direction: Direction, distance: i32) -> GridVector {
        GridVector {
            origin,
            direction,
            distance,
            _iter_state: origin
        }
    }

    fn iter_is_finished(&self) -> bool {
        match self.direction {
            Direction::U => self._iter_state.y == self.origin.y + self.distance,
            Direction::D => self._iter_state.y == self.origin.y - self.distance,
            Direction::R => self._iter_state.x == self.origin.x + self.distance,
            Direction::L => self._iter_state.x == self.origin.x - self.distance,
        }
    }
}

#[test]
fn test_gridvector_iterator(){
    let mut gv = GridVector::new(Point{x:0,y:0},Direction::U,3);
    assert_eq!(Some(Point{x:0,y:3}),
               gv.last());
    assert_eq!(Some(Point{x:0,y:2}),
               gv.nth(1));

    let gv2 = GridVector::new(Point{x:0,y:0},Direction::L,5);
    assert_eq!(Some(Point{x:-5,y:0}),
               gv2.last());
}
impl Iterator for GridVector {
    // Produces points along the line drawn by the vector
    // Mostly to avoid duplication for the purposes of this puzzle, the origin
    // point is not included in the output
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_is_finished() {
            // reset
            self._iter_state = self.origin;
            None
        } else {
            match self.direction {
                Direction::U => self._iter_state.y += 1,
                Direction::D => self._iter_state.y -= 1,
                Direction::R => self._iter_state.x += 1,
                Direction::L => self._iter_state.x -= 1,
            }
            Some(self._iter_state)
        }
    }
}

#[test]
fn test_get_manhattan_distance(){
    assert_eq!(1,get_manhattan_distance(&Point{x:0,y:0},&Point{x:0,y:1}));
    assert_eq!(9,get_manhattan_distance(&Point{x:0,y:1},&Point{x:3,y:7}));
    assert_eq!(6,get_manhattan_distance(&Point{x:0,y:0},&Point{x:-3,y:-3}));
    assert_eq!(9,get_manhattan_distance(&Point{x:-7,y:3},&Point{x:1,y:2}));
}
fn get_manhattan_distance(a: &Point, b: &Point) -> i32{
   i32::abs(a.x - b.x) + i32::abs(a.y - b.y)
}

#[test]
fn test_find_points_visited(){
    let input = vec![
        (Direction::U,3),
        (Direction::L,2),
    ];

    let expected: HashSet<Point> = [
        Point{x:0,y:1},
        Point{x:0,y:2},
        Point{x:0,y:3},
        Point{x:-1,y:3},
        Point{x:-2,y:3}
    ].iter().cloned().collect();

    assert_eq!(expected,find_points_visited(input));
}
fn find_points_visited(path: &Vec<(Direction, i32)>) -> HashSet<Point> {
    let mut start = Point{x:0,y:0};
    let mut visited = HashSet::new();

    for entry in path {
        let (direction, distance) = entry;
        let gv = GridVector::new(start,*direction,*distance);
        for point in gv {
            visited.insert(point);
            start = point;
        }
    }
    visited
}

#[test]
fn test_parse_wire_to_vec(){
    let expected: Vec<(Direction, i32)> = vec![
        (Direction::U,16),
        (Direction::U,7),
        (Direction::U,1),
        (Direction::R,9),
        (Direction::L,6),
        (Direction::D,9),
    ];
    assert_eq!(
        expected,
        parse_wire_to_vec("U16,U07,U1,R9,L6,D9")
    );
}
fn parse_wire_to_vec(wire: &str) -> Vec<(Direction,i32)> {
    wire.split(",")
         .map(|s| (s[0..1].parse::<Direction>().unwrap(),
                   s[1..].parse::<i32>().unwrap()))
        .collect()
}

fn part1(wire1: &Vec<(Direction,i32)>,wire2: &Vec<(Direction,i32)>) -> i32 {

    let visited_wire1 = find_points_visited(wire1);
    let visited_wire2 = find_points_visited(wire2);

    visited_wire1.intersection(&visited_wire2)
                 .map(|x| get_manhattan_distance(x,&Point{x:0,y:0}))
                 .min().unwrap()
}

fn main() {
    let input: Vec<&str> = include_str!("input").trim().split("\n").collect();

    let wire1: Vec<(Direction, i32)> = parse_wire_to_vec(input[0]);
    let wire2: Vec<(Direction, i32)> = parse_wire_to_vec(input[1]);

    println!("Part 1: Answer is {}", part1(&wire1,&wire2));
}
