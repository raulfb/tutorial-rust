use std::io::stdin;

#[derive(Debug)]
struct Visitante {
    nombre: String,
    saludo: String,
}

impl Visitante {
    fn nuevo(nombre: &str, saludo: &str) -> Self {
        Self {
            nombre: nombre.to_lowercase(),
            saludo: saludo.to_string(),
        }
    }
    fn saludar_visitante(&self) {
        println!("{}", self.saludo)
    }
}

fn cual_es_tu_nombre() -> String {
    let mut tu_nombre = String::from("");
    stdin()
        .read_line(&mut tu_nombre)
        .expect("Error al leer la linea");

    tu_nombre.trim().to_lowercase()
}

fn main() {
    let mut listado_de_visitantes = vec![
        Visitante::nuevo("Raul", "Hola Raúl, disfruta del juego!"),
        Visitante::nuevo("Manolo", "Bienvenido Manolo!"),
    ];
    let banner="\n\n\
    ────────────────────────────────────────────────────────────────────────────────
    ─██████████████─██████████████─██████████████─████████████████───██████████████─
    ─██░░░░░░░░░░██─██░░░░░░░░░░██─██░░░░░░░░░░██─██░░░░░░░░░░░░██───██░░░░░░░░░░██─
    ─██░░██████████─██████░░██████─██░░██████░░██─██░░████████░░██───██████░░██████─
    ─██░░██─────────────██░░██─────██░░██──██░░██─██░░██────██░░██───────██░░██─────
    ─██░░██████████─────██░░██─────██░░██████░░██─██░░████████░░██───────██░░██─────
    ─██░░░░░░░░░░██─────██░░██─────██░░░░░░░░░░██─██░░░░░░░░░░░░██───────██░░██─────
    ─██████████░░██─────██░░██─────██░░██████░░██─██░░██████░░████───────██░░██─────
    ─────────██░░██─────██░░██─────██░░██──██░░██─██░░██──██░░██─────────██░░██─────
    ─██████████░░██─────██░░██─────██░░██──██░░██─██░░██──██░░██████─────██░░██─────
    ─██░░░░░░░░░░██─────██░░██─────██░░██──██░░██─██░░██──██░░░░░░██─────██░░██─────
    ─██████████████─────██████─────██████──██████─██████──██████████─────██████─────
    ────────────────────────────────────────────────────────────────────────────────
    ";
    println!("{}",banner);
    loop {

        println!("Hola, cual es tu nombre?(Es)");
        let nombre = cual_es_tu_nombre();
        let visitante_conocido = listado_de_visitantes
            .iter()
            .find(|visitante| visitante.nombre == nombre);
        match visitante_conocido {
            Some(visitante) => visitante.saludar_visitante(),
            None => {
                if nombre=="salir" {
                    break;
                } else {
                    println!(
                        "El usuario {} no está en la lista de visitantes admitidos.",
                        nombre
                    );
                    listado_de_visitantes.push(Visitante::nuevo(&nombre, "Usuario agregado!"));
                }
            }
        }
    }
}
