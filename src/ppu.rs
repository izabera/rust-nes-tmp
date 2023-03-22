pub fn write_ctrl(value: u8) {
    let p = 0x2000 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn write_mask(value: u8) {
    let p = 0x2001 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn write_addr(value: u16) {
    let p = 0x2006 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, (value >> 8) as u8);
        core::ptr::write_volatile(p, value as u8);
    }
}

pub fn write_data(value: u8) {
    let p = 0x2007 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn scroll(x: u8, y: u8) {
    let p = 0x2005 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, x);
        core::ptr::write_volatile(p, y);
    }
}


#[inline(never)]
pub fn draw_text(text: &str) {
    for ch in text.chars() {
        write_data(ch as u8 - 32);
    }
}
