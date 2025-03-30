use std::io::Write;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct NotificationHandler {
    pub total: usize,
    pub current: usize,
    pub notification_type: u32,
    pub last_notification_type: u32,
}

impl NotificationHandler {
    pub fn new() -> Self {
        NotificationHandler {
            total: 0,
            current: 0,
            notification_type: 0,
            last_notification_type: 0,
        }
    }

    pub fn print(&self) {
        if self.notification_type == uuu_notify_NOTIFY_TYPE_NOTIFY_TRANS_POS {
            let progress = self.current * 100 / self.total;
            if progress < 100 {
                print!("\rProgress: {}%", progress);
                std::io::stdout().flush().unwrap();
            } else {
                println!("\rProgress: {}%", progress);
            }
        }
    }
}

extern "C" fn notification_callback(
    nt: uuu_notify,
    p: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe {
        let nt_handler = &mut *(p as *mut NotificationHandler);
        nt_handler.last_notification_type = nt_handler.notification_type;
        nt_handler.notification_type = nt.type_;
        match nt.type_ {
            uuu_notify_NOTIFY_TYPE_NOTIFY_TRANS_SIZE => {
                nt_handler.total = nt.__bindgen_anon_1.total;
            }
            uuu_notify_NOTIFY_TYPE_NOTIFY_TRANS_POS => {
                nt_handler.current = nt.__bindgen_anon_1.index;
            }
            _ => {}
        }

        nt_handler.print();
    }

    0
}

pub fn register_notification_callback(nt_handler: &mut NotificationHandler) {
    unsafe {
        uuu_register_notify_callback(
            Some(notification_callback),
            nt_handler as *mut _ as *mut ::std::os::raw::c_void,
        );
    }
}
