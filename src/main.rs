
mod framebuffer;
mod maze;
mod player;
mod caster;
mod texture;
use crate::caster::{cast_ray, Intersect, Orientation}; // Importar Orientation
use gilrs::{Gilrs, Button, Event, EventType, Axis};
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::{Player, eventos_jugador};
use texture::load_texture;

const FUENTE_NUMEROS: [[u8; 5]; 10] = [
    [0b01110, 0b10001, 0b10001, 0b10001, 0b01110],
    [0b00100, 0b01100, 0b00100, 0b00100, 0b01110],
    [0b01110, 0b10001, 0b00110, 0b01000, 0b11111],
    [0b01110, 0b10001, 0b00110, 0b10001, 0b01110],
    [0b00100, 0b01100, 0b10100, 0b11111, 0b00100],
    [0b11111, 0b10000, 0b11110, 0b00001, 0b11110],
    [0b01110, 0b10000, 0b11110, 0b10001, 0b01110],
    [0b11111, 0b00010, 0b00100, 0b01000, 0b10000],
    [0b01110, 0b10001, 0b01110, 0b10001, 0b01110],
    [0b01110, 0b10001, 0b01111, 0b00001, 0b01110],
];

fn dibujar_digitos(framebuffer: &mut Framebuffer, x: usize, y: usize, digito: u8) {
    if digito > 9 {
        return;
    }
    for (row, bits) in FUENTE_NUMEROS[digito as usize].iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                if x + col < framebuffer.width && y + row < framebuffer.height {
                    framebuffer.point(x + col, y + row);
                }
            }
        }
    }
}

fn dibujar_fps(framebuffer: &mut Framebuffer, fps: u32) {
    let fps_string = fps.to_string();
    let eje_x = 10;
    let eje_y = 10;
    let tamaño_digito = 6;

    framebuffer.set_current_color(0xFFFFFF);

    for (i, ch) in fps_string.chars().enumerate() {
        if let Some(digito) = ch.to_digit(10) {
            dibujar_digitos(framebuffer, eje_x + i * tamaño_digito, eje_y, digito as u8);
        }
    }
}

fn dibujar_celdas(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    tamaño_block: usize,
    celda: char,
) {
    // Asegúrate de que esta función dibuje las celdas no vacías
    if celda != ' ' {
        framebuffer.set_current_color(0x87CEFA); // Color de las celdas del laberinto
        for x in xo..xo + tamaño_block {
            for y in yo..yo + tamaño_block {
                if x < framebuffer.width && y < framebuffer.height {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}
fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let tamaño_block = (framebuffer.width / maze[0].len()) as usize;

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            dibujar_celdas(
                framebuffer,
                col * tamaño_block,
                row * tamaño_block,
                tamaño_block,
                maze[row][col],
            );
        }
    }

    // Dibujar la posición del jugador en el mapa 2D como un triángulo (flecha)
    framebuffer.set_current_color(0xFF0000); // Color rojo para la flecha del jugador
    let jugador_x = ((player.pos.x / 100.0) * tamaño_block as f32) as isize;
    let jugador_y = ((player.pos.y / 100.0) * tamaño_block as f32) as isize;

    let longitud_flecha = 15.0; // Longitud de la flecha
    let ancho_flecha = 10.0; // Ancho de la base del triángulo

    // Calcula los tres vértices del triángulo
    let punta_x = (jugador_x as f32 + player.a.cos() * longitud_flecha) as isize;
    let punta_y = (jugador_y as f32 + player.a.sin() * longitud_flecha) as isize;

    let base_x1 = (jugador_x as f32 + player.a.sin() * -ancho_flecha / 2.0) as isize;
    let base_y1 = (jugador_y as f32 - player.a.cos() * -ancho_flecha / 2.0) as isize;

    let base_x2 = (jugador_x as f32 + player.a.sin() * ancho_flecha / 2.0) as isize;
    let base_y2 = (jugador_y as f32 - player.a.cos() * ancho_flecha / 2.0) as isize;

    // Dibujar el triángulo que representa al jugador
    framebuffer.triangle(punta_x, punta_y, base_x1, base_y1, base_x2, base_y2);
}

fn render_minimapa(framebuffer: &mut Framebuffer, player: &Player, escala: f32) {
    let maze = load_maze("./maze.txt");
    let tamaño_block = (20.0 * escala) as usize; // Escala ajustada para el minimapa

    // Posición del minimapa en la esquina superior derecha
    let x_offset = framebuffer.width - (maze[0].len() * tamaño_block) - 20; // 20 píxeles de margen derecho
    let y_offset = 20; // 20 píxeles de margen superior

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            dibujar_celdas(
                framebuffer,
                x_offset + col * tamaño_block,
                y_offset + row * tamaño_block,
                tamaño_block,
                maze[row][col],
            );
        }
    }

    // Ajustar las coordenadas del jugador según la escala y la posición del minimapa
    framebuffer.set_current_color(0xFF0000); // Color rojo para la flecha del jugador en el minimapa
    let jugador_x = (x_offset as f32 + ((player.pos.x / 100.0) * tamaño_block as f32)) as isize;
    let jugador_y = (y_offset as f32 + ((player.pos.y / 100.0) * tamaño_block as f32)) as isize;

    let longitud_flecha = 10.0; // Longitud de la flecha en el minimapa
    let ancho_flecha = 7.0; // Ancho de la base del triángulo en el minimapa

    // Calcula los tres vértices del triángulo
    let punta_x = (jugador_x as f32 + player.a.cos() * longitud_flecha) as isize;
    let punta_y = (jugador_y as f32 + player.a.sin() * longitud_flecha) as isize;

    let base_x1 = (jugador_x as f32 + player.a.sin() * -ancho_flecha / 2.0) as isize;
    let base_y1 = (jugador_y as f32 - player.a.cos() * -ancho_flecha / 2.0) as isize;

    let base_x2 = (jugador_x as f32 + player.a.sin() * ancho_flecha / 2.0) as isize;
    let base_y2 = (jugador_y as f32 - player.a.cos() * ancho_flecha / 2.0) as isize;

    // Dibujar el triángulo que representa al jugador en el minimapa
    framebuffer.triangle(punta_x, punta_y, base_x1, base_y1, base_x2, base_y2);
}



fn render3d(framebuffer: &mut Framebuffer, player: &Player, wall_texture: &texture::Texture, floor_texture: &texture::Texture) {
    let maze = load_maze("./maze.txt");
    let tamaño_block = 100;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Dibujar el fondo (cielo) antes de las paredes
    framebuffer.set_current_color(0x000000); // Color del fondo superior (negro)
    for y in 0..hh as usize {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    // Renderizar las paredes
    for i in 0..num_rays {
        let ray_actual = i as f32 / num_rays as f32;
        let mut a = player.a - (player.fov / 2.0) + (player.fov * ray_actual);
        a = a.rem_euclid(2.0 * PI); 

        let interseccion = cast_ray(framebuffer, &maze, &player, a, tamaño_block, false);

        let mut distancia_a_pared = interseccion.distance;
        distancia_a_pared *= (player.a - a).cos(); // Corrección de la distancia

        if distancia_a_pared == 0.0 {
            distancia_a_pared = 0.1; 
        }

        let distancia_al_plano = 277.0; 
        let altura_pared = (tamaño_block as f32 / distancia_a_pared) * distancia_al_plano;

        let stake_t = (hh - altura_pared / 2.0) as i32;
        let stake_b = (hh + altura_pared / 2.0) as i32;

        // Suavizar el renderizado con interpolación lineal
        let line_height = stake_b - stake_t;
        let texture_step = wall_texture.height as f32 / line_height as f32;

        for y in stake_t.max(0) as usize..stake_b.min(framebuffer.height as i32) as usize {
            let proporcion_y = (y as f32 - stake_t as f32) * texture_step;
            let texture_y = proporcion_y as usize % wall_texture.height as usize;

            let texture_x = match interseccion.orientation {
                Orientation::Vertical => interseccion.point.y as usize % tamaño_block,
                Orientation::Horizontal => interseccion.point.x as usize % tamaño_block,
            };

            let pixel_index = texture_y * wall_texture.width as usize + (texture_x % wall_texture.width as usize);
            let color = wall_texture.get_pixel(pixel_index);

            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}
fn main() {
    let mut gilrs = Gilrs::new().expect("Failed to initialize gilrs"); // Inicializa Gilrs para manejar el mando

    let ancho_ventana = 1300;
    let altura_ventana = 900;
    let margen_sensible = 100.0;
    let ancho_framebuffer = 1300;
    let altura_framebuffer = 900;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(ancho_framebuffer, altura_framebuffer);

    let mut window = Window::new(
        "Proyecto 1",
        ancho_ventana,
        altura_ventana,
        WindowOptions::default(),
    )
    .unwrap();

    let stone_texture = load_texture("assets/paper.jpg").expect("No se pudo cargar la textura de piedra");
    let floor_texture = load_texture("assets/gris1.jpg").expect("No se pudo cargar la textura del suelo");

    framebuffer.set_background_color(0x000000);

    let maze = load_maze("./maze.txt");

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    let mut vista_3d = true;
    let mut mostrar_minimapa = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let tiempo_inicial = Instant::now();
        framebuffer.clear();

        // Manejar entrada desde el mando
        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(Button::South, _) => {
                    // Acción del botón (ej: salto, disparo, etc.)
                }
                EventType::AxisChanged(Axis::LeftStickX, value, _) => {
                    // Movimiento horizontal con el stick izquierdo
                    player.a += value * 0.05; // Ajusta la sensibilidad según sea necesario
                }
                EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                    let direction = Vec2::new(player.a.cos(), player.a.sin());
                    let move_speed = if value > 0.0 { 5.0 } else { -5.0 }; // Cambia según la dirección del eje
                    player.move_player(direction, move_speed, &maze);
                }
                _ => {}
            }
        }

        // Capturar la posición del mouse y rotar la cámara
        if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            let sensitivity = 0.005;  // Ajusta la sensibilidad según sea necesario

            // Detectar si el mouse está en la zona sensible izquierda
            if mouse_x < margen_sensible {
                player.a -= (margen_sensible - mouse_x) * sensitivity;
            }

            // Detectar si el mouse está en la zona sensible derecha
            if mouse_x > (ancho_ventana as f32 - margen_sensible) {
                player.a += (mouse_x - (ancho_ventana as f32 - margen_sensible)) * sensitivity;
            }
        }

        // Manejar entrada desde el teclado
        eventos_jugador(&window, &mut player, &maze);

        // Cambiar la vista según el estado actual
        if vista_3d {
            render3d(&mut framebuffer, &player, &stone_texture, &floor_texture);
            if mostrar_minimapa {
                render_minimapa(&mut framebuffer, &player, 0.2);
            }
        } else {
            render2d(&mut framebuffer, &player);
        }

        if window.is_key_down(Key::Y) {
            vista_3d = !vista_3d;
            std::thread::sleep(Duration::from_millis(200));
        }

        if window.is_key_down(Key::M) {
            mostrar_minimapa = !mostrar_minimapa;
            std::thread::sleep(Duration::from_millis(200));
        }

        let duracion = tiempo_inicial.elapsed();
        let tiempo_frame = duracion.as_secs_f32();
        let fps = (1.0 / tiempo_frame) as u32;
        dibujar_fps(&mut framebuffer, fps);

        window
            .update_with_buffer(&framebuffer.buffer, ancho_framebuffer, altura_framebuffer)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}