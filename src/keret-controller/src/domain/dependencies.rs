/// enum to indicate the users desired interaction
/// which is calculated by which button was pressed
#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum InteractionRequest {
    #[default]
    None,
    ToggleMode,
    Reset,
}
