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
    pub arguments: [u8; 80],
}

//TODO: Use more idiomatic constants
impl RazerReport {
    pub fn new(
        status: Status,
        transaction_id: u8,
        remaining_packets: u16,
        data_size: u8,
        command_class: u8,
        command_id: u8,
        arguments: [u8; 80],
    ) -> Self {
        let header = ReportHeader {
            status,
            transaction_id,
            remaining_packets,
            protocol_type: 0x00,
            data_size,
            command_class,
            command_id,
        };

        RazerReport { header, arguments }
    }
}

impl From<&RazerReport> for [u8; RZ_REPORT_LEN] {
    fn from(report: &RazerReport) -> Self {
        let mut data: [u8; RZ_REPORT_LEN] = [0; RZ_REPORT_LEN];

        data[0] = report.header.status.as_u8();
        data[1] = report.header.transaction_id;

        //Big Endian conversion
        data[2] = ((report.header.remaining_packets >> 8) & 0xff) as u8;
        data[3] = (report.header.remaining_packets & 0xff) as u8;

        data[4] = report.header.protocol_type;
        data[5] = report.header.data_size;
        data[6] = report.header.command_class;
        data[7] = report.header.command_id;

        data[8..(8 + report.arguments.len())].copy_from_slice(&report.arguments);

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
        let mut arguments: [u8; 80] = [0; 80];
        arguments.copy_from_slice(&data[8..(RZ_REPORT_LEN - 2)]);

        //TODO: Protocol type
        Ok(RazerReport::new(
            data[0].try_into()?,
            data[1],
            u16::from_be_bytes([data[2], data[3]]),
            data[5],
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
