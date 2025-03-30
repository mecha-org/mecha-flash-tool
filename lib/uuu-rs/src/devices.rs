include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use tabled::{Table, Tabled};
#[derive(Clone, Tabled)]
pub struct UsbDevice {
    #[tabled(rename = "Path")]
    path: String,
    #[tabled(rename = "Chip")]
    chip: String,
    #[tabled(rename = "Protocol")]
    protocol: String,
    vendor_id: String,
    product_id: String,
    bcd: String,
    serial_no: String,
}

pub struct UsbDevices {
    devices: Vec<UsbDevice>,
}

impl UsbDevices {
    pub fn new() -> Self {
        let mut devices = Vec::new();

        unsafe {
            let mut temp_devices = Vec::new();
            let temp_devices_ptr = &mut temp_devices as *mut Vec<UsbDevice>;
            uuu_for_each_devices(
                Some(process_usb_device),
                temp_devices_ptr as *mut ::std::os::raw::c_void,
            );
            devices = temp_devices;
        }

        UsbDevices { devices }
    }

    pub fn iter(&self) -> impl Iterator<Item = &UsbDevice> {
        self.devices.iter()
    }
}

impl IntoIterator for UsbDevices {
    type Item = UsbDevice;
    type IntoIter = std::vec::IntoIter<UsbDevice>;

    fn into_iter(self) -> Self::IntoIter {
        self.devices.into_iter()
    }
}

extern "C" fn process_usb_device(
    path: *const ::std::os::raw::c_char,
    chip: *const ::std::os::raw::c_char,
    pro: *const ::std::os::raw::c_char,
    vid: u16,
    pid: u16,
    bcd: u16,
    serial_no: *const ::std::os::raw::c_char,
    _p: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let path_str = unsafe { std::ffi::CStr::from_ptr(path) }.to_str().unwrap();
    let chip_str = unsafe { std::ffi::CStr::from_ptr(chip) }.to_str().unwrap();
    let pro_str = unsafe { std::ffi::CStr::from_ptr(pro) }.to_str().unwrap();
    let serial_str = unsafe { std::ffi::CStr::from_ptr(serial_no) }
        .to_str()
        .unwrap();

    let device = UsbDevice {
        path: path_str.to_string(),
        chip: chip_str.to_string(),
        protocol: pro_str.to_string().trim_end_matches(":").to_string(),
        vendor_id: format!("0x{:04X}", vid),
        product_id: format!("0x{:04X}", pid),
        bcd: format!("0x{:04X}", bcd),
        serial_no: serial_str.to_string(),
    };

    unsafe {
        let devices = &mut *(_p as *mut Vec<UsbDevice>);
        devices.push(device);
    }

    0
}

/// Pretty print the list of compatible USB devices
pub fn print_devices() {
    let usb_devices = UsbDevices::new();
    let num_devices = usb_devices.iter().count();
    if num_devices == 0 {
        println!("{}", "No compatible USB devices found.");
        return;
    }

    let mut table = Table::new(usb_devices);
    table
        .with(tabled::settings::Style::rounded())
        .with(tabled::settings::Extract::columns(0..3));
    println!("Found {} compatible USB device(s):", num_devices);
    println!("{}\n", table);
}

/// Get the list of connected and compatible USB devices
pub fn get_devices() -> Vec<UsbDevice> {
    let usb_devices = UsbDevices::new();
    usb_devices.into_iter().collect()
}
