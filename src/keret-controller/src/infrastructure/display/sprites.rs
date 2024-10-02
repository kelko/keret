use crate::domain::model::AppMode;
use tiny_led_matrix::Render;

/// mapping of an domain AppMode to the sprite that shall be shown on the display.
/// Render is a trait defined by the underlying Display module
impl Render for AppMode {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        match self {
            AppMode::Idle => IDLE_SPRITE[y][x],
            AppMode::Running(_) => RUNNING_SPRITE[y][x],
            AppMode::Error => ERROR_SPRITE[y][x],
        }
    }
}

/// simple struct to hold a 5x5 byte matrix which can be shown on the Display
#[repr(transparent)]
pub(crate) struct DisplayMode([[u8; 5]; 5]);

impl Render for DisplayMode {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }
}

/// the sprite to show while the app idles ("pause" icon)
const IDLE_SPRITE: [[u8; 5]; 5] = [
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
];

/// the sprite to show while the app is running ("play" icon)
const RUNNING_SPRITE: [[u8; 5]; 5] = [
    [0, 5, 0, 0, 0],
    [0, 5, 5, 0, 0],
    [0, 5, 5, 5, 0],
    [0, 5, 5, 0, 0],
    [0, 5, 0, 0, 0],
];

/// the sprite to show if the app is in an error mode (exclamation mark)
const ERROR_SPRITE: [[u8; 5]; 5] = [
    [5, 5, 5, 5, 5],
    [0, 5, 5, 5, 0],
    [0, 0, 5, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 5, 0, 0],
];

/// the sprite to show if the app ran into a fatal error it can't recover from
/// (a large X)
pub(crate) const FATAL_SPRITE: DisplayMode = DisplayMode([
    [5, 0, 0, 0, 5],
    [0, 5, 0, 5, 0],
    [0, 0, 5, 0, 0],
    [0, 5, 0, 5, 0],
    [5, 0, 0, 0, 5],
]);
