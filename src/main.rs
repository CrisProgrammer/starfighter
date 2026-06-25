use macroquad::prelude::*;

//mod input;
mod entidades;

//use input::Input;
use entidades::Juego;

#[macroquad::main("Movimiento")]
async fn main() {
    let mut juego = Juego::new();

    loop {
        match juego.verificar_estado() {
            "game_over" => {
                draw_text(
                    "GAME OVER",
                    250.0,
                    250.0,
                    50.0,
                    RED,
                );

                draw_text(
                    "Presione R para reiniciar",
                    180.0,
                    320.0,
                    30.0,
                    WHITE,
                );
            }
            "jugando" => {
                juego.update().await;
                juego.draw();
            }
        }
        //juego.render();
        next_frame().await;
    }
}