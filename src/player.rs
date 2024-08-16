
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
        let grid_size = 100.0; // Tamaño de la celda
        let new_pos = self.pos + direction * move_speed;

        let current_cell_x = (self.pos.x / grid_size).floor() as usize;
        let current_cell_y = (self.pos.y / grid_size).floor() as usize;

        let new_cell_x = (new_pos.x / grid_size).floor() as usize;
        let new_cell_y = (new_pos.y / grid_size).floor() as usize;

        // Si el jugador está en la misma celda o se mueve a una nueva celda
        if current_cell_x != new_cell_x || current_cell_y != new_cell_y {
            match can_move_to(new_pos, maze) {
                Some(true) => self.pos = new_pos,
                None => return true, // Meta alcanzada
                _ => (),
            }
        } else {
            self.pos = new_pos;
        }

        // Comprobar si el jugador está cerca de la "E"
        if is_near_exit(new_pos, maze, grid_size) {
            return true; // Mostrar pantalla de éxito
        }

        false
    }
}

// Nueva función para verificar si el jugador está cerca de la "E"
fn is_near_exit(pos: Vec2, maze: &Vec<Vec<char>>, grid_size: f32) -> bool {
    let x = (pos.x / grid_size).floor() as isize;
    let y = (pos.y / grid_size).floor() as isize;

    for offset_x in -1..=1 {
        for offset_y in -1..=1 {
            let nx = x + offset_x;
            let ny = y + offset_y;
            if nx >= 0 && ny >= 0 && (ny as usize) < maze.len() && (nx as usize) < maze[0].len() {
                if maze[ny as usize][nx as usize] == 'E' {
                    return true;
                }
            }
        }
    }
    false
}


fn can_move_to(pos: Vec2, maze: &Vec<Vec<char>>) -> Option<bool> {
    let grid_size = 100.0; // Tamaño de la celda en unidades del juego
    let x = (pos.x / grid_size).floor() as usize;
    let y = (pos.y / grid_size).floor() as usize;

    if y < maze.len() && x < maze[0].len() {
        let cell = maze[y][x];
        if cell == 'E' {
            return None; // El jugador llegó a la meta
        }
        Some(cell != '|' && cell != '+' && cell != '-') // Considera estos como paredes
    } else {
        Some(false)
    }
}


pub fn eventos_jugador(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>) {
    const WALK_SPEED: f32 = 5.0;
    const RUN_SPEED: f32 = 15.0;  // Velocidad al trotar
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