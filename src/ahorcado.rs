pub mod ahorcado {
    #[derive(Default)]
    pub struct Ahorcado {
        palabra: String,
        intentos: usize,
        letras_incorrectas: [char; 5],
        letras_correctas: Vec<char>,
    }

    pub enum ResultadoIntento {
        PuedeSeguir,
        Gana,
        Muere,
    }

    const LETRA_EN_BLANCO:char = '_';

    impl Ahorcado {
        pub fn new(palabra: String) -> Ahorcado {
            let vec_size = palabra.len(); // Calculo size antes.
            Ahorcado { 
                palabra, // Se queda con la palabra.
                intentos: 0, 
                letras_incorrectas: Default::default(), 
                letras_correctas: vec![LETRA_EN_BLANCO; vec_size], // Debe ser un vector porque no se puede saber el tamaño durante la compilación.
            }
        }

        fn probar_letra(&mut self, l:char) -> ResultadoIntento {
            
            if self.letras_correctas.contains(&l) || self.letras_incorrectas.contains(&l) {
                return ResultadoIntento::PuedeSeguir;
            }

            let mut acerto_letra = false;
            let mut quedan_letras = false;

            for (i, c) in self.palabra.chars().enumerate(){
                if c == l {
                    self.letras_correctas[i] = l;
                    acerto_letra = true;
                }

                if self.letras_correctas[i] == LETRA_EN_BLANCO {
                    quedan_letras = true;
                }
            }

            if !quedan_letras {
                ResultadoIntento::Gana
            } else if acerto_letra {
                ResultadoIntento::PuedeSeguir
            } else {
                self.letras_incorrectas[self.intentos] = l;
                self.intentos += 1;

                if self.intentos == 5 {
                    ResultadoIntento::Muere
                } else {
                    ResultadoIntento::PuedeSeguir
                }
            }
        }

        pub fn texto_juego(&self) {
            const TITULO:&str  = "Bienvenido al ahorcado de LAMBDA!";
            const PALABRA:&str = "La palabra hasta el momento es:";
            const INTENTOS:(&str, &str) = ("Te quedan", "intentos.");
            const LETRAS:&str = "Letras usadas:";
            const PROMPT:&str = "Ingresa una letra:";


            let mut adivinada_vec:Vec<char> = Vec::<char>::new();

            for l in &self.letras_correctas {
                adivinada_vec.push(*l); // Se toma el iterador.
                adivinada_vec.push(' ');
            }
            adivinada_vec.pop();

            let adivinada:String = adivinada_vec.into_iter().collect();

            
            let len_incorrectas = self.letras_incorrectas.len();
            let mut incorrectas_vec = vec![' '; len_incorrectas * 2 - 1];

            for (i, l) in self.letras_incorrectas[..len_incorrectas - 1].into_iter().enumerate() {
                incorrectas_vec[i * 2] = *l; 
            }

            let incorrectas:String = incorrectas_vec.into_iter().collect();


            std::process::Command::new("clear").status().expect("Error abriendo el archivo."); // TODO: ver si este unwrap puede quedar.
            println!("{}", TITULO);
            println!("{} {:#?}", PALABRA, adivinada);
            println!("{} {} {}", INTENTOS.0, 5 - self.intentos, INTENTOS.1);
            println!("{} {}", LETRAS, incorrectas);
            println!("{}", PROMPT);
        }

        pub fn entrada_usr(&mut self) -> ResultadoIntento {
            let mut v = String::from("");
            std::io::stdin().read_line(&mut v).expect("Error leyendo la linea.");
            self.probar_letra(v.as_bytes()[0] as char)
        }
    }
}
