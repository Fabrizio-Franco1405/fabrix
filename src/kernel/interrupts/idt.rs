use core::arch::asm;
use core::mem::size_of;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: u16,
    options: u16,
    pointer_mid: u16,
    pointer_high: u32,
    reserved: u32,
}

impl Entry {
    pub fn missing() -> Self {
        Self {
            pointer_low: 0,
            gdt_selector: 0,
            options: 0,
            pointer_mid: 0,
            pointer_high: 0,
            reserved: 0,
        }
    }

    pub fn new(handler_addr: u64) -> Self {
        let mut entry = Self::missing();
        entry.set_handler_addr(handler_addr);

        entry.gdt_selector = 0x08;
        entry.options = 0x8E00;

        entry
    }

    fn set_handler_addr(&mut self, address: u64) {
        self.pointer_low = address as u16;
        self.pointer_mid = (address >> 16) as u16;
        self.pointer_high = (address >> 32) as u32;
    }
}

pub struct InterruptDescriptorTable {
    entries: [Entry; 256],
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        Self {
            entries: [Entry::missing(); 256],
        }
    }

    pub fn set_entry(&mut self, index: usize, entry: Entry) {
        self.entries[index] = entry;
    }

    pub fn get_entry(&self, index: usize) -> &Entry {
        &self.entries[index]
    }

    pub fn load(&'static self) {
        let descriptor = IdtDescriptor {
            limit: (size_of::<Self>() - 1) as u16,
            base: self as *const _ as u64,
        };

        unsafe {
            asm!("lidt [{}]", in(reg) &descriptor, options(readonly, nostack, preserves_flags));
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtDescriptor {
    limit: u16,
    base: u64,
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let idt = InterruptDescriptorTable::new();

        
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}