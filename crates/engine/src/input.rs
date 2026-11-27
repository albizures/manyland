//! Button state: which of the eight buttons are held on a tick.

use core::fmt;

/// One of the eight buttons. The order is the bit order and the replay order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

impl Button {
    pub const ALL: [Button; 8] = [
        Button::Up,
        Button::Down,
        Button::Left,
        Button::Right,
        Button::A,
        Button::B,
        Button::Start,
        Button::Select,
    ];

    /// The lowercase name used in replays.
    pub fn name(self) -> &'static str {
        match self {
            Button::Up => "up",
            Button::Down => "down",
            Button::Left => "left",
            Button::Right => "right",
            Button::A => "a",
            Button::B => "b",
            Button::Start => "start",
            Button::Select => "select",
        }
    }

    pub fn from_name(name: &str) -> Option<Button> {
        Button::ALL.into_iter().find(|b| b.name() == name)
    }

    fn bit(self) -> u8 {
        let index = Button::ALL
            .iter()
            .position(|&b| b == self)
            .expect("every button is in ALL");
        1 << index
    }
}

/// The set of buttons held on one tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ButtonState(u8);

impl ButtonState {
    pub const NONE: ButtonState = ButtonState(0);

    pub fn with(self, button: Button) -> Self {
        ButtonState(self.0 | button.bit())
    }

    pub fn held(self, button: Button) -> bool {
        self.0 & button.bit() != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl FromIterator<Button> for ButtonState {
    fn from_iter<I: IntoIterator<Item = Button>>(iter: I) -> Self {
        iter.into_iter().fold(ButtonState::NONE, ButtonState::with)
    }
}

/// Replay syntax: button names in [`Button::ALL`] order, or `-` for none.
impl fmt::Display for ButtonState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return f.write_str("-");
        }
        let names: Vec<&str> = Button::ALL
            .into_iter()
            .filter(|&b| self.held(b))
            .map(Button::name)
            .collect();
        f.write_str(&names.join(" "))
    }
}

/// A tick's input: the button state now and on the previous tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Input {
    now: ButtonState,
    before: ButtonState,
}

impl Input {
    /// Moves to the next tick, whose button state is `now`.
    pub fn advance(&mut self, now: ButtonState) {
        self.before = self.now;
        self.now = now;
    }

    pub fn held(&self, button: Button) -> bool {
        self.now.held(button)
    }

    /// Held now, not held on the previous tick.
    pub fn pressed(&self, button: Button) -> bool {
        self.now.held(button) && !self.before.held(button)
    }

    /// Held on the previous tick, not held now.
    pub fn released(&self, button: Button) -> bool {
        !self.now.held(button) && self.before.held(button)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip_and_bits_are_distinct() {
        let mut all = ButtonState::NONE;
        for b in Button::ALL {
            assert_eq!(Button::from_name(b.name()), Some(b));
            assert!(!all.held(b));
            all = all.with(b);
        }
        assert_eq!(all, ButtonState(0xff));
        assert_eq!(Button::from_name("x"), None);
    }

    #[test]
    fn pressed_and_released_compare_with_previous_tick() {
        let a: ButtonState = [Button::A].into_iter().collect();
        let mut input = Input::default();

        input.advance(a);
        assert!(input.held(Button::A) && input.pressed(Button::A));

        input.advance(a);
        assert!(input.held(Button::A) && !input.pressed(Button::A));

        input.advance(ButtonState::NONE);
        assert!(!input.held(Button::A) && input.released(Button::A));
    }

    #[test]
    fn adding_a_held_button_changes_nothing() {
        let a = ButtonState::NONE.with(Button::A);
        assert_eq!(a.with(Button::A), a);
        let twice: ButtonState = [Button::A, Button::A].into_iter().collect();
        assert_eq!(twice, a);
    }

    /// Exhaustive over every (previous, current) state pair and button: a
    /// button is pressed or released exactly when its held state changed,
    /// pressed means held now, released means not held now.
    #[test]
    fn pressed_and_released_hold_for_every_transition() {
        for before in 0..=u8::MAX {
            for now in 0..=u8::MAX {
                let mut input = Input::default();
                input.advance(ButtonState(before));
                input.advance(ButtonState(now));
                for b in Button::ALL {
                    let (pressed, released) = (input.pressed(b), input.released(b));
                    let changed = ButtonState(before).held(b) != ButtonState(now).held(b);
                    let case = format!("before {before:#010b}, now {now:#010b}, {b:?}");
                    assert_eq!(pressed || released, changed, "{case}");
                    assert!(!pressed || input.held(b), "pressed but not held: {case}");
                    assert!(!released || !input.held(b), "released but held: {case}");
                }
            }
        }
    }

    #[test]
    fn displays_in_replay_syntax() {
        let state: ButtonState = [Button::A, Button::Right].into_iter().collect();
        assert_eq!(state.to_string(), "right a");
        assert_eq!(ButtonState::NONE.to_string(), "-");
    }
}
