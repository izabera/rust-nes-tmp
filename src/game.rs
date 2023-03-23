use meta_nestris::{State, Input, GameType, Modifier};

static STATE_ADDR: u16 = 0x300;

fn state_ptr() ->
    *mut State<{ Modifier {
        uncapped_score: true,
        select_adds_20_levels: true,
    } }>
{
    STATE_ADDR as _
}

pub fn allocate_state() {
    unsafe {
        *state_ptr() = State::new_with_modifier();
    }
}

pub fn frame() {

}

pub fn render() {

}
