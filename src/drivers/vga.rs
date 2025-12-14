const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

pub struct Color(pub u8);

#[allow(dead_code)]
impl Color {
    pub const BLUE: Color = Color(0x0b);
    pub const WHITE: Color = Color(0x07);
}

pub fn write_string(text: &str, color: Color) {
    for (i, byte) in text.bytes().enumerate() {
        write_byte(i, byte, color.0);
    }
}

pub fn write_byte(offset: usize, byte: u8, color: u8) {
    unsafe {
        *VGA_BUFFER.offset((offset * 2) as isize) = byte;
        *VGA_BUFFER.offset((offset * 2 + 1) as isize) = color;
    }
}