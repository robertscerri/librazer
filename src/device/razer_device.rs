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

    fn interface_index(&self) -> u8;

    fn open(&mut self) -> Result<()> {
        self.usb_device().open()?;

        let iface = self.interface_index();
        self.usb_device().claim_interface(iface)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        //Interfaces will be released by USBDevice.close() code.
        self.usb_device().close()
    }

    fn send_report(&mut self, report: RazerReport) -> Result<()> {
        let data: [u8; RZ_REPORT_LEN] = report.into();
        let index: u16 = self.interface_index().into();

        //TODO: Use more idiomatic constants for wValue.
        self.usb_device().write_control(
            request_type(
                rusb::Direction::Out,
                rusb::RequestType::Class,
                rusb::Recipient::Interface,
            ),
            LIBUSB_REQUEST_SET_CONFIGURATION,
            0x300,
            index,
            &data,
            CONTROL_REPORT_TIMEOUT,
        )?;

        Ok(())
    }

    fn read_report(&mut self) -> Result<RazerReport> {
        let index: u16 = self.interface_index().into();

        //TODO: Use more idiomatic constants for wValue.
        let buf: Vec<u8> = self.usb_device().read_control(
            request_type(
                rusb::Direction::In,
                rusb::RequestType::Class,
                rusb::Recipient::Interface,
            ),
            LIBUSB_REQUEST_CLEAR_FEATURE,
            0x300,
            index,
            RZ_REPORT_LEN,
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
            $name: ident {
                product_id: $product_id:expr
                $(, interface_index: $interface_index:expr)?
                $(,)?
            }
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

                fn interface_index(&self) -> u8 {
                    define_razer_device!(@unwrap_iface $($interface_index)?)
                }
            }
        )*
    };

    (@unwrap_iface $iface:expr) => {$iface};
    (@unwrap_iface) => {0x00};
}

define_razer_device!(
    Irichu2011 { product_id: 0x0013 },
    DeathAdder3_5g { product_id: 0x0016 },
    Abyssus1800 { product_id: 0x0020 },
    Mamba2012Wired { product_id: 0x0024 },
    Mamba2012Wireless { product_id: 0x0025 },
    DeathAdder3_5gBlack { product_id: 0x0029 },
    Naga2012 { product_id: 0x002E },
    Imperator { product_id: 0x002F },
    Ouroboros { product_id: 0x0032 },
    Taipan { product_id: 0x0034 },
    NagaHexRed { product_id: 0x0036 },
    DeathAdder2013 { product_id: 0x0037 },
    DeathAdder1800 { product_id: 0x0038 },
    Orochi2013 { product_id: 0x0039 },
    NagaEpicChroma { product_id: 0x003E },
    NagaEpicChromaDock { product_id: 0x003F },
    Naga2014 { product_id: 0x0040 },
    NagaHex { product_id: 0x0041 },
    Abyssus { product_id: 0x0042 },
    DeathAdderChroma { product_id: 0x0043 },
    MambaWired { product_id: 0x0044 },
    MambaWireless { product_id: 0x0045 },
    MambaTEWired { product_id: 0x0046 },
    OrochiChroma { product_id: 0x0048 },
    DiamondbackChroma { product_id: 0x004C },
    DeathAdder2000 { product_id: 0x004F },
    NagaHexV2 { product_id: 0x0050 },
    NagaChroma { product_id: 0x0053 },
    DeathAdder3500 { product_id: 0x0054 },
    LanceheadWired { product_id: 0x0059 },
    LanceheadWireless { product_id: 0x005A },
    AbyssusV2 { product_id: 0x005B },
    DeathAdderElite { product_id: 0x005C },
    Abyssus2000 { product_id: 0x005E },
    LanceheadTEWired { product_id: 0x0060 },
    AtherisReceiver { product_id: 0x0062 },
    Basilisk { product_id: 0x0064 },
    BasiliskEssential { product_id: 0x0065 },
    NagaTrinity { product_id: 0x0067 },
    AbyssusEliteDvaEdition { product_id: 0x006A },
    AbyssusEssential { product_id: 0x006B },
    MambaElite { product_id: 0x006C },
    DeathAdderEssential { product_id: 0x006E },
    LanceheadWirelessReceiver { product_id: 0x006F },
    LanceheadWirelessWired { product_id: 0x0070 },
    DeathAdderEssentialWhiteEdition { product_id: 0x0071 },
    MambaWirelessReceiver { product_id: 0x0072 },
    MambaWirelessWired { product_id: 0x0073 },
    ProClickReceiver { product_id: 0x0077 },
    Viper { product_id: 0x0078 },
    ViperUltimateWired { product_id: 0x007A },
    ViperUltimateWireless { product_id: 0x007B },
    DeathAdderV2ProWired { product_id: 0x007C },
    DeathAdderV2ProWireless { product_id: 0x007D },
    ProClickWired { product_id: 0x0080 },
    BasiliskXHyperspeed { product_id: 0x0083 },
    DeathAdderV2 { product_id: 0x0084 },
    BasiliskV2 { product_id: 0x0085 },
    BasiliskUltimateWired { product_id: 0x0086 },
    BasiliskUltimateReceiver { product_id: 0x0088 },
    ViperMini { product_id: 0x008A },
    DeathAdderV2Mini { product_id: 0x008C },
    NagaLeftHanded2020 { product_id: 0x008D },
    NagaProWired { product_id: 0x008F },
    NagaProWireless { product_id: 0x0090 },
    Viper8k { product_id: 0x0091 },
    OrochiV2Receiver { product_id: 0x0094 },
    OrochiV2Bluetooth { product_id: 0x0095 },
    NagaX { product_id: 0x0096 },
    DeathAdderEssential2021 { product_id: 0x0098 },
    BasiliskV3 { product_id: 0x0099 },
    ProClickMiniReceiver { product_id: 0x009A },
    DeathAdderV2XHyperspeed { product_id: 0x009C },
    ViperMiniSEWired { product_id: 0x009E },
    ViperMiniSEWireless { product_id: 0x009F },
    DeathAdderV2Lite { product_id: 0x00A1 },
    Cobra { product_id: 0x00A3 },
    ViperV2ProWired { product_id: 0x00A5 },
    ViperV2ProWireless { product_id: 0x00A6 },
    BasiliskV3ProWired { product_id: 0x00AA },
    BasiliskV3ProWireless { product_id: 0x00AB },
    CobraPro { product_id: 0x00B0 },
    DeathAdderV3 { product_id: 0x00B2 },
    HyperpollingWirelessDongle { product_id: 0x00B3 },
    NagaV2HyperspeedReceiver { product_id: 0x00B4 },
    DeathAdderV3ProWired { product_id: 0x00B6 },
    DeathAdderV3ProWireless { product_id: 0x00B7 },
    ViperV3Hyperspeed { product_id: 0x00B8 },
    BlackWidowUltimate2012 { product_id: 0x010D },
    BlackWidowStealthEdition { product_id: 0x010E },
    Anansi { product_id: 0x010F },
    Nostromo { product_id: 0x0111 },
    Orbweaver { product_id: 0x0113 },
    DeathstalkerEssential { product_id: 0x0118 },
    BlackWidowUltimate2013 { product_id: 0x011A },
    BlackWidowStealth { product_id: 0x011B },
    BlackWidowTE2014 { product_id: 0x011C },
    Tartarus { product_id: 0x0201 },
    DeathstalkerExpert { product_id: 0x0202 },
    BlackWidowChroma { product_id: 0x0203 },
    DeathstalkerChroma { product_id: 0x0204 },
    BladeStealth { product_id: 0x0205 },
    OrbweaverChroma { product_id: 0x0207 },
    TartarusChroma { product_id: 0x0208 },
    BlackWidowChromaTE { product_id: 0x0209 },
    BladeQHD { product_id: 0x020F },
    BladeProLate2016 { product_id: 0x0210 },
    BlackWidowOverwatch { product_id: 0x0211 },
    BlackWidowUltimate2016 { product_id: 0x0214 },
    BlackWidowXChroma { product_id: 0x0216 },
    BlackWidowXUltimate { product_id: 0x0217 },
    BlackWidowXChromaTE { product_id: 0x021A },
    OrnataChroma { product_id: 0x021E },
    Ornata { product_id: 0x021F },
    BladeStealthLate2016 { product_id: 0x0220 },
    BlackWidowChromaV2 {
        product_id: 0x0221,
        interface_index: 0x02
    },
    BladeLate2016 { product_id: 0x0224 },
    BladePro2017 { product_id: 0x0225 },
    HuntsmanElite { product_id: 0x0226 },
    Huntsman { product_id: 0x0227 },
    BlackWidowElite { product_id: 0x0228 },
    CynosaChroma { product_id: 0x022A },
    TartarusV2 { product_id: 0x022B },
    CynosaChromaPro { product_id: 0x022C },
    BladeStealthMid2017 { product_id: 0x022D },
    BladePro2017FullHD { product_id: 0x022F },
    BladeStealthLate2017 { product_id: 0x0232 },
    Blade2018 { product_id: 0x0233 },
    BladePro2019 { product_id: 0x0234 },
    BlackWidowLite { product_id: 0x0235 },
    BlackWidowEssential { product_id: 0x0237 },
    BladeStealth2019 { product_id: 0x0239 },
    Blade2019Adv { product_id: 0x023A },
    Blade2018Base { product_id: 0x023B },
    CynosaLite { product_id: 0x023F },
    Blade2018Mercury { product_id: 0x0240 },
    BlackWidow2019 { product_id: 0x0241 },
    HuntsmanTE { product_id: 0x0243 },
    BladeMid2019Mercury { product_id: 0x0245 },
    Blade2019Base { product_id: 0x0246 },
    BladeStealthLate2019 { product_id: 0x024A },
    BladeProLate2019 { product_id: 0x024C },
    BladeStudioEdition2019 { product_id: 0x024D },
    BlackWidowV3 { product_id: 0x024E },
    BladeStealthEarly2020 { product_id: 0x0252 },
    Blade15Adv2020 { product_id: 0x0253 },
    BladeEarly2020Base { product_id: 0x0255 },
    BladeProEarly2020 { product_id: 0x0256 },
    HuntsmanMini { product_id: 0x0257 },
    BlackWidowV3Mini { product_id: 0x0258 },
    BladeStealthLate2020 { product_id: 0x0259 },
    BlackWidowV3ProWired { product_id: 0x025A },
    OrnataV2 { product_id: 0x025D },
    CynosaV2 { product_id: 0x025E },
    HuntsmanV2Analog { product_id: 0x0266 },
    HuntsmanMiniJP { product_id: 0x0269 },
    Book2020 { product_id: 0x026A },
    HuntsmanV2Tenkeyless { product_id: 0x026B },
    HuntsmanV2 { product_id: 0x026C },
    Blade15AdvEarly2021 { product_id: 0x026D },
    Blade17ProEarly2021 { product_id: 0x026E },
    Blade15BaseEarly2021 { product_id: 0x026F },
    Blade14_2021 { product_id: 0x0270 },
    BlackWidowV3MiniWireless { product_id: 0x0271 },
    Blade15AdvMid2021 { product_id: 0x0276 },
    Blade17ProMid2021 { product_id: 0x0279 },
    Blade15Base2022 { product_id: 0x027A },
    HuntsmanMiniAnalog { product_id: 0x0282 },
    Blade15AdvEarly2022 { product_id: 0x028A },
    Blade17_2022 { product_id: 0x028B },
    Blade14_2022 { product_id: 0x028C },
    BlackWidowV4Pro { product_id: 0x028D },
    DeathstalkerV2ProWireless { product_id: 0x0290 },
    DeathstalkerV2ProWired { product_id: 0x0292 },
    OrnataV3X { product_id: 0x0294 },
    DeathstalkerV2 { product_id: 0x0295 },
    DeathstalkerV2ProTenkeylessWireless { product_id: 0x0296 },
    DeathstalkerV2ProTenkeylessWired { product_id: 0x0298 },
    Blade15_2023 { product_id: 0x029E },
    Blade16_2023 { product_id: 0x029F },
    Blade18_2023 { product_id: 0x02A0 },
    OrnataV3XAlt { product_id: 0x02A2 },
    BlackWidowV3Tenkeyless { product_id: 0x0A24 },
    KrakenClassic { product_id: 0x0501 },
    Kraken { product_id: 0x0504 },
    KrakenClassicAlt { product_id: 0x0506 },
    KrakenV2 { product_id: 0x0510 },
    KrakenUltimate { product_id: 0x0527 },
    FireflyHyperflux { product_id: 0x0068 },
    MouseDock { product_id: 0x007E },
    Core { product_id: 0x0215 },
    NommoChroma { product_id: 0x0517 },
    NommoPro { product_id: 0x0518 },
    Firefly { product_id: 0x0C00 },
    GoliathusChroma { product_id: 0x0C01 },
    GoliathusChromaExtended { product_id: 0x0C02 },
    FireflyV2 { product_id: 0x0C04 },
    ChromaMug { product_id: 0x0F07 },
    ChromaBase { product_id: 0x0F08 },
    ChromaHdk { product_id: 0x0F09 },
    LaptopStandChroma { product_id: 0x0F0D },
    Raptor27 { product_id: 0x0F12 },
    KrakenKittyEdition { product_id: 0x0F19 },
    CoreXChroma { product_id: 0x0F1A },
    MouseBungeeV3Chroma { product_id: 0x0F1D },
    ChromaAddressableRgbController { product_id: 0x0F1F },
    BaseStationV2Chroma { product_id: 0x0F20 },
    Thunderbolt4DockChroma { product_id: 0x0F21 },
    ChargingPadChroma { product_id: 0x0F26 },
    LaptopStandChromaV2 { product_id: 0x0F2B }
);
