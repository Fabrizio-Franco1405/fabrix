#![no_std] // no enlazar con la biblioteca estándar de Rust
#![no_main] // deshabilitar todos los puntos de entrada a nivel de Rust

use core::panic::PanicInfo;

/// Esta función se llama cuando ocurre un pánico.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hola mundo!";

#[unsafe(no_mangle)] // no modificar el nombre de esta función
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0b; // atributo de}
        }
    }
    
    // Llamada `_start` por defecto
    loop {}
}