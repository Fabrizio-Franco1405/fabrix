use crate::drivers::vga;
use crate::drivers::vga::Color;
use crate::kernel::interrupts;

pub fn start() -> ! {
    vga::write_string("Fabrix Kernel v0.1.0", Color::BLUE);
    
    interrupts::init_idt();
    vga::write_string(" -> IDT Cargada correctamente!", Color::WHITE);

    loop {
        core::hint::spin_loop();
    }
}