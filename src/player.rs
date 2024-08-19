use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use minifb::{Window, Key};

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
}

impl Player {
    pub fn move_player(&mut self, direction: Vec2, move_speed: f32, maze: &Vec<Vec<char>>) -> bool {
        let grid_size = 100.0;
        let new_pos = self.pos + direction * move_speed;

        let new_cell_x = (new_pos.x / grid_size).floor() as usize;
        let new_cell_y = (new_pos.y / grid_size).floor() as usize;

        if maze[new_cell_y][new_cell_x] == 'E' {
            return true;
        }

        if maze[new_cell_y][new_cell_x] != ' ' {
            return false;
        }

        self.pos = new_pos;
        false
    }
}

pub fn eventos_jugador(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>) {
    const WALK_SPEED: f32 = 5.0;
    const RUN_SPEED: f32 = 15.0;
    const ROTATION_SPEED: f32 = PI / 35.0;

    let move_speed = if window.is_key_down(Key::Space) {
        RUN_SPEED
    } else {
        WALK_SPEED
    };

    if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
        player.a += ROTATION_SPEED;
    }
    if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
        let direction = Vec2::new(player.a.cos(), player.a.sin());
        player.move_player(direction, move_speed, maze);
    }
    if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
        let direction = Vec2::new(player.a.cos(), player.a.sin());
        player.move_player(-direction, move_speed, maze);
    }
}