const STACK_SIZE: usize = 1024 * 1024; // 1MB

pub struct Stack {
    top: *mut u8, // Apunta al último valor apilado
    base: *mut u8, // Apunta al inicio del Stack
}

impl Stack {
    //Inicializar Stack
    pub fn new() -> Self {
        let mut stack_memory: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let base = stack_memory.as_mut_ptr();
        let top = unsafe { base.add(STACK_SIZE) }; // Apunta al final del

        Stack { top, base }
    }

    // Empuja un valor en el Stack
    pub fn push(&mut self, value: u8) {
        unsafe {
            //Decrementar el puntero top
            self.top = self.top.sub(1);
            *self.top = value;
        }
    }

    // Extrae un valor del stack
    pub fn pop(&mut self) -> Option<u8> {
        if self.top != self.base {
            unsafe {
                let value = *self.top;
                self.top = self.top.add(1);
                Some(value)
            }
        } else {
            None
        }
    }
}