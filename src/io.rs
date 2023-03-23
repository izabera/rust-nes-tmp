// need to import at least one C function to force the linker to work (?)
extern "C" {
    fn wait_vblank();
}

pub fn wait_for_vblank() {
    unsafe { wait_vblank() };
}

static JOYPAD1: u16 = 0x4016;
static BUTTONS: u16 = 0xE;

pub fn poll_controller() {
    let joy = JOYPAD1 as *mut u8;
    let buttons = BUTTONS as *mut u8;

    // TODO: https://www.nesdev.org/wiki/Controller_reading_code#DPCM_Safety_using_Repeated_Reads

    unsafe {
        core::ptr::write_volatile(joy, 1);
        core::ptr::write_volatile(joy, 0);

        for _ in 0..8 {
            let a = core::ptr::read_volatile(joy);
            *buttons <<= 1;
            *buttons |= a & 1;
        }
    }
}

pub fn controller_buttons() -> u8 {
    unsafe { *(BUTTONS as *mut u8) }
}
