fn main() {
    let input = include_str!("../../input/day17.txt");

    let input: Vec<Direction> = input
        .chars()
        .map(|char| match char {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Invalid character"),
        })
        .collect();

    let mut moves = Moves::new(input);

    let mut grid: Grid = Vec::new();

    for i in 1..=1_000_000_000_000 {
        let rock: Rock = ((i) % 5).into();
        let mut position = if grid.is_empty() {
            (2, 3).into()
        } else {
            rock.start_position(grid.last().unwrap().y)
        };
        let mut current_move = 1;
        loop {
            if current_move % 2 == 1 {
                position = rock.gas_push(position, &grid, moves.get_next_direction());
                current_move += 1;
            } else {
                if !grid.can_move_down(&rock.get_parts(position)) {
                    break;
                }
                position.y -= 1;
                current_move = 1;
            }
        }

        grid.append(&mut Vec::from_iter(rock.get_parts(position)));

        grid.sort();

        if i == 2022 {
            println!("Part 1: {}", grid.last().unwrap().y + 1);
        } else if i == 1000000000000 {
            println!("Part 2: {}", grid.last().unwrap().y + 1); // This takes too long but I dont have motivation to optimize my solution
        }
    }
}

type Grid = Vec<Position>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
enum Rock {
    Flat,
    Plus,
    ReverseL,
    I,
    Cube,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.y.cmp(&other.y))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
    }
}

impl From<i64> for Rock {
    fn from(i: i64) -> Self {
        match i {
            1 => Rock::Flat,
            2 => Rock::Plus,
            3 => Rock::ReverseL,
            4 => Rock::I,
            0 => Rock::Cube,
            _ => panic!("Invalid rock"),
        }
    }
}

enum Direction {
    Left,
    Right,
}

impl Rock {
    fn get_parts(&self, position: Position) -> Vec<Position> {
        match self {
            Rock::Flat => vec![
                position,
                (position.x + 1, position.y).into(),
                (position.x + 2, position.y).into(),
                (position.x + 3, position.y).into(),
            ],
            Rock::Plus => vec![
                position,
                (position.x + 1, position.y).into(),
                (position.x + 2, position.y).into(),
                (position.x + 1, position.y + 1).into(),
                (position.x + 1, position.y - 1).into(),
            ],
            Rock::ReverseL => vec![
                position,
                (position.x + 1, position.y).into(),
                (position.x + 2, position.y).into(),
                (position.x + 2, position.y + 1).into(),
                (position.x + 2, position.y + 2).into(),
            ],
            Rock::I => vec![
                position,
                (position.x, position.y + 1).into(),
                (position.x, position.y + 2).into(),
                (position.x, position.y + 3).into(),
            ],
            Rock::Cube => vec![
                position,
                (position.x, position.y + 1).into(),
                (position.x + 1, position.y).into(),
                (position.x + 1, position.y + 1).into(),
            ],
        }
    }

    fn start_position(&self, highest_y: i64) -> Position {
        let mut parts = self.get_parts((0, 0).into());
        parts.sort_by(|a, b| a.y.cmp(&b.y));
        (2, (highest_y + parts.first().unwrap().y.abs()) + 4).into()
    }

    fn gas_push(
        &self,
        position: Position,
        used_positions: &Vec<Position>,
        direction: &Direction,
    ) -> Position {
        let new = match direction {
            Direction::Left => (position.x - 1, position.y).into(),
            Direction::Right => (position.x + 1, position.y).into(),
        };

        if self
            .get_parts(new)
            .iter()
            .any(|position| position.x < 0 || position.x > 6 || used_positions.contains(position))
        {
            position
        } else {
            new
        }
    }
}

trait MoveDown {
    fn can_move_down(&self, positions: &Vec<Position>) -> bool;
}

impl MoveDown for Grid {
    fn can_move_down(&self, positions: &Grid) -> bool {
        !positions
            .into_iter()
            .any(|position| self.contains(&(position.x, position.y - 1).into()) || position.y == 0)
    }
}

struct Moves {
    directions: Vec<Direction>,
    cursor: usize,
}

impl Moves {
    fn new(directions: Vec<Direction>) -> Self {
        Self {
            directions,
            cursor: 0,
        }
    }

    fn get_next_direction(&mut self) -> &Direction {
        if self.cursor >= self.directions.len() {
            self.cursor = 0;
        }
        let direction = &self.directions[self.cursor];
        self.cursor += 1;
        direction
    }
}
