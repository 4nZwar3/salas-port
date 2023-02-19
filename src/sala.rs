use {
    std::io::{self, Write},
    crate::obra::*,
};
pub struct Sala {
    obras: Vec<Obra>,
    nombre: String
}
impl Sala {
    pub fn new(nom: &str) -> Sala {
        Sala {
            obras: Vec::new(),
            nombre: nom.to_string(),
        }
    }
    fn consult_obra(obra: &Obra) {
        println!("Titulo: {}", obra.get_titulo());
        println!("Autor: {}", obra.get_autor());
        println!("Tipo: {}", obra.get_tipo());
        println!("Año: {}", obra.get_anio());
        println!("Descripción: {}", obra.get_desc());
    }
    fn consult_obras(&self) {
        for (index, obra) in self.obras.iter().enumerate() {
            println!("\n\tObra {}.", index + 1);
            Sala::consult_obra(obra);
        }
    }
    fn edit_obra(&mut self) {
        print!("\nInserte el numero de obra que desea editar: ");
        io::stdout().flush().unwrap();
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let num = match num.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => 0
        };
        if num > -1 && num as usize <= self.obras.len() {
            self.obras[(num - 1) as usize].obra_menu();
        } else {
            println!("\nOpción inválida.");
        }

    }
    pub fn menu(&mut self) {
        loop {
            println!("\nMenú de Sala {}", self.nombre);
            println!("1) Agregar obra.");
            println!("2) Agregar obra vacía.");
            println!("3) Consultar obras.");
            println!("4) Editar obra.");
            println!("5) Salir");
            print!("Selecciona una opción: ");
            io::stdout().flush().unwrap();
            let mut opc = String::new();
            io::stdin().read_line(&mut opc).unwrap();
            let opc = match opc.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => -1
            };
            match opc {
                1 => self.obras.push(Obra::init()),
                2 => self.obras.push(Obra::new()),
                3 => self.consult_obras(),
                4 => self.edit_obra(),
                5 => break,
                _ => println!("\nOpción no válida.")
            }
        }
    }
}