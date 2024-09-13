use tiny_led_matrix::Render;

type Instant = u64;

#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum AppMode {
    #[default]
    Idle,
    Running(Instant),
    Sending,
    Error
}

impl Render for AppMode {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        match self {
            AppMode::Idle => IDLE_SPRITE[y][x],
            AppMode::Running(_) => RUNNING_SPRITE[y][x],
            AppMode::Sending => SENDING_SPRITE[y][x],
            AppMode::Error => ERROR_SPRITE[y][x],
        }
    }
}

const IDLE_SPRITE: [[u8;5];5] = [
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
    [5, 5, 0, 5, 5],
];

const RUNNING_SPRITE: [[u8;5];5] = [
    [0, 5, 0, 0, 0],
    [0, 5, 5, 0, 0],
    [0, 5, 5, 5, 0],
    [0, 5, 5, 0, 0],
    [0, 5, 0, 0, 0],
];

const SENDING_SPRITE: [[u8;5];5] = [
    [0, 0, 5, 0, 0],
    [0, 5, 5, 5, 0],
    [5, 0, 5, 0, 5],
    [0, 0, 5, 0, 0],
    [0, 0, 5, 0, 0],
];


const ERROR_SPRITE: [[u8;5];5] = [
    [5, 5, 5, 5, 5],
    [0, 5, 5, 5, 0],
    [0, 0, 5, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 5, 0, 0],
];