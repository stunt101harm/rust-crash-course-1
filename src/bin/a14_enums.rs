#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(PartialEq, Debug)]
pub struct Turn {
    number: u64,
    direction: Direction,
}

fn main() {
    let left: Direction = Direction::Left;
    let right: Direction = Direction::Right;
    let up: Direction = Direction::Up;
    let down: Direction = Direction::Down;

    let game = Turn {
        number: 4,
        direction: Direction::Left,
    };

    if game.direction == Direction::Left {
        println!("{:?}", left)
    } else if game.direction == Direction::Right {
        println!("{:?}", right)
    } else if game.direction == Direction::Up {
        println!("{:?}", up)
    } else {
        println!("{:?}", down)
    }
}
