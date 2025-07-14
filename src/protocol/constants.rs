pub const RZ_REPORT_LEN: usize = 90;

#[derive(Debug)]
#[repr(u8)]
pub enum LedState {
    Off,
    On,
}

#[derive(Debug)]
#[repr(u8)]
pub enum LedStorage {
    NoStore,
    VarStore,
}

#[derive(Debug)]
#[repr(u8)]
pub enum LedDefinitions {
    Zero = 0x00,
    ScrollWheel = 0x01,
    Battery = 0x03,
    Logo = 0x04,
    Backlight = 0x05,
    Macro = 0x07,
    Game = 0x08,
    RedProfile = 0x0C,
    GreenProfile = 0x0D,
    BlueProfile = 0x0E,
    RightSide = 0x10,
    LeftSide = 0x11,
    ARGBChannel1 = 0x1A,
    ARGBChannel2 = 0x1B,
    ARGBChannel3 = 0x1C,
    ARGBChannel4 = 0x1D,
    ARGBChannel5 = 0x1E,
    ARGBChannel6 = 0x1F,
    Charging = 0x20,
    FastCharging = 0x21,
    FullyCharged = 0x22,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MatrixEffect {
    Off,
    Wave(WaveDirection),
    Reactive(ReactiveSpeed, RGB),
    Breathing,
    Spectrum,
    Custom,
    Static(RGB),
    Starlight,
}

impl From<MatrixEffect> for u8 {
    fn from(value: MatrixEffect) -> Self {
        match value {
            MatrixEffect::Off => 0x00,
            MatrixEffect::Wave(_) => 0x01,
            MatrixEffect::Reactive(_, _) => 0x02,
            MatrixEffect::Breathing => 0x03, //TODO: Implement effect.
            MatrixEffect::Spectrum => 0x04,
            MatrixEffect::Custom => 0x05, //TODO: Implement effect.
            MatrixEffect::Static(_) => 0x06,
            MatrixEffect::Starlight => 0x19, //TODO: Implement effect.
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum WaveDirection {
    Right = 0x01,
    Left = 0x02,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ReactiveSpeed {
    Slowest,
    Slow,
    Fast,
    Fastest,
}

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
