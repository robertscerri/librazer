impl RzDevice {
    // fn get_w_index(&self) -> u16 {
    //     match self.pid {
    //         RZ_PID_BLACKWIDOW_CHROMA_V2 => 0x02,
    //         _ => 0x00,
    //     }
    // }

    pub fn open(&mut self, pid: u16) {
        self.pid = pid;
        self.usb_dev = usbcommon::usb_get_dev_by_pid(pid);
        self.w_index = self.get_w_index();
        self.dev_type = self.get_device_type();

        if self.usb_dev.is_none() {
            return;
        }

        //Trying to claim an interface on macOS gives an access error, however functions work as intended without claiming the interface on macOS.
        //To prevent access errors, the kernel driver must be detached, however this renders the device unusable.
        //This patch is placed here until a better solution is found.
        if cfg!(target_os = "macos") {
            return;
        }

        let res = self
            .usb_dev
            .as_ref()
            .unwrap()
            .claim_interface(self.w_index as u8);

        match res {
            Ok(_) => {}
            Err(e) => println!("Failed to claim interface: {:?}", e),
        }
    }

    pub fn close(&self) {
        if self.usb_dev.is_none() {
            return;
        }

        //Since for macOS targets no interfaces are claimed, none should thus be released.
        if cfg!(target_os = "macos") {
            return;
        }

        let res = self
            .usb_dev
            .as_ref()
            .unwrap()
            .release_interface(self.w_index as u8);

        match res {
            Ok(_) => {}
            Err(e) => println!("Failed to release interface: {:?}", e),
        }
    }

    pub fn send_report(&self, report: &RzReport) -> bool {
        if self.usb_dev.is_none() {
            return false;
        }

        let data: [u8; RZ_REPORT_LEN] = report.into();

        return self
            .usb_dev
            .as_ref()
            .unwrap()
            .write_control(
                0x21,
                0x09,
                0x300,
                self.w_index,
                &data,
                std::time::Duration::from_millis(2000),
            )
            .unwrap()
            == RZ_REPORT_LEN;
    }
}
