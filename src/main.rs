#![no_std]
#![feature(start)]
#![allow(unused_imports)]

use core::panic::PanicInfo;

// gym nest ascii 4block

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
    let p2 = 0xF1 as *mut u8;
        unsafe { *p2 = 1; };
    let ctrl = PPUCTRL as *mut u8;
    unsafe {
        *ctrl = 0x80;
    }
        unsafe { *p2 = 2; };
    loop {

        unsafe { *p2 = 3; };
        wait_for_vblank();
        unsafe { *p2 = 4; };
        unsafe {
            *p = '#' as u8;
            p = p.add(1);
        }
        unsafe { *p2 = 5; };
    }
    0
}

// "The techniques llvm-mos uses for interrupt handling are somewhat unusual" - https://llvm-mos.org/wiki/C_interrupts

extern "C" {
    fn foo_c();
}

#[no_mangle]
pub unsafe extern "C" fn nmi_fn()  {
    let p = 0xF0 as *mut u8;
    foo_c();
    unsafe { *p += 1; };
}

// #[link_section = ".nmi.foo"]
// #[no_mangle]
// pub static NMI_VECTOR: unsafe extern "C" fn() = nmi_fn;

// #[link_section = ".chr_rom"]
// #[no_mangle]
// pub static POOP: &[u8] = &[ 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, ];
