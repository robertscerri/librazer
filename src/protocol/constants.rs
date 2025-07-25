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
    Breathing(BreathingMode),
    Spectrum,
    Custom,
    Static(RGB),
    Starlight(StarlightMode, StarlightSpeed),
}

impl From<MatrixEffect> for u8 {
    fn from(value: MatrixEffect) -> Self {
        match value {
            MatrixEffect::Off => 0x00,
            MatrixEffect::Wave(_) => 0x01,
            MatrixEffect::Reactive(_, _) => 0x02,
            MatrixEffect::Breathing(_) => 0x03,
            MatrixEffect::Spectrum => 0x04,
            MatrixEffect::Custom => 0x05, //TODO: Implement effect.
            MatrixEffect::Static(_) => 0x06,
            MatrixEffect::Starlight(_, _) => 0x19,
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
    Fastest = 0x01,
    Fast = 0x02,
    Slow = 0x03,
    Slowest = 0x04,
}

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum BreathingMode {
    Single(RGB),
    Dual(RGB, RGB),
    Random,
}

impl From<BreathingMode> for u8 {
    fn from(value: BreathingMode) -> Self {
        match value {
            BreathingMode::Single(_) => 0x01,
            BreathingMode::Dual(_, _) => 0x02,
            BreathingMode::Random => 0x03,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StarlightMode {
    Single(RGB),
    Dual(RGB, RGB),
    Random,
}

impl From<StarlightMode> for u8 {
    fn from(value: StarlightMode) -> Self {
        match value {
            StarlightMode::Single(_) => 0x01,
            StarlightMode::Dual(_, _) => 0x02,
            StarlightMode::Random => 0x03,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum StarlightSpeed {
    Fast = 0x01,
    Medium = 0x02,
    Slow = 0x03,
}
