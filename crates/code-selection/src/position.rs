use crate::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub y: i32,
    pub x: i32,
}

impl std::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // SAFETY: i32 and u32 have the same size and alignment.
        let (x, y) = unsafe {
            (
                std::mem::transmute::<i32, u32>(self.x),
                std::mem::transmute::<i32, u32>(self.y),
            )
        };

        let x = x as u64;
        let y = y as u64;

        let data = (y << 32) | x;

        data.hash(state);
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.to_offset()
    }
}

impl std::ops::Add<AreaSize> for Position {
    type Output = Position;

    fn add(self, rhs: AreaSize) -> Self::Output {
        Position {
            x: self.x + rhs.width as i32,
            y: self.y + rhs.height as i32,
        }
    }
}

impl Position {
    #[inline(always)]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelativePosition {
    pub y: u32,
    pub x: u32,
}

impl std::hash::Hash for RelativePosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let x = self.x as u64;
        let y = self.y as u64;

        let data = (y << 32) | x;

        data.hash(state);
    }
}

impl std::ops::Add<RelativePosition> for Position {
    type Output = Position;

    fn add(self, rhs: RelativePosition) -> Self::Output {
        Position {
            x: self.x + rhs.x as i32,
            y: self.y + rhs.y as i32,
        }
    }
}

impl std::ops::Mul<AreaSize> for RelativePosition {
    type Output = RelativePosition;

    fn mul(self, rhs: AreaSize) -> Self::Output {
        RelativePosition {
            x: self.x * rhs.width as u32,
            y: self.y * rhs.height as u32,
        }
    }
}

impl std::ops::Add<RelativePosition> for RelativePosition {
    type Output = RelativePosition;

    fn add(self, rhs: RelativePosition) -> Self::Output {
        RelativePosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<RelativePosition> for Position {
    type Output = Position;

    fn sub(self, rhs: RelativePosition) -> Self::Output {
        Position {
            x: self.x - rhs.x as i32,
            y: self.y - rhs.y as i32,
        }
    }
}

impl From<RelativePosition> for Position {
    fn from(value: RelativePosition) -> Self {
        Position {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl RelativePosition {
    #[inline(always)]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[test]
fn test_position_ord() {
    let pos0 = Position::new(0, 0);
    let pos1 = Position::new(1, 0);
    let pos2 = Position::new(0, 1);
    let pos3 = Position::new(1, 1);

    assert_eq!(pos0.cmp(&pos1), std::cmp::Ordering::Less);
    assert_eq!(pos0.cmp(&pos2), std::cmp::Ordering::Less);
    assert_eq!(pos0.cmp(&pos3), std::cmp::Ordering::Less);

    assert_eq!(pos1.cmp(&pos2), std::cmp::Ordering::Less);
    assert_eq!(pos1.cmp(&pos3), std::cmp::Ordering::Less);

    assert_eq!(pos2.cmp(&pos3), std::cmp::Ordering::Less);
}
