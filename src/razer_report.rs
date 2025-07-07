pub const RZ_REPORT_LEN: usize = 90;

#[derive(Debug)]
struct ReportHeader {
    status: u8,
    transaction_id: u8,
    remaining_packets: u8,
    data_size: u8,
    command_class: u8,
    command_id: u8,
}

#[derive(Debug)]
pub struct RazerReport {
    header: ReportHeader,
    arguments: [u8; 80],
}

impl RazerReport {
    fn new(
        status: u8,
        transaction_id: u8,
        remaining_packets: u8,
        data_size: u8,
        command_class: u8,
        command_id: u8,
        arguments: [u8; 80],
    ) -> Self {
        let header = ReportHeader {
            status,
            transaction_id,
            remaining_packets,
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

        data[0] = report.header.status;
        data[1] = report.header.transaction_id;
        data[2] = report.header.remaining_packets;
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

impl From<[u8; RZ_REPORT_LEN]> for RazerReport {
    fn from(data: [u8; RZ_REPORT_LEN]) -> Self {
        let mut arguments: [u8; 80] = [0; 80];
        arguments.copy_from_slice(&data[8..(RZ_REPORT_LEN - 2)]);

        RazerReport::new(
            data[0], data[1], data[2], data[5], data[6], data[7], arguments,
        )
    }
}

fn compute_crc(data: &[u8; RZ_REPORT_LEN]) -> u8 {
    let mut crc: u8 = 0;

    for byte in data.iter().skip(2).take(RZ_REPORT_LEN - 4) {
        crc ^= byte;
    }

    crc
}
