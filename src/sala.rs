use {
    std::io::{self, Write},
    crate::obra::*,
};
pub struct Sala {
    obras: Vec<Obra>,
    nombre: String
}
impl Sala {
    pub fn new() -> Sala {
        Sala {
            obras: Vec::new(),
            nombre: String::new(),
        }
    }
    pub fn init() -> Sala {
        println!("\n\t\tAgregar Sala.");
        print!("Nombre: ");
        io::stdout().flush().unwrap();
        let mut titulito = String::new();
        io::stdin().read_line(&mut titulito).unwrap();
        titulito = titulito.trim().to_string();
        let mut obritas: Vec<Obra> = Vec::new();
        loop {
            print!("Desea añadir una obra (y/N): ");
            io::stdout().flush().unwrap();
            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();
            match response.trim().to_ascii_lowercase().parse::<char>() {
                Ok('y') => obritas.push(Obra::init()),
                _ => break
            };
        }
        Sala {
            nombre: titulito,
            obras: obritas
        }
    }
    pub fn get_name(&self) -> String {
        self.nombre.clone()
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
            println!("\n\t- Obra {}.", index + 1);
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
        if num > 0 && num as usize <= self.obras.len() {
            self.obras[(num - 1) as usize].obra_menu();
        } else {
            println!("\nOpción inválida.");
        }
    }
    pub fn show(&self) {
        println!("Nombre: {}", self.nombre);
        self.consult_obras();
    }
    pub fn menu(&mut self) {
        loop {
            println!("\nMenú de Sala {}", self.nombre);
            println!("1) Agregar obra.");
            println!("2) Agregar obra vacía.");
            println!("3) Consultar obras.");
            println!("4) Editar obra.");
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
                Ok(1) => self.obras.push(Obra::init()),
                Ok(2) => self.obras.push(Obra::new()),
                Ok(3) => self.consult_obras(),
                Ok(4) => self.edit_obra(),
                Err('e') => break,
                _ => println!("\nOpción no válida.")
            }
        }
    }
}