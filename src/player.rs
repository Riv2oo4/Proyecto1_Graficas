use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use minifb::{Window, Key};

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
}

pub fn eventos_jugador(window: &Window, player: &mut Player) {
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
        player.pos += direction * move_speed;
    }
    if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
        let direction = Vec2::new(player.a.cos(), player.a.sin());
        player.pos -= direction * move_speed;
    }
}