use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i8,
    y: i8,
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

enum Color {
    Black,
    White,
}

enum Chessman {
    Pawn(Position, Color),
    Knight(Position, Color),
    Bishop(Position, Color),
    Rook(Position, Color),
    Queen(Position, Color),
    King(Position, Color),
}

impl Chessman {
    fn move(&mut self, new_position: Position) -> bool {
        match self {
            Chessman::Pawn(ref mut pos, ref col) => {
                let difference = new_position - *pos;
                match col {
                    Color::Black => {
                        if difference.y == -1 && difference.x.abs() <= 1 {
                            *pos = new_position;
                            true
                        } else {
                            false
                        }
                    }
                    Color::White => {
                        if difference.y == 1 && difference.x.abs() <= 1 {
                            *pos = new_position;
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            _ => false, // other pieces not implemented
        }
    }
}

fn main() {
    let mut pawn = Chessman::Pawn(Position { x: 1, y: 2 }, Color::White);
    assert_eq!(pawn.move(Position { x: 1, y: 3 }), true);
    assert_eq!(pawn.move(Position { x: 1, y: 5 }), false);
}
