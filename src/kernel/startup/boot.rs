use crate::drivers::vga;
use crate::drivers::vga::Color;

pub fn start() -> ! {
    vga::write_string("Fabrix Kernel v0.1.0", Color::BLUE);
    
    // Próximamente:
    // crate::kernel::interrupts::init_idt();

    loop {
        core::hint::spin_loop();
    }
}