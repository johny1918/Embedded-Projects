#[derive(Debug, Clone, Copy)]
pub enum LedState {
    Off,
    On,
    FastBlink,
    SlowBlink,
}

#[derive(Debug, Clone, Copy)]
pub struct LedStateMachine {
    state: LedState,
}

impl LedState {
    pub fn next(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::FastBlink,
            Self::FastBlink => Self::SlowBlink,
            Self::SlowBlink => Self::Off,
        }
    }
}

impl LedStateMachine {
    pub fn new(state: LedState) -> Self {
        Self { state }
    }

    pub fn next_state(&mut self) {
        self.state = self.state.next();
    }

    pub fn get_state(&self) -> LedState {
        self.state
    }
}
