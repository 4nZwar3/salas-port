use {
    std::io::{self, Write},
    crate::sala::*,
};
pub struct Museo {
    nombre_museo: String,
    salas: Vec<Sala>,
}
impl Museo {
    pub fn new(nom: &str) -> Museo {
        Museo {
            salas: Vec::new(),
            nombre_museo: nom.to_string(),
        }
    }
    fn editar_sala(&mut self) {
        self.mostrar_salas();
        print!("Inserte el numero de sala a editar: ");
        io::stdout().flush().unwrap();
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let num = match num.trim().parse::<isize>() {
            Ok(num) => num,
            Err(_) => 0
        };
        if num > 0 && num as usize <= self.salas.len() {
            self.salas[(num - 1) as usize].menu();
        } else {
            println!("\nOpción inválida.");
        }
    }
    fn mostrar_salas(&self) {
        for (index, sala) in self.salas.iter().enumerate() {
            println!("\n\tSala {}.", index + 1);
            sala.show();
        }
    }
    fn buscar_sala(&self) {
        print!("Inserte la sala a buscar: ");
        io::stdout().flush().unwrap();
        let mut salita = String::new();
        io::stdin().read_line(&mut salita).unwrap();
        let salita = salita.trim().to_ascii_uppercase();
        let results = self.salas
        .iter()
        .filter(|painting| painting.get_name().to_lowercase().contains(&salita))
        .collect::<Vec<&Sala>>();
        if results.is_empty() {
            println!("No results found for '{}'.", salita);
        } else {
            println!("{} result(s) found:", results.len());
            for result in results {
                result.show();
            }
        }
    }
    pub fn menu(& mut self) {
        loop {
            println!("\nMenú de Museo {}", self.nombre_museo);
            println!("1) Agregar sala.");
            println!("2) Agregar sala vacía.");
            println!("3) Editar sala.");
            println!("4) Mostrar salas.");
            println!("5) Buscar sala.");
            println!("e) Salir");
            print!("Selecciona una opción: ");
            io::stdout().flush().unwrap();
            let mut opc = String::new();
            io::stdin().read_line(&mut opc).unwrap();
            let opc = match opc.trim().parse::<i32>() {
                Ok(num) => Ok(num),
                Err(_) => match opc.trim().parse::<char>() {
                    Ok(ch) => Err(ch),
                    Err(_) => Ok(0)
                }
            };
            match opc {
                Ok(1) => self.salas.push(Sala::init()),
                Ok(2) => self.salas.push(Sala::new()),
                Ok(3) => self.editar_sala(),
                Ok(4) => self.mostrar_salas(),
                Ok(5) => self.buscar_sala(),
                Err('e') => break,
                _ => println!("\nOpción no válida.")
            }
        }
    }
}