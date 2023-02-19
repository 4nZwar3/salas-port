mod obra;
mod sala;
use sala::*;
fn main() {
    let mut sala = Sala::new("Tularmagedon");
    sala.menu();
}
