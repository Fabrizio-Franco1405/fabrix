pub mod init;

use crate::libs::stack;

pub fn init() {
    let mut stack = stack::Stack::new(); // Crear instancia del Stack

    stack.push(42); // Empujar valor 42 al Stack
    let val = stack.pop(); // Extraer valor del Stack

    if let Some(v) = val {
        // Imprimir el valor si se logra extraer
        
    }
}