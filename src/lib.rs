extern crate libc;

mod sys;

use sys::{wlc_handle, wlc_view_bring_to_front, wlc_view_focus, wlc_view_set_state, wlc_view_state_bit, wlc_interface, wlc_view, wlc_init, wlc_run};

use libc::{c_char, c_int};

use std::ffi;

use std::env::{args, Args};

#[test]
pub fn example() {
    extern "C" fn view_created(view: wlc_handle) -> bool {
        unsafe {
            wlc_view_bring_to_front(view);
            wlc_view_focus(view);
        }
        true
    }
    extern "C" fn view_focus(view: wlc_handle, focus: bool) {
        unsafe { wlc_view_set_state(view, wlc_view_state_bit::WLC_BIT_ACTIVATED, focus) }
    }

    let mut interface = wlc_interface::default();

    interface.view.created = Some(view_created);
    interface.view.focus = Some(view_focus);

    let argv:Vec<ffi::CString> = args().map(|arg| { ffi::CString::new(arg).unwrap() } ).collect();
    let args:Vec<*const c_char> = argv.into_iter().map(|arg| { arg.as_ptr() } ).collect();

    unsafe {
        if !wlc_init(&mut interface, args.len() as c_int, args.as_ptr()) {
            panic!("lolwut")
        }
        wlc_run();
    }
}
