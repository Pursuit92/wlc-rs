extern crate libc;

mod sys;

use sys::{wlc_handle, wlc_view_bring_to_front, wlc_view_focus, wlc_view_set_state, WLC_BIT_ACTIVATED, Struct_wlc_interface, wlc_init, wlc_run, Enum_wlc_log_type, wlc_log_set_handler};

use libc::{c_char, c_int};

use std::ffi;

use std::env::{args, Args};

#[test]
pub fn example() {
    extern "C" fn view_created(view: wlc_handle) -> u8 {
        println!("created");
        unsafe {
            wlc_view_bring_to_front(view);
            wlc_view_focus(view);
        }
        true as u8
    }
    extern "C" fn view_focus(view: wlc_handle, focus: u8) {
        println!("focused");
        unsafe { wlc_view_set_state(view, WLC_BIT_ACTIVATED, focus) }
    }

    extern "C" fn cb_log(log_type: Enum_wlc_log_type, msg: *const c_char) -> () {
        unsafe {
            let msg_string: String = String::from_utf8_lossy(ffi::CStr::from_ptr(msg).to_bytes()).into_owned();
            println!("{}", msg_string);
        }
    }

    let mut interface = Struct_wlc_interface::default();

    interface.view.created = Some(view_created);
    interface.view.focus = Some(view_focus);

    let mut argv:Vec<ffi::CString> = args().map(|arg| { ffi::CString::new(arg).unwrap() } ).collect();
    let mut args:Vec<*mut c_char> = argv.into_iter().map(|arg| { arg.as_ptr() as *mut c_char } ).collect();

    unsafe {
        wlc_log_set_handler(Some(cb_log));
        if !(wlc_init(&mut interface, args.len() as c_int, args.as_mut_ptr()) == 1) {
            panic!("lolwut")
        }
        wlc_run();
    }
}
