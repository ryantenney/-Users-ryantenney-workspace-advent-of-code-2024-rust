use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub};
use std::str::FromStr;
use anyhow::anyhow;

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Grid<T> {
    pub bounds: Position,
    pub grid: Vec<Vec<T>>,
}

impl <T> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid { bounds: Position::new(0, 0), grid: vec![] }
    }

    pub fn init(&mut self, input: &Vec<String>) where T: From<char> {
        for line in input.iter() {
            self.grid.push(line.chars().map(|c| c.into()).collect());
        }
        self.bounds = Position::new(self.grid[0].len() - 1, self.grid.len() - 1);
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        self.grid.get(pos.y).and_then(|row| row.get(pos.x))
    }

    pub fn get_dir(&self, pos: Position, dir: &Direction) -> Option<&T> {
        if (pos.x == 0 && *dir == Direction::Left)
            || (pos.x == self.bounds.x && *dir == Direction::Right)
            || (pos.y == 0 && *dir == Direction::Up)
            || (pos.y == self.bounds.y && *dir == Direction::Down) {
            return None;
        }
        self.get(pos + *dir)
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        self.grid.get_mut(pos.y).and_then(|row| row.get_mut(pos.x))
    }

    pub fn set(&mut self, pos: Position, value: T) {
        self.grid[pos.y][pos.x] = value;
    }

    pub fn enumerate(&self) -> GridIterator<T> {
        GridIterator::new(self)
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x <= self.bounds.x && pos.y <= self.bounds.y
    }

}

impl<T> Display for Grid<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for col in row.iter() {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    pos: Position,
}

impl <'a, T> GridIterator<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> GridIterator<'a, T> {
        GridIterator { grid, pos: Position::new(0, 0) }
    }
}

impl<'a, T> Iterator for GridIterator<'a, T> where T: Clone {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        self.pos.x += 1;
        if self.pos.x > self.grid.bounds.x {
            self.pos.x = 0;
            self.pos.y += 1;
        }
        self.grid.get(pos).map(|v| (pos, v))
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct State {
    pub pos: Position,
    pub vec: Vector,
}

impl State {
    pub fn new(pos: Position, vec: Vector) -> State {
        State { pos, vec }
    }

    pub fn copy(&self) -> Self {
        Self::new(self.pos, self.vec)
    }
}


#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn checked_add(&self, vec: &Vector) -> Option<Position> {
        self.x.checked_add_signed(vec.x)
            .zip(self.y.checked_add_signed(vec.y))
            .map(|(x, y)| Position::new(x, y))
    }

    pub fn checked_sub(&self, vec: &Vector) -> Option<Position> {
        self.x.checked_sub_signed(vec.x)
            .zip(self.y.checked_sub_signed(vec.y))
            .map(|(x, y)| Position::new(x, y))
    }

}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.y, self.x)
    }
}

impl Add<Vector> for Position {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x.checked_add_signed(rhs.x).unwrap(),
            y: self.y.checked_add_signed(rhs.y).unwrap(),
        }
    }
}

impl AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        self.x = self.x.checked_add_signed(rhs.x).unwrap();
        self.y = self.y.checked_add_signed(rhs.y).unwrap();
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let vector: Vector = rhs.into();
        self + vector
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        let rhs: Vector = rhs.into();
        self.x = self.x.checked_add_signed(rhs.x).unwrap();
        self.y = self.y.checked_add_signed(rhs.y).unwrap();
    }
}

impl Sub for Position {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x as isize - rhs.x as isize,
            y: self.y as isize - rhs.y as isize,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Self::new(pos.0, pos.1)
    }
}


#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Vector {
    pub fn new(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }
}

impl From<(isize, isize)> for Vector {
    fn from(vec: (isize, isize)) -> Self {
        Vector::new(vec.1, vec.0)
    }
}

impl From<Direction> for Vector {
    fn from(value: Direction) -> Self {
        value.to_tuple().into()
    }
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output { x: self.x * rhs as isize, y: self.y * rhs as isize }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Move for Vector {
    fn grid_move(&self, state: State) -> Vec<State> {
        state.pos.checked_add(self)
            .map(|pos| State::new(pos, *self))
            .map_or_else(|| vec![], |state| vec![state])
    }
}

impl ToVector for Vector {
    fn to_tuple(&self) -> (isize, isize) {
        (self.y, self.x)
    }

    fn to_vector(&self) -> Vector {
        *self
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.y, self.x)
    }
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    North,
    South,
    East,
    West,
}

impl Direction {
    pub const ALL: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    pub fn flip(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "u" | "up" => Ok(Direction::Up),
            "d" | "down" => Ok(Direction::Down),
            "l" | "left" => Ok(Direction::Left),
            "r" | "right" => Ok(Direction::Right),
            "n" | "north" => Ok(Direction::North),
            "s" | "south" => Ok(Direction::South),
            "w" | "west" => Ok(Direction::West),
            "e" | "east" => Ok(Direction::East),
            _ => Err(anyhow!("Invalid direction: {}", s)),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        };
        write!(f, "{}", c)
    }
}

impl ToVector for Direction {
    fn to_tuple(&self) -> (isize, isize) {
        match self {
            Direction::Up | Direction::North => (-1, 0),
            Direction::Down | Direction::South => (1, 0),
            Direction::Left | Direction::West => (0, -1),
            Direction::Right | Direction::East => (0, 1),
        }
    }

    fn to_vector(&self) -> Vector {
        match self {
            Direction::Up | Direction::North => Vector::new(0, -1),
            Direction::Down | Direction::South => Vector::new(0, 1),
            Direction::Left | Direction::West => Vector::new(-1, 0),
            Direction::Right | Direction::East => Vector::new(1, 0),
        }
    }
}


pub trait Move {
    fn grid_move(&self, state: State) -> Vec<State>;
}

pub trait ToVector {
    fn to_tuple(&self) -> (isize, isize);
    fn to_vector(&self) -> Vector;
}


impl Move for Direction {
    fn grid_move(&self, _state: State) -> Vec<State> {
        todo!()
    }
}
