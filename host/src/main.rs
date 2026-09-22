mod game;

use game::{Direction, MAP_HEIGHT, MAP_WIDTH, Player};
use minifb::{Key, KeyRepeat, Window, WindowOptions};

const TILE_SIZE: usize = 96;
const WIDTH: usize = MAP_WIDTH * TILE_SIZE;
const HEIGHT: usize = MAP_HEIGHT * TILE_SIZE;

const COLOR_BACKGROUND: u32 = 0x00131A22;
const COLOR_WALL: u32 = 0x002A3542;
const COLOR_FLOOR: u32 = 0x00D6D0C4;
const COLOR_GRID: u32 = 0x00B8B1A6;
const COLOR_PLAYER: u32 = 0x00E7B84B;

fn main() -> Result<(), minifb::Error> {
    let mut window = Window::new(
        "Stelle — WASD / Arrow Keys, Esc to quit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )?;

    window.set_target_fps(60);
    window.set_key_repeat_delay(0.18);
    window.set_key_repeat_rate(0.08);

    let mut buffer = vec![COLOR_BACKGROUND; WIDTH * HEIGHT];
    let mut player = Player::spawn();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(&window, &mut player);
        render(&mut buffer, player);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }

    Ok(())
}

fn handle_input(window: &Window, player: &mut Player) {
    if pressed(window, Key::W) || pressed(window, Key::Up) {
        player.try_move(Direction::Up);
    } else if pressed(window, Key::S) || pressed(window, Key::Down) {
        player.try_move(Direction::Down);
    } else if pressed(window, Key::A) || pressed(window, Key::Left) {
        player.try_move(Direction::Left);
    } else if pressed(window, Key::D) || pressed(window, Key::Right) {
        player.try_move(Direction::Right);
    }
}

fn pressed(window: &Window, key: Key) -> bool {
    window.is_key_pressed(key, KeyRepeat::Yes)
}

fn render(buffer: &mut [u32], player: Player) {
    buffer.fill(COLOR_BACKGROUND);

    for tile_y in 0..MAP_HEIGHT {
        for tile_x in 0..MAP_WIDTH {
            let color = if game::tile_is_walkable(tile_x, tile_y) {
                COLOR_FLOOR
            } else {
                COLOR_WALL
            };

            fill_tile(buffer, tile_x, tile_y, color);
        }
    }

    draw_grid(buffer);

    let px = player.x as usize * TILE_SIZE;
    let py = player.y as usize * TILE_SIZE;
    let margin = TILE_SIZE / 4;

    fill_rect(
        buffer,
        px + margin,
        py + margin,
        TILE_SIZE - margin * 2,
        TILE_SIZE - margin * 2,
        COLOR_PLAYER,
    );
}

fn fill_tile(buffer: &mut [u32], tile_x: usize, tile_y: usize, color: u32) {
    fill_rect(
        buffer,
        tile_x * TILE_SIZE,
        tile_y * TILE_SIZE,
        TILE_SIZE,
        TILE_SIZE,
        color,
    );
}

fn draw_grid(buffer: &mut [u32]) {
    for tile_x in 0..=MAP_WIDTH {
        let x = (tile_x * TILE_SIZE).min(WIDTH - 1);
        fill_rect(buffer, x, 0, 1, HEIGHT, COLOR_GRID);
    }

    for tile_y in 0..=MAP_HEIGHT {
        let y = (tile_y * TILE_SIZE).min(HEIGHT - 1);
        fill_rect(buffer, 0, y, WIDTH, 1, COLOR_GRID);
    }
}

fn fill_rect(
    buffer: &mut [u32],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    let max_x = (x + width).min(WIDTH);
    let max_y = (y + height).min(HEIGHT);

    for py in y..max_y {
        let row = py * WIDTH;
        for px in x..max_x {
            buffer[row + px] = color;
        }
    }
}
