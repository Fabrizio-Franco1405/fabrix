use crate::drivers::vga;
use crate::drivers::vga::Color;

pub fn start() {
    vga::write_string("Fabrix Kernel iniciado", Color::BLUE);
}
