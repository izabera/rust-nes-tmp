// need to import at least one C function to force the linker to work (?)
extern "C" {
    fn wait_vblank();
}

pub fn wait_for_vblank() {
    unsafe { wait_vblank() };
}
