use crate::{
    protocol::status::Status,
    utils::errors::{Error, Result},
};

pub const RZ_REPORT_LEN: usize = 90;

#[derive(Debug)]
pub struct ReportHeader {
    status: Status,
    transaction_id: u8,
    remaining_packets: u16,
    protocol_type: u8,
    data_size: u8,
    command_class: u8,
    command_id: u8,
}

#[derive(Debug)]
pub struct RazerReport {
    pub header: ReportHeader,
    pub arguments: Vec<u8>,
}

//TODO: Use more idiomatic constants
impl RazerReport {
    pub fn new(
        status: Status,
        transaction_id: u8,
        remaining_packets: u16,
        command_class: u8,
        command_id: u8,
        arguments: Vec<u8>,
    ) -> Self {
        //Truncate more than 80 elements to comply with RZ_REPORT_LEN
        let arguments = arguments[0..arguments.len().min(80)].to_vec();

        let header = ReportHeader {
            status,
            transaction_id,
            remaining_packets,
            protocol_type: 0x00,
            data_size: arguments.len() as u8,
            command_class,
            command_id,
        };

        RazerReport { header, arguments }
    }
}

impl From<&RazerReport> for [u8; RZ_REPORT_LEN] {
    fn from(report: &RazerReport) -> Self {
        let mut data: [u8; RZ_REPORT_LEN] = [0; RZ_REPORT_LEN];

        data[0] = report.header.status as u8;
        data[1] = report.header.transaction_id;

        //Big Endian conversion
        let remaining_packets_be = u16::to_be_bytes(report.header.remaining_packets);
        data[2] = remaining_packets_be[0];
        data[3] = remaining_packets_be[1];

        data[4] = report.header.protocol_type;
        data[5] = report.header.data_size;
        data[6] = report.header.command_class;
        data[7] = report.header.command_id;

        //TODO: Any way to make this cleaner?
        let arguments_dest = &mut data[8..(8 + report.arguments.len())];
        arguments_dest.copy_from_slice(&report.arguments);

        data[RZ_REPORT_LEN - 2] = compute_crc(&data);

        data
    }
}

impl From<RazerReport> for [u8; RZ_REPORT_LEN] {
    fn from(report: RazerReport) -> Self {
        (&report).into()
    }
}

impl TryFrom<[u8; RZ_REPORT_LEN]> for RazerReport {
    type Error = Error;

    fn try_from(data: [u8; RZ_REPORT_LEN]) -> Result<Self> {
        let args_end = 8 + data[5] as usize;
        let arguments = data[8..args_end].to_vec();

        //TODO: Protocol type
        Ok(RazerReport::new(
            data[0].try_into()?,
            data[1],
            u16::from_be_bytes([data[2], data[3]]),
            data[6],
            data[7],
            arguments,
        ))
    }
}

fn compute_crc(data: &[u8; RZ_REPORT_LEN]) -> u8 {
    let mut crc: u8 = 0;

    for byte in data.iter().skip(2).take(RZ_REPORT_LEN - 4) {
        crc ^= byte;
    }

    crc
}
