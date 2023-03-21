
mod tests;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        }
    }
}

pub struct Robot{x: i32, y: i32, d: Direction}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot{x, y, d}
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Robot{d: self.d.turn_right(), ..self}
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot{d: self.d.turn_left(), ..self}
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Robot{y: self.y+1, ..self},
            Direction::South => Robot{y: self.y-1, ..self},
            Direction::East => Robot{x: self.x+1, ..self},
            Direction::West => Robot{x: self.x-1, ..self}
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut result = Robot{..self};
        for instruction in instructions.chars() {
            result = match instruction {
                'R' => result.turn_right(),
                'L' => result.turn_left(),
                'A' => result.advance(),
                _ => panic!()
            }
        }
        result
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
