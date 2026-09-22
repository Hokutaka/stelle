pub const MAP_WIDTH: usize = 7;
pub const MAP_HEIGHT: usize = 5;

// Temporary host-side mirror of src/world.ceru.
//
// false = wall
// true  = walkable
//
// #######
// #.....#
// #..#..#
// #.....#
// #######
const TILES: [bool; MAP_WIDTH * MAP_HEIGHT] = [
    false, false, false, false, false, false, false,
    false, true,  true,  true,  true,  true,  false,
    false, true,  true,  false, true,  true,  false,
    false, true,  true,  true,  true,  true,  false,
    false, false, false, false, false, false, false,
];

#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Player {
    pub fn spawn() -> Self {
        Self { x: 1, y: 1 }
    }

    pub fn try_move(&mut self, direction: Direction) {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let next_x = self.x + dx;
        let next_y = self.y + dy;

        if is_walkable(next_x, next_y) {
            self.x = next_x;
            self.y = next_y;
        }
    }
}

pub fn is_walkable(x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    let x = x as usize;
    let y = y as usize;

    if x >= MAP_WIDTH || y >= MAP_HEIGHT {
        return false;
    }

    TILES[y * MAP_WIDTH + x]
}

pub fn tile_is_walkable(x: usize, y: usize) -> bool {
    TILES[y * MAP_WIDTH + x]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_cannot_walk_into_center_wall() {
        let mut player = Player { x: 2, y: 2 };
        player.try_move(Direction::Right);

        assert_eq!(player.x, 2);
        assert_eq!(player.y, 2);
    }

    #[test]
    fn player_can_walk_on_floor() {
        let mut player = Player::spawn();
        player.try_move(Direction::Right);

        assert_eq!(player.x, 2);
        assert_eq!(player.y, 1);
    }
}
