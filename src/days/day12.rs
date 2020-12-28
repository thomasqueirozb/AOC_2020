use std::fmt;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Action {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

pub fn day12(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day12.txt");
    let data = std::fs::read_to_string(p)?;

    let instructions: Vec<_> = data
        .split('\n')
        .filter_map(|line| {
            let mut lc = line.chars();
            let action = lc.next()?;

            let value: String = lc.collect();
            let mut value = value.parse::<i64>().ok()?;

            let action = match action {
                'N' => Action::NORTH,
                'S' => Action::SOUTH,
                'E' => Action::EAST,
                'W' => Action::WEST,
                'L' => {
                    if value % 90 != 0 {
                        return None;
                    }
                    value = value / 90;

                    Action::LEFT
                }
                'R' => {
                    if value % 90 != 0 {
                        return None;
                    }
                    value = value / 90;

                    Action::RIGHT
                }

                'F' => Action::FORWARD,
                _ => return None,
            };

            Some((action, value))
        })
        .collect();

    println!("Part 1");
    part1(&instructions);
    println!("\nPart 2");
    part2(&instructions);
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Mul<Output = T>> Mul for Point<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<&T> for Point<T> {
    type Output = Self;
    fn mul(self, other: &T) -> Self {
        Self::new(self.x * *other, self.y * *other)
    }
}

fn part1(instructions: &Vec<(Action, i64)>) {
    let mut pos: Point<i64> = Point { x: 0, y: 0 };
    const DIRECTION_EAST: Point<i64> = Point { x: 1, y: 0 };
    const DIRECTION_WEST: Point<i64> = Point { x: -1, y: 0 };
    const DIRECTION_NORTH: Point<i64> = Point { x: 0, y: 1 };
    const DIRECTION_SOUTH: Point<i64> = Point { x: 0, y: -1 };
    let faces: [Point<i64>; 4] = [
        DIRECTION_EAST,
        DIRECTION_SOUTH,
        DIRECTION_WEST,
        DIRECTION_NORTH,
    ];
    let mut facing: usize = 0; // starts facing east

    for (action, value) in instructions {
        match action {
            Action::NORTH => pos.y += value,
            Action::SOUTH => pos.y -= value,
            Action::EAST => pos.x += value,
            Action::WEST => pos.x -= value,
            Action::FORWARD => pos += faces[facing] * value,
            Action::LEFT => {
                facing = ((facing as i64 - value).rem_euclid(4)) as usize;
            }
            Action::RIGHT => {
                facing = ((facing as i64 + value).rem_euclid(4)) as usize;
            }
        }
    }
    println!(
        "Final position: {}. Manhattan distance: {}",
        pos,
        pos.x.abs() + pos.y.abs()
    );
}

fn part2(instructions: &Vec<(Action, i64)>) {
    let mut waypoint: Point<i64> = Point { x: 10, y: 1 };
    let mut pos: Point<i64> = Point { x: 0, y: 0 };
    const DIRECTION_EAST: Point<i64> = Point { x: 1, y: 0 };
    const DIRECTION_WEST: Point<i64> = Point { x: -1, y: 0 };
    const DIRECTION_NORTH: Point<i64> = Point { x: 0, y: 1 };
    const DIRECTION_SOUTH: Point<i64> = Point { x: 0, y: -1 };
    for (action, value) in instructions {
        // println!("Pos {} | waypoint {}", pos, waypoint);
        match action {
            Action::NORTH => waypoint += DIRECTION_NORTH * value,
            Action::SOUTH => waypoint += DIRECTION_SOUTH * value,
            Action::EAST => waypoint += DIRECTION_EAST * value,
            Action::WEST => waypoint += DIRECTION_WEST * value,
            Action::FORWARD => pos += waypoint * value,
            Action::LEFT => {
                for _ in 0..*value {
                    waypoint = Point::new(-waypoint.y, waypoint.x);
                }
            }
            Action::RIGHT => {
                for _ in 0..*value {
                    waypoint = Point::new(waypoint.y, -waypoint.x);
                }
            }
        }
    }
    println!(
        "Final position: {}. Manhattan distance: {}",
        pos,
        pos.x.abs() + pos.y.abs()
    );
}
