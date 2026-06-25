use macroquad::prelude::*;

pub struct Juego {
    jugador: Jugador,
    enemigos: Vec<Enemigo>,
    disparos: Vec<Disparo>,
    puntuacion: i32,
    oleada: i32,
    estado: String,
}
impl Juego {
    pub fn new() -> Self {
        Juego {
            jugador: Jugador {
                vida: 3,
                nombre: String::from("Jugador1"),
                posicion: Posicion { x: 300.0, y: 500.0 },
                disparos: 100,
            },
            enemigos: Vec::new(),
            disparos: Vec::new(),
            puntuacion: 0,
            oleada: 0,
            estado: String::from("jugando"),
        }
    }
    pub async fn update(&mut self) {
        clear_background(BLACK);
        self.mover_jugador();
        self.mover_disparos();
        self.mover_enemigos();
        self.detectar_colisiones();
        /*
        self.limpiar_objetos();*/
        self.generar_enemigos();
    }
    pub fn render(&self) {
        println!("Puntuación: {}", self.puntuacion);
        println!("Vida del jugador: {}", self.jugador.vida);
        println!("Número de enemigos: {}", self.enemigos.len());
    }   
    pub fn draw(&self) {
        draw_rectangle(self.jugador.posicion.x-25.0, self.jugador.posicion.y-10.0, 50.0, 20.0, GREEN);
        for enemigo in &self.enemigos {
            if enemigo.activo {
                draw_rectangle(enemigo.posicion.x-20.0, enemigo.posicion.y-10.0, 40.0, 20.0, RED);
            }
        }
        for disparo in &self.disparos {
            if disparo.activo {
                draw_rectangle(disparo.posicion.x-2.5, disparo.posicion.y-15.0, 5.0, 30.0, YELLOW);
            }
        }
    }
    pub fn mover_jugador(&mut self){
        // La función reconoce la flecha presionada y actualiza la posicion del jugador en consecuencia. 
        if is_key_down(KeyCode::Right) {
            self.jugador.posicion.x += 5.0;
        }

        if is_key_down(KeyCode::Left) {
            self.jugador.posicion.x -= 5.0;
        }

        if is_key_down(KeyCode::Down) {
            self.jugador.posicion.y += 5.0;
        }


        if is_key_down(KeyCode::Up) {
            self.jugador.posicion.y -= 5.0;
        }
    }pub fn mover_disparos(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            match self.jugador.disparar() {
                Some(disparo) => self.disparos.push(disparo),
                None => println!("No hay disparos disponibles"),
            }
        }
        for disparo in &mut self.disparos {
            if disparo.direccion {
                disparo.posicion.y -= disparo.velocidad;
            } else {
                disparo.posicion.y += disparo.velocidad;
            }
        }
    }pub fn generar_enemigos(&mut self) {
        if self.enemigos.is_empty() {
            self.oleada += 1;
            self.crear_formacion(3, 8, 1, "basico");
        }
    }fn crear_formacion(&mut self, filas: usize, columnas: usize, vida: i32, tipo: &str) {
        let start_x = 100.0;
        let start_y = 50.0;
        let spacing_x = 60.0;
        let spacing_y = 40.0;

        for fila in 0..filas {
            for col in 0..columnas {
                self.enemigos.push(Enemigo {
                    vida,
                    tipo: tipo.to_string(),
                    posicion: Posicion {
                        x: start_x + col as f32 * spacing_x,
                        y: start_y + fila as f32 * spacing_y,
                    },
                    activo: true,
                    direccion: true,
                });
            }
        }
    }pub fn mover_enemigos(&mut self) {
        let mut direccion_cambiada = false;
        for enemigo in &mut self.enemigos {
            if enemigo.activo {
                if enemigo.direccion {
                    enemigo.posicion.x += 1.0; // Movimiento hacia la derecha
                } else {
                    enemigo.posicion.x -= 1.0; // Movimiento hacia la izquierda
                }
                enemigo.posicion.y += 0.5; // Movimiento hacia abajo
                if enemigo.posicion.x > 700.0 || enemigo.posicion.x < 0.0 {
                    direccion_cambiada = true;
                }
            }
        }for enemigo in &mut self.enemigos {
            if enemigo.activo {
                if direccion_cambiada {
                    enemigo.direccion = !enemigo.direccion; // Cambia la dirección
                }
            }
        }
    }fn detectar_colisiones(&mut self) {
        for enemigo in &mut self.enemigos {
            if enemigo.activo {
                for disparo in &mut self.disparos {
                    if disparo.activo && 
                    ((disparo.posicion.x - 25.0 < enemigo.posicion.x + 20.0 &&
                    disparo.posicion.x - 25.0 > enemigo.posicion.x - 20.0)
                    || (disparo.posicion.x + 25.0 > enemigo.posicion.x - 20.0 && 
                    disparo.posicion.x + 25.0 < enemigo.posicion.x + 20.0)) &&
                    ((disparo.posicion.y - 10.0 < enemigo.posicion.y + 10.0 &&
                    disparo.posicion.y - 10.0 > enemigo.posicion.y - 10.0)
                    || (disparo.posicion.y + 10.0 < enemigo.posicion.y + 10.0 &&
                    disparo.posicion.y + 10.0 > enemigo.posicion.y - 10.0)) {
                        disparo.activo = false;
                        enemigo.vida -= 1;
                        if enemigo.vida <= 0 {
                            enemigo.activo = false;
                            disparo.activo = false;
                            self.puntuacion += 10;
                        }
                    }
                }
                if enemigo.posicion.y > 550.0
                || (((self.jugador.posicion.x - 2.5 < enemigo.posicion.x + 20.0 &&
                self.jugador.posicion.x - 2.5 > enemigo.posicion.x - 20.0)
                || (self.jugador.posicion.x + 2.5 > enemigo.posicion.x - 20.0 && 
                self.jugador.posicion.x + 2.5 < enemigo.posicion.x + 20.0)) &&
                ((self.jugador.posicion.y - 15.0 < enemigo.posicion.y + 20.0 &&
                self.jugador.posicion.y - 15.0 > enemigo.posicion.y - 20.0)
                || (self.jugador.posicion.y + 15.0 < enemigo.posicion.y + 20.0 &&
                self.jugador.posicion.y + 15.0 > enemigo.posicion.y - 20.0))) {
                    enemigo.activo = false;
                    self.jugador.vida -= 1;
                }
            }
        }
    }pub fn verificar_estado(&mut self) -> String{
        self.estado.clone();
    }
}
struct Jugador {
    vida: i32,
    nombre: String,
    posicion: Posicion,
    disparos: i32,
}impl Jugador {
    pub fn new() -> Self {
        Jugador {
            vida: 3,
            nombre: String::from("Jugador1"),
            posicion: Posicion { x: 0.0, y: 0.0 },
            disparos: 0,
        }
    }
    fn disparar(&mut self) -> Option<Disparo> {
        if self.disparos > 0 {
            self.disparos -= 1;
            let nuevo_disparo = Disparo::new(true, self.posicion);
            Some(nuevo_disparo)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
struct Posicion {
    x: f32,
    y: f32,
}impl Posicion {
    pub fn new() -> Self {
        Posicion {
            x: 0.0,
            y: 0.0,
        }
    }
}

struct Enemigo {
    vida: i32,
    tipo: String,
    posicion: Posicion,
    activo: bool,
    direccion: bool,
}impl Enemigo {
    pub fn new() -> Self {
        Enemigo {
            vida: 1,
            tipo: "Enemigo1".to_string(),
            posicion: Posicion {x: 0.0, y: 0.0},
            activo: true,
            direccion: true,
        }
    }
}

struct Disparo {
    velocidad: f32,
    direccion: bool,
    activo: bool,
    posicion: Posicion,
}impl Disparo {
    pub fn new(direccion: bool, posicion: Posicion) -> Self {
        Disparo {
            velocidad: 10.0,
            direccion,
            posicion,
            activo: true,
        }
    }
}