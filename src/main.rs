use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod ahorcado;
use ahorcado::ahorcado::{Ahorcado, ResultadoIntento};

fn loop_principal(ahorcado:&mut Ahorcado) {
    ahorcado.texto_juego();
    let resultado_intento = ahorcado.entrada_usr();

    match resultado_intento {
        ResultadoIntento::PuedeSeguir => loop_principal(ahorcado),
        ResultadoIntento::Gana => {
            ahorcado.texto_juego();
            println!("Ganaste!");
        },
        ResultadoIntento::Muere => {
            ahorcado.texto_juego();
            println!("Ahorcado! X_X");
        },
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut palabras = Vec::<String>::new();
    if let Ok(lines) = read_lines("res/palabras") {
        for line in lines {
            palabras.push(line.expect("no se pudo leer el archivo"));
        }
    }

    let mut rng = rand::thread_rng();
    let indx = rng.gen_range(0..palabras.len());

    let juego = &mut Ahorcado::new(palabras[indx].to_string());
    loop_principal(juego);
}
