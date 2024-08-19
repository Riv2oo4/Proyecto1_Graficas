# Proyecto de Laberinto en Rust

## Descripción

Este proyecto es un juego de laberinto desarrollado en Rust que se renderiza tanto en 2D como en 3D. El objetivo del juego es guiar al jugador desde el punto de inicio hasta la salida del laberinto, evitando obstáculos y utilizando el minimapa para la navegación.

El juego incluye:
- Movimiento del jugador controlado por el teclado y el mouse.
- Renderizado 3D del laberinto con texturas en las paredes.
- Minimap en 2D para facilitar la navegación.
- Texturas y colores personalizables para las paredes y el fondo.

## Tabla de Contenidos

- [Instalación](#instalación)
- [Uso](#uso)
- [Controles](#controles)
- [Contribución](#contribución)
- [Licencia](#licencia)

## Instalación

Para instalar y ejecutar el proyecto, sigue los siguientes pasos:

1. Clona el repositorio:
    ```bash
    git clone https://github.com/Riv2oo4/Proyecto1_Graficas.git
    ```
2. Navega al directorio del proyecto:
    ```bash
    cd Proyecto1_Graficas
    ```
3. Compila el proyecto utilizando Cargo:
    ```bash
    cargo build --release
    ```
4. Ejecuta el juego:
    ```bash
    cargo run --release
    ```

## Uso

El juego se ejecuta en una ventana que muestra el laberinto en 3D. Utiliza las teclas de dirección para mover al jugador (flechas del teclado o WASD), y el mouse para controlar la cámara, de igual manera está adapatado para usarse con cualquier mando.

### Ejemplo de un Laberinto

El archivo `maze.txt` define la estructura del laberinto. Aquí tienes un ejemplo del formato:

```txt
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                 |                       |
+  +--+--+  +--+  +  +--+--+--+  +--+--+  +
|     |     |  |     |        |  |     |  |
+--+  +--+  +  +  +--+  +--+  +  +  +  +  +
|  |     |     |     |     |     |  |  |  |
+  +  +  +--+  +  +  +--+  +--+  +  +  +  +
|  |  |     |     |     |  |     |  |     |
+  +  +  +  +  +--+--+  +  +  +--+  +--+  +
|     |  |  |  |        |     |        |  |
+  +--+  +  +  +--+  +--+  +--+--+  +  +  +
|        |     |     |  |     |     |     |
+  +  +--+  +--+  +  +  +--+  +  +--+  +  +
|  |  |     |     |     |  |  |     |  |  |
+--+  +--+  +  +--+--+  +  +  +  +--+  +--+
|     |     |        |     |     |        E
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
