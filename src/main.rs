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

static PPUSTATUS: usize = 0x2002;
static PPUCTRL: usize = 0x2000;

fn wait_for_vblank() {
    let status = PPUSTATUS as *const u8;
    loop {
        if unsafe { *status } & 0x80 != 0 { break; }
    }
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

    let ctrl = PPUCTRL as *mut u8;
    unsafe { *ctrl = 0x80; }
    loop {

        unsafe { wait_vblank(); }
        unsafe {
            *p += 1;
        }
    }
    0
}

fn ppu_addr(high: u8, low: u8) {
    let p = 0x2006 as *mut u8;
    unsafe {
        *p = high;
        *p = low;
    }
}

fn ppu_data(value: u8) {
    let p = 0x2007 as *mut u8;
    unsafe { *p = value; }
}

fn ppu_scroll(x: u8, y: u8) {
    let p = 0x2005 as *mut u8;
    unsafe {
        *p = x;
        *p = y;
    }
}


#[no_mangle]
pub unsafe extern "C" fn render()  {
    let p = 0xF0 as *mut u8;
    unsafe { *p += 1; };


    ppu_addr(0x20, 0x05);
    ppu_data(5);
    ppu_data(6);
    // ppu_scroll(0, 0);
}


#[link_section = ".chr_rom"]
#[no_mangle]
pub static TILES: [u8; 4096] = *include_bytes!("./tiles.chr");
