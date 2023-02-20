mod obra;
mod sala;
mod museo;
use museo::*;
fn main() {
    let mut museo = Museo::new("museo");
    museo.menu();
}
