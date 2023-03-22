// need to import at least one C function to force the linker to work (?)
extern "C" {
    fn wait_vblank();
}

pub fn wait_for_vblank() {
    unsafe { wait_vblank() };
}

pub fn ppu_ctrl(value: u8) {
    let p = 0x2000 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn ppu_mask(value: u8) {
    let p = 0x2001 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn ppu_addr(value: u16) {
    let p = 0x2006 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, (value >> 8) as u8);
        core::ptr::write_volatile(p, value as u8);
    }
}

pub fn ppu_data(value: u8) {
    let p = 0x2007 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn ppu_scroll(x: u8, y: u8) {
    let p = 0x2005 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, x);
        core::ptr::write_volatile(p, y);
    }
}

pub fn draw_text(text: &str) {
    for ch in text.chars() {
        ppu_data(ch as u8 - 55);
    }
}
