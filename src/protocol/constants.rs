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
