use meta_nestris::{State, Input, GameType, Modifier, MenuMode};

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
    let input = Input::from(crate::io::controller_buttons());
    unsafe {
        (*state_ptr()).step(input)
    };
}

use crate::ppu;

pub fn render() {
    ppu::write_addr(0x2205);


    match  unsafe { &*state_ptr() } {
        State::MenuState(state) => {
            use MenuMode::*;
            let text = match state.menu_mode {
                CopyrightScreen => "legal",
                TitleScreen => "title",
                GameTypeSelect => "game type",
                LevelSelect => "levelselect",
                InitializingGame => "???",
            };
            ppu::draw_text(text);
        },
        State::GameplayState(state) => {
            ppu::draw_text("GAME");
        },
    }
}
