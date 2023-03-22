#![no_std]
#![feature(start)]
#![allow(unused_imports)]

mod io;

use io::*;

use core::panic::PanicInfo;

// gym nest ascii 4block


// TODO
// controller
// sprites
// allocate / meta

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

    ppu_ctrl(0x80);
    ppu_mask(0x1E);

    loop {

        wait_for_vblank();
        unsafe {
            *p += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn render()  {
    let p = 0xF0 as *mut u8;
    unsafe { *p += 1; }

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

    // reset scroll
    ppu_addr(0x2000);
    ppu_scroll(0, 0);
}


#[link_section = ".chr_rom"]
#[no_mangle]
pub static TILES: [u8; 4096] = *include_bytes!("./chr/tiles.chr");
