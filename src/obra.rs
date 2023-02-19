use std::io::{self, Write};
pub struct Obra {
    titulo: String,
    autor: String,
    tipo: String,
    anio: i32,
    desc: String
}
impl Obra {
    pub fn new() -> Obra {
        Obra {
            titulo: String::new(),
            autor: String::new(),
            tipo: String::new(),
            anio: 0,
            desc: String::new()
        }
    }
    pub fn init() -> Obra {
        println!("\n\t\tAgregar Obra.");
        print!("Titulo: ");
        io::stdout().flush().unwrap();
        let mut titulito = String::new();
        io::stdin().read_line(&mut titulito).unwrap();
        titulito = titulito.trim().to_string();
        print!("Autor: ");
        io::stdout().flush().unwrap();
        let mut autorito = String::new();
        io::stdin().read_line(&mut autorito).unwrap();
        autorito = autorito.trim().to_string();
        print!("Tipo: ");
        io::stdout().flush().unwrap();
        let mut tipito = String::new();
        io::stdin().read_line(&mut tipito).unwrap();
        tipito = tipito.trim().to_string();
        print!("Año: ");
        io::stdout().flush().unwrap();
        let mut anito = String::new();
        io::stdin().read_line(&mut anito).unwrap();
        let anito = match anito.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => 0
        };
        print!("Descripcion: ");
        io::stdout().flush().unwrap();
        let mut descsito = String::new();
        io::stdin().read_line(&mut descsito).unwrap();
        descsito = descsito.trim().to_string();
        Obra {
            titulo: titulito,
            autor: autorito,
            tipo: tipito,
            anio: anito,
            desc: descsito
        }
    }
    pub fn get_titulo(&self) -> String {
        self.titulo.clone()
    }
    fn set_titulo(&mut self, valor: &mut String) {
        self.titulo = valor.to_string()
    }
    pub fn get_autor(&self) -> String {
        self.autor.clone()
    }
    fn set_autor(&mut self, valor: &mut String) {
        self.autor = valor.to_string()
    }
    pub fn get_tipo(&self) -> String {
        self.tipo.clone()
    }
    fn set_tipo(&mut self, valor: &mut String) {
        self.tipo = valor.to_string()
    }
    pub fn get_anio(&self) -> i32 {
        self.anio
    }
    fn set_anio(&mut self, valor: i32) {
        self.anio = valor
    }
    pub fn get_desc(&self) -> String {
        self.desc.clone()
    }
    fn set_desc(&mut self, valor: &mut String) {
        self.desc = valor.to_string()
    }
    pub fn obra_menu(&mut self) {
        println!("\nMenú de Obra {}", self.get_titulo());
        println!("1) Editar Título.");
        println!("2) Editar Autor.");
        println!("3) Editar Tipo.");
        println!("4) Editar Año.");
        println!("5) Editar Descripción.");
        print!("Seleccione una opción: ");
        io::stdout().flush().unwrap();
        let mut opc = String::new();
        io::stdin().read_line(&mut opc).unwrap();
        let opc = match opc.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => 0
        };
        if opc < 1 || opc > 5 {
            println!("Opción no válida.");
            return;
        }
        print!("\nInserte el nuevo valor: ");
        io::stdout().flush().unwrap();
        let mut valor = String::new();
        io::stdin().read_line(&mut valor).unwrap();
        let valor = match valor.trim().parse::<i32>() {
            Ok(num) => match opc {
                4 => Ok(num),
                _ => Err(valor.trim().to_string())
            },
            Err(_) => match valor.trim().parse::<String>() {
                Ok(str) => Err(str),
                Err(_) => Err(" ".to_string())
            }
        };
        match valor {
            Err(mut str) => match opc {
                1 => self.set_titulo(&mut str),
                2 => self.set_autor(&mut str),
                3 => self.set_tipo(&mut str),
                5 => self.set_desc(&mut str),
                _ => return
            },
            Ok(num) => self.set_anio(num)
        }
    }
}