use rusb::{
    constants::{LIBUSB_REQUEST_CLEAR_FEATURE, LIBUSB_REQUEST_SET_CONFIGURATION},
    request_type,
};

use crate::{
    device::usb_device::USBDevice,
    protocol::razer_report::{RazerReport, RZ_REPORT_LEN},
    utils::errors::Result,
};

const RAZER_VENDOR_ID: u16 = 0x1532;
const CONTROL_REPORT_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(2000);

pub trait RazerDevice {
    fn usb_device(&mut self) -> &mut USBDevice;

    fn open(&mut self) -> Result<()> {
        //TODO: Claim interface here, see legacy code for reference.
        self.usb_device().open()
    }

    fn close(&mut self) -> Result<()> {
        //TODO: Release interface here, see legacy code for reference.
        self.usb_device().close()
    }

    fn send_report(&mut self, report: RazerReport) -> Result<()> {
        let data: [u8; RZ_REPORT_LEN] = report.into();

        //TODO: Use more idiomatic constants for wValue and wIndex.
        self.usb_device().write_control(
            request_type(
                rusb::Direction::Out,
                rusb::RequestType::Class,
                rusb::Recipient::Interface,
            ),
            LIBUSB_REQUEST_SET_CONFIGURATION,
            0x300,
            0x00,
            &data,
            CONTROL_REPORT_TIMEOUT,
        )?;

        Ok(())
    }

    fn read_report(&mut self) -> Result<RazerReport> {
        //TODO: Use more idiomatic constants for wValue and wIndex.
        let buf: Vec<u8> = self.usb_device().read_control(
            request_type(
                rusb::Direction::In,
                rusb::RequestType::Class,
                rusb::Recipient::Interface,
            ),
            LIBUSB_REQUEST_CLEAR_FEATURE,
            0x300,
            0x00,
            CONTROL_REPORT_TIMEOUT,
        )?;

        //TODO: Sort out buffer messiness
        let mut data: [u8; RZ_REPORT_LEN] = [0; RZ_REPORT_LEN];
        data[0..RZ_REPORT_LEN].copy_from_slice(buf.as_slice());

        let report = data.try_into()?;

        Ok(report)
    }
}

//Device definitions
macro_rules! define_razer_device {
    (
        $(
            $name: ident = $product_id:expr
        ),* $(,)?
    ) => {
        $(
            pub struct $name {
                usb_device: USBDevice
            }

            impl $name {
                pub fn new() -> Self {
                    let usb_device = USBDevice::new(
                        RAZER_VENDOR_ID,
                        $product_id,
                    );

                    $name { usb_device }
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    $name::new()
                }
            }

            impl RazerDevice for $name {
                fn usb_device(&mut self) -> &mut USBDevice {
                    &mut self.usb_device
                }
            }
        )*
    }
}

define_razer_device!(
    //Mice
    Irichu2011 = 0x0013,
    DeathAdder3_5g = 0x0016,
    Abyssus1800 = 0x0020,
    Mamba2012Wired = 0x0024,
    Mamba2012Wireless = 0x0025,
    DeathAdder3_5gBlack = 0x0029,
    Naga2012 = 0x002E,
    Imperator = 0x002F,
    Ouroboros = 0x0032,
    Taipan = 0x0034,
    NagaHexRed = 0x0036,
    DeathAdder2013 = 0x0037,
    DeathAdder1800 = 0x0038,
    Orochi2013 = 0x0039,
    NagaEpicChroma = 0x003E,
    NagaEpicChromaDock = 0x003F,
    Naga2014 = 0x0040,
    NagaHex = 0x0041,
    Abyssus = 0x0042,
    DeathAdderChroma = 0x0043,
    MambaWired = 0x0044,
    MambaWireless = 0x0045,
    MambaTEWired = 0x0046,
    OrochiChroma = 0x0048,
    DiamondbackChroma = 0x004C,
    DeathAdder2000 = 0x004F,
    NagaHexV2 = 0x0050,
    NagaChroma = 0x0053,
    DeathAdder3500 = 0x0054,
    LanceheadWired = 0x0059,
    LanceheadWireless = 0x005A,
    AbyssusV2 = 0x005B,
    DeathAdderElite = 0x005C,
    Abyssus2000 = 0x005E,
    LanceheadTEWired = 0x0060,
    AtherisReceiver = 0x0062,
    Basilisk = 0x0064,
    BasiliskEssential = 0x0065,
    NagaTrinity = 0x0067,
    AbyssusEliteDvaEdition = 0x006A,
    AbyssusEssential = 0x006B,
    MambaElite = 0x006C,
    DeathAdderEssential = 0x006E,
    LanceheadWirelessReceiver = 0x006F,
    LanceheadWirelessWired = 0x0070,
    DeathAdderEssentialWhiteEdition = 0x0071,
    MambaWirelessReceiver = 0x0072,
    MambaWirelessWired = 0x0073,
    ProClickReceiver = 0x0077,
    Viper = 0x0078,
    ViperUltimateWired = 0x007A,
    ViperUltimateWireless = 0x007B,
    DeathAdderV2ProWired = 0x007C,
    DeathAdderV2ProWireless = 0x007D,
    ProClickWired = 0x0080,
    BasiliskXHyperspeed = 0x0083,
    DeathAdderV2 = 0x0084,
    BasiliskV2 = 0x0085,
    BasiliskUltimateWired = 0x0086,
    BasiliskUltimateReceiver = 0x0088,
    ViperMini = 0x008A,
    DeathAdderV2Mini = 0x008C,
    NagaLeftHanded2020 = 0x008D,
    NagaProWired = 0x008F,
    NagaProWireless = 0x0090,
    Viper8k = 0x0091,
    OrochiV2Receiver = 0x0094,
    OrochiV2Bluetooth = 0x0095,
    NagaX = 0x0096,
    DeathAdderEssential2021 = 0x0098,
    BasiliskV3 = 0x0099,
    ProClickMiniReceiver = 0x009A,
    DeathAdderV2XHyperspeed = 0x009C,
    ViperMiniSEWired = 0x009E,
    ViperMiniSEWireless = 0x009F,
    DeathAdderV2Lite = 0x00A1,
    Cobra = 0x00A3,
    ViperV2ProWired = 0x00A5,
    ViperV2ProWireless = 0x00A6,
    BasiliskV3ProWired = 0x00AA,
    BasiliskV3ProWireless = 0x00AB,
    CobraPro = 0x00B0,
    DeathAdderV3 = 0x00B2,
    HyperpollingWirelessDongle = 0x00B3,
    NagaV2HyperspeedReceiver = 0x00B4,
    DeathAdderV3ProWired = 0x00B6,
    DeathAdderV3ProWireless = 0x00B7,
    ViperV3Hyperspeed = 0x00B8,
    //Keyboards
    BlackWidowUltimate2012 = 0x010D,
    BlackWidowStealthEdition = 0x010E,
    Anansi = 0x010F,
    Nostromo = 0x0111,
    Orbweaver = 0x0113,
    DeathstalkerEssential = 0x0118,
    BlackWidowUltimate2013 = 0x011A,
    BlackWidowStealth = 0x011B,
    BlackWidowTE2014 = 0x011C,
    Tartarus = 0x0201,
    DeathstalkerExpert = 0x0202,
    BlackWidowChroma = 0x0203,
    DeathstalkerChroma = 0x0204,
    BladeStealth = 0x0205,
    OrbweaverChroma = 0x0207,
    TartarusChroma = 0x0208,
    BlackWidowChromaTE = 0x0209,
    BladeQHD = 0x020F,
    BladeProLate2016 = 0x0210,
    BlackWidowOverwatch = 0x0211,
    BlackWidowUltimate2016 = 0x0214,
    BlackWidowXChroma = 0x0216,
    BlackWidowXUltimate = 0x0217,
    BlackWidowXChromaTE = 0x021A,
    OrnataChroma = 0x021E,
    Ornata = 0x021F,
    BladeStealthLate2016 = 0x0220,
    BlackWidowChromaV2 = 0x0221,
    BladeLate2016 = 0x0224,
    BladePro2017 = 0x0225,
    HuntsmanElite = 0x0226,
    Huntsman = 0x0227,
    BlackWidowElite = 0x0228,
    CynosaChroma = 0x022A,
    TartarusV2 = 0x022B,
    CynosaChromaPro = 0x022C,
    BladeStealthMid2017 = 0x022D,
    BladePro2017FullHD = 0x022F,
    BladeStealthLate2017 = 0x0232,
    Blade2018 = 0x0233,
    BladePro2019 = 0x0234,
    BlackWidowLite = 0x0235,
    BlackWidowEssential = 0x0237,
    BladeStealth2019 = 0x0239,
    Blade2019Adv = 0x023A,
    Blade2018Base = 0x023B,
    CynosaLite = 0x023F,
    Blade2018Mercury = 0x0240,
    BlackWidow2019 = 0x0241,
    HuntsmanTE = 0x0243,
    BladeMid2019Mercury = 0x0245,
    Blade2019Base = 0x0246,
    BladeStealthLate2019 = 0x024A,
    BladeProLate2019 = 0x024C,
    BladeStudioEdition2019 = 0x024D,
    BlackWidowV3 = 0x024E,
    BladeStealthEarly2020 = 0x0252,
    Blade15Adv2020 = 0x0253,
    BladeEarly2020Base = 0x0255,
    BladeProEarly2020 = 0x0256,
    HuntsmanMini = 0x0257,
    BlackWidowV3Mini = 0x0258,
    BladeStealthLate2020 = 0x0259,
    BlackWidowV3ProWired = 0x025A,
    OrnataV2 = 0x025D,
    CynosaV2 = 0x025E,
    HuntsmanV2Analog = 0x0266,
    HuntsmanMiniJP = 0x0269,
    Book2020 = 0x026A,
    HuntsmanV2Tenkeyless = 0x026B,
    HuntsmanV2 = 0x026C,
    Blade15AdvEarly2021 = 0x026D,
    Blade17ProEarly2021 = 0x026E,
    Blade15BaseEarly2021 = 0x026F,
    Blade14_2021 = 0x0270,
    BlackWidowV3MiniWireless = 0x0271,
    Blade15AdvMid2021 = 0x0276,
    Blade17ProMid2021 = 0x0279,
    Blade15Base2022 = 0x027A,
    HuntsmanMiniAnalog = 0x0282,
    Blade15AdvEarly2022 = 0x028A,
    Blade17_2022 = 0x028B,
    Blade14_2022 = 0x028C,
    BlackWidowV4Pro = 0x028D,
    DeathstalkerV2ProWireless = 0x0290,
    DeathstalkerV2ProWired = 0x0292,
    OrnataV3X = 0x0294,
    DeathstalkerV2 = 0x0295,
    DeathstalkerV2ProTenkeylessWireless = 0x0296,
    DeathstalkerV2ProTenkeylessWired = 0x0298,
    Blade15_2023 = 0x029E,
    Blade16_2023 = 0x029F,
    Blade18_2023 = 0x02A0,
    OrnataV3XAlt = 0x02A2,
    BlackWidowV3Tenkeyless = 0x0A24,
    //Kraken
    KrakenClassic = 0x0501,
    Kraken = 0x0504,
    KrakenClassicAlt = 0x0506,
    KrakenV2 = 0x0510,
    KrakenUltimate = 0x0527,
    //Accessories
    FireflyHyperflux = 0x0068,
    MouseDock = 0x007E,
    Core = 0x0215,
    NommoChroma = 0x0517,
    NommoPro = 0x0518,
    Firefly = 0x0C00,
    GoliathusChroma = 0x0C01,
    GoliathusChromaExtended = 0x0C02,
    FireflyV2 = 0x0C04,
    ChromaMug = 0x0F07,
    ChromaBase = 0x0F08,
    ChromaHdk = 0x0F09,
    LaptopStandChroma = 0x0F0D,
    Raptor27 = 0x0F12,
    KrakenKittyEdition = 0x0F19,
    CoreXChroma = 0x0F1A,
    MouseBungeeV3Chroma = 0x0F1D,
    ChromaAddressableRgbController = 0x0F1F,
    BaseStationV2Chroma = 0x0F20,
    Thunderbolt4DockChroma = 0x0F21,
    ChargingPadChroma = 0x0F26,
    LaptopStandChromaV2 = 0x0F2B,
);
