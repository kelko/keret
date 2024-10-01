use crate::domain::AppMode;
use tiny_led_matrix::Render;

impl Render for AppMode {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        match self {
            AppMode::Idle => IDLE_SPRITE[y][x],
            AppMode::Running(_) => RUNNING_SPRITE[y][x],
            AppMode::Error => ERROR_SPRITE[y][x],
        }
    }
}

#[repr(transparent)]
pub(crate) struct DisplayMode([[u8; 5]; 5]);

impl Render for DisplayMode {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }
}

const IDLE_SPRITE: [[u8; 5]; 5] = [
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
];

const RUNNING_SPRITE: [[u8; 5]; 5] = [
    [0, 5, 0, 0, 0],
    [0, 5, 5, 0, 0],
    [0, 5, 5, 5, 0],
    [0, 5, 5, 0, 0],
    [0, 5, 0, 0, 0],
];

const ERROR_SPRITE: [[u8; 5]; 5] = [
    [5, 5, 5, 5, 5],
    [0, 5, 5, 5, 0],
    [0, 0, 5, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 5, 0, 0],
];

pub(crate) const FATAL_SPRITE: DisplayMode = DisplayMode([
    [5, 0, 0, 0, 5],
    [0, 5, 0, 5, 0],
    [0, 0, 5, 0, 0],
    [0, 5, 0, 5, 0],
    [5, 0, 0, 0, 5],
]);
