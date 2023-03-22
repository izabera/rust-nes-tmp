use bitmask_enum::bitmask;

#[bitmask(u8)]
pub enum Input {
    None = 0,
    Right = 0x01,
    Left = 0x02,
    Down = 0x04,
    Up = 0x08,
    Start = 0x10,
    Select = 0x20,
    B = 0x40,
    A = 0x80,
}

impl Input {
    #[must_use]
    pub fn new() -> Self {
        Self::None
    }

    #[must_use]
    pub fn get(self, button: Input) -> bool {
        self & button != 0
    }

    #[must_use]
    pub fn get_only_input(self, button: Input) -> bool {
        self == button
    }

    pub fn set(mut self, button: Input, state: bool) {
        if state {
            self |= button;
        } else {
            self &= !button;
        }
    }

    #[must_use]
    pub fn difference(self, other: Input) -> Input {
        self & !other
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
