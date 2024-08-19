mod framebuffer;
mod maze;
mod player;
mod caster;

use crate::caster::cast_ray;
use gilrs::{Gilrs, Button, Event, EventType, Axis};
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::{Player, eventos_jugador};
use rodio::{Decoder,Source, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

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
    match celda {
        'E' => framebuffer.set_current_color(0x00FF00), 
        ' ' => return, 
        _   => framebuffer.set_current_color(0x87CEFA), 
    }

    for x in xo..xo + tamaño_block {
        for y in yo..yo + tamaño_block {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.point(x, y);
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

    framebuffer.set_current_color(0xFF0000); 
    let jugador_x = ((player.pos.x / 100.0) * tamaño_block as f32) as isize;
    let jugador_y = ((player.pos.y / 100.0) * tamaño_block as f32) as isize;

    let longitud_flecha = 15.0; 
    let ancho_flecha = 10.0; 

    let punta_x = (jugador_x as f32 + player.a.cos() * longitud_flecha) as isize;
    let punta_y = (jugador_y as f32 + player.a.sin() * longitud_flecha) as isize;

    let base_x1 = (jugador_x as f32 + player.a.sin() * -ancho_flecha / 2.0) as isize;
    let base_y1 = (jugador_y as f32 - player.a.cos() * -ancho_flecha / 2.0) as isize;

    let base_x2 = (jugador_x as f32 + player.a.sin() * ancho_flecha / 2.0) as isize;
    let base_y2 = (jugador_y as f32 - player.a.cos() * ancho_flecha / 2.0) as isize;

    framebuffer.triangle(punta_x, punta_y, base_x1, base_y1, base_x2, base_y2);
}

fn render_minimapa(framebuffer: &mut Framebuffer, player: &Player, escala: f32) {
    let maze = load_maze("./maze.txt");
    let tamaño_block = (20.0 * escala) as usize; 

    let x_offset = framebuffer.width - (maze[0].len() * tamaño_block) - 20; 
    let y_offset = 20; 

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

    framebuffer.set_current_color(0xFF0000); 
    let jugador_x = (x_offset as f32 + ((player.pos.x / 100.0) * tamaño_block as f32)) as isize;
    let jugador_y = (y_offset as f32 + ((player.pos.y / 100.0) * tamaño_block as f32)) as isize;

    let longitud_flecha = 10.0; 
    let ancho_flecha = 7.0; 

    let punta_x = (jugador_x as f32 + player.a.cos() * longitud_flecha) as isize;
    let punta_y = (jugador_y as f32 + player.a.sin() * longitud_flecha) as isize;

    let base_x1 = (jugador_x as f32 + player.a.sin() * -ancho_flecha / 2.0) as isize;
    let base_y1 = (jugador_y as f32 - player.a.cos() * -ancho_flecha / 2.0) as isize;

    let base_x2 = (jugador_x as f32 + player.a.sin() * ancho_flecha / 2.0) as isize;
    let base_y2 = (jugador_y as f32 - player.a.cos() * ancho_flecha / 2.0) as isize;

    framebuffer.triangle(punta_x, punta_y, base_x1, base_y1, base_x2, base_y2);
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let tamaño_block = 100;
    let num_rays = framebuffer.width;


    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(0xD3D3D3); 
    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    for i in 0..num_rays {
        let ray_actual = i as f32 / num_rays as f32;
        let mut a = player.a - (player.fov / 2.0) + (player.fov * ray_actual);
        a = a.rem_euclid(2.0 * PI); 

        let interseccion = cast_ray(framebuffer, &maze, &player, a, tamaño_block, false);

        let mut distancia_a_pared = interseccion.distance;
        distancia_a_pared *= (player.a - a).cos(); 

        if distancia_a_pared == 0.0 {
            distancia_a_pared = 0.1; 
        }

        let distancia_al_plano = 277.0; 
        let altura_pared = (tamaño_block as f32 / distancia_a_pared) * distancia_al_plano;

        let stake_t = (hh - altura_pared / 2.0) as i32;
        let stake_b = (hh + altura_pared / 2.0) as i32;

        let color = match interseccion.impact {
             '+' => 0xFF6F61,  
            '-' => 0x6B8E23,  
            '|' => 0x4682B4,  
            'E' => 0xFFD700,  
            _ => 0xFFFFFF,    
        };

        framebuffer.set_current_color(color);

        for y in stake_t.max(0) as usize..stake_b.min(framebuffer.height as i32) as usize {
            framebuffer.point(i, y);
        }
    }
}


fn dibujar_letra(framebuffer: &mut Framebuffer, x: usize, y: usize, letra: char) {
    let patrones = match letra {
        'H' => [0b10001, 0b10001, 0b11111, 0b10001, 0b10001],
        'a' => [0b01110, 0b10001, 0b11111, 0b10001, 0b10001],
        's' => [0b01111, 0b10000, 0b01110, 0b00001, 0b11110],
        _ => [0b11111, 0b11111, 0b11111, 0b11111, 0b11111],
    };

    for (row, bits) in patrones.iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                framebuffer.point(x + col, y + row);
            }
        }
    }
}

fn mostrar_pantalla_exito(framebuffer: &mut Framebuffer) {
    framebuffer.clear(); 

    framebuffer.set_current_color(0x00FF00); 

    let mensaje = "¡HAS ALCANZADO LA META! ¡FELICITACIONES!";
    let x = framebuffer.width / 2 - (mensaje.len() * 6) / 2; 
    let y = framebuffer.height / 2;

    for (i, ch) in mensaje.chars().enumerate() {
        let offset_x = x + i * 6;
        dibujar_letra(framebuffer, offset_x, y, ch);
    }
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open("assets/Enchanted.wav").unwrap());
    let source = Decoder::new(file).unwrap().repeat_infinite();

    sink.append(source);
    sink.play();

    let mut gilrs = Gilrs::new().expect("Failed to initialize gilrs");

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

    framebuffer.set_background_color(0x000000);

    let maze = load_maze("./maze.txt");

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    let mut vista_3d = true;
    let mut mostrar_minimapa = false;
    let mut exito = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let tiempo_inicial = Instant::now();
        framebuffer.clear();

        if exito {
            mostrar_pantalla_exito(&mut framebuffer);
            window.update_with_buffer(&framebuffer.buffer, ancho_framebuffer, altura_framebuffer).unwrap();
            std::thread::sleep(Duration::from_secs(3));
            break;
        } else {
            while let Some(Event { event, .. }) = gilrs.next_event() {
                match event {
                    EventType::ButtonPressed(Button::South, _) => {}
                    EventType::AxisChanged(Axis::LeftStickX, value, _) => {
                        player.a += value * 0.05;
                    }
                    EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                        let direction = Vec2::new(player.a.cos(), player.a.sin());
                        let move_speed = if value > 0.0 { 5.0 } else { -5.0 };
                        if player.move_player(direction, move_speed, &maze) {
                            exito = true;
                        }
                    }
                    _ => {}
                }
            }

            if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
                let sensitivity = 0.005;

                if mouse_x < margen_sensible {
                    player.a -= (margen_sensible - mouse_x) * sensitivity;
                }

                if mouse_x > (ancho_ventana as f32 - margen_sensible) {
                    player.a += (mouse_x - (ancho_ventana as f32 - margen_sensible)) * sensitivity;
                }
            }

            eventos_jugador(&window, &mut player, &maze);

            if vista_3d {
                render3d(&mut framebuffer, &player);
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
