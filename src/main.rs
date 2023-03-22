#![no_std]
#![feature(start)]
#![allow(unused_imports)]

use core::panic::PanicInfo;

// gym nest ascii 4block

// need to import at least one functions to force the C file to link properly (?)
extern "C" {
    fn wait_vblank();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: panic handler
    let mut p = 0xE0 as *mut u8;
    for ch in "panic".chars() {
        unsafe {
            *p = ch as u8;
            p = p.add(1);
        }
    }
    loop {}
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut p = 0x100 as *mut u8;
    for ch in "ram write test".chars() {
        unsafe {
            *p = ch as u8;
            p = p.add(1);
        }
    }

    ppu_ctrl(0x80);
    ppu_mask(0x1E);

    loop {

        unsafe { wait_vblank(); }
        unsafe {
            *p += 1;
        }
    }
}

fn ppu_ctrl(value: u8) {
    let p = 0x2000 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

fn ppu_mask(value: u8) {
    let p = 0x2001 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

fn ppu_addr(value: u16) {
    let p = 0x2006 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, (value >> 8) as u8);
        core::ptr::write_volatile(p, value as u8);
    }
}

fn ppu_data(value: u8) {
    let p = 0x2007 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

fn ppu_scroll(x: u8, y: u8) {
    let p = 0x2005 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, x);
        core::ptr::write_volatile(p, y);
    }
}


#[no_mangle]
pub unsafe extern "C" fn render()  {
    let p = 0xF0 as *mut u8;
    unsafe { *p += 1; };

    ppu_addr(0x2008);

    for i in 0x2000..0x2040 {
        ppu_data(0xFF);

    }

    for ch in "HELLO WORLD".chars() {
        ppu_data(ch as u8 - 65 + 10);
    }

    for i in 0..10 {
        ppu_data(0xFF);

    }


    ppu_addr(0x2000); // reset scroll
    ppu_scroll(0, 0);
}


#[link_section = ".chr_rom"]
#[no_mangle]
pub static TILES: [u8; 4096] = *include_bytes!("./tiles.chr");
