#![no_std]
#![feature(start)]
#![allow(unused_imports)]

mod io;
mod ppu;
mod game;

// gym nest ascii 4block

// TODO
// sprites
// 0x800 of ram
// 0x100 of OAM
// remove unneeded derives

// memory usage;
// 0xE - buttons
// 0xF - nmi check bit
// 0x300+ - game state

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {

    game::allocate_state();

    ppu::write_addr(0x2042);
    ppu::draw_text("Public domain roms??");

    ppu::write_addr(0x22C5);
    ppu::draw_text("very cool (~'v')~");

    ppu::write_ctrl(0x80);
    ppu::write_mask(0x1E);

    loop {
        io::wait_for_vblank();
        game::frame();
    }
}

#[no_mangle]
pub extern "C" fn render()  {
    game::render();

    io::poll_controller();

    let p = 0xF0 as *mut u8;
    unsafe { *p += 1; }

    ppu::write_addr(0x2180 + (unsafe { *p >> 2 } % 32) as u16);
    ppu::draw_text(" WOW! ");

    ppu::write_addr(0x21A0);
    for _ in 0..0x4 {
        ppu::draw_text(" ");
    }

    // reset scroll
    ppu::write_addr(0x2000);
    ppu::scroll(0, 0);
}


#[link_section = ".chr_rom"]
#[no_mangle]
pub static TILES: [u8; 4096] = *include_bytes!("./chr/tiles.chr");

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut p = 0xE0 as *mut u8;
    let message = info.payload().downcast_ref::<&str>().unwrap_or(&"!!PANIC!!");
    for ch in message.chars() {
        unsafe {
            *p = ch as u8;
            p = p.add(1);
        }
    }
    loop {}
}
