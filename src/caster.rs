use crate::framebuffer::Framebuffer;
use crate::player::Player;
use nalgebra_glm::Vec2;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub _point: Vec2,
    pub _orientation: Orientation,
}

pub enum Orientation {
    Vertical,
    Horizontal,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &[Vec<char>],
    player: &Player,
    angle: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    const STEP_SIZE: f32 = 0.5;
    let mut distance = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let (cos_angle, sin_angle) = (distance * angle.cos(), distance * angle.sin());
        let (x, y) = (
            (player.pos.x + cos_angle) as usize,
            (player.pos.y + sin_angle) as usize,
        );

        let (i, j) = (x / block_size, y / block_size);

        if j >= maze.len() || i >= maze[j].len() {
            return Intersect {
                distance,
                impact: ' ', 
                _point: Vec2::new(x as f32, y as f32),
                _orientation: if angle.sin().abs() > angle.cos().abs() {
                    Orientation::Vertical
                } else {
                    Orientation::Horizontal
                },
            };
        }

        if maze[j][i] != ' ' {
            return Intersect {
                distance,
                impact: maze[j][i],
                _point: Vec2::new(x as f32, y as f32),
                _orientation: if angle.sin().abs() > angle.cos().abs() {
                    Orientation::Vertical
                } else {
                    Orientation::Horizontal
                },
            };
        }

        if draw_line {
        }

        distance += STEP_SIZE;
    }
}