#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// wlc commit 28157d62f14d49c782a03c1b3509307a8444652d

use libc::{int32_t, uint32_t, uint8_t, c_char, c_double, c_int, uintptr_t};

// From wlc/defines.h

pub type wlc_handle = uintptr_t;

// From geometry.h

/** Fixed 2D point */
#[repr(C)]
#[derive(Default)]
pub struct wlc_origin {
    pub x: int32_t,
    pub y: int32_t,
}

/** Fixed 2D size */
#[repr(C)]
#[derive(Default)]
pub struct wlc_size {
    pub w: int32_t,
    pub h: int32_t,
}

/** Fixed 2D point, size pair */
#[repr(C)]
#[derive(Default)]
pub struct wlc_geometry {
    pub origin: wlc_origin,
    pub size: wlc_size,
}

// from wlc.h

pub struct wlc_event_source;

pub struct xkb_state;
pub struct xkb_keymap;
pub struct libinput_device;

/** wlc_log(), wlc_vlog(); */
#[repr(C)]
pub enum wlc_log_type {
   WLC_LOG_INFO,
   WLC_LOG_WARN,
   WLC_LOG_ERROR,
   WLC_LOG_WAYLAND,
}

/** wlc_get_backend_type(); */
#[repr(C)]
pub enum wlc_backend_type {
   WLC_BACKEND_NONE,
   WLC_BACKEND_DRM,
   WLC_BACKEND_X11,
}

/** mask in wlc_event_loop_add_fd(); */
#[repr(C)]
pub enum wlc_event_bit {
   WLC_EVENT_READABLE = 0x01,
   WLC_EVENT_WRITABLE = 0x02,
   WLC_EVENT_HANGUP = 0x04,
   WLC_EVENT_ERROR = 0x08,
}

/** wlc_output_get_connector_type(); */
#[repr(C)]
pub enum wlc_connector_type {
   /* used when running wlc with backend (e.g. x11) that does not use real output. */
   WLC_CONNECTOR_WLC,

   /* these are based on xf86drm.h */
   WLC_CONNECTOR_UNKNOWN,
   WLC_CONNECTOR_VGA,
   WLC_CONNECTOR_DVII,
   WLC_CONNECTOR_DVID,
   WLC_CONNECTOR_DVIA,
   WLC_CONNECTOR_COMPOSITE,
   WLC_CONNECTOR_SVIDEO,
   WLC_CONNECTOR_LVDS,
   WLC_CONNECTOR_COMPONENT,
   WLC_CONNECTOR_DIN,
   WLC_CONNECTOR_DP,
   WLC_CONNECTOR_HDMIA,
   WLC_CONNECTOR_HDMIB,
   WLC_CONNECTOR_TV,
   WLC_CONNECTOR_eDP,
   WLC_CONNECTOR_VIRTUAL,
   WLC_CONNECTOR_DSI,
}

/** wlc_view_get_state(); */
#[repr(C)]
pub enum wlc_view_state_bit {
   WLC_BIT_MAXIMIZED = 1<<0,
   WLC_BIT_FULLSCREEN = 1<<1,
   WLC_BIT_RESIZING = 1<<2,
   WLC_BIT_MOVING = 1<<3,
   WLC_BIT_ACTIVATED = 1<<4,
}

/** wlc_view_get_type(); */
#[repr(C)]
pub enum wlc_view_type_bit {
   WLC_BIT_OVERRIDE_REDIRECT = 1<<0, // Override redirect (x11)
   WLC_BIT_UNMANAGED = 1<<1, // Tooltips, DnD's, menus (x11)
   WLC_BIT_SPLASH = 1<<2, // Splash screens (x11)
   WLC_BIT_MODAL = 1<<3, // Modal windows (x11)
   WLC_BIT_POPUP = 1<<4, // xdg-shell, wl-shell popups
}

/** wlc_view_set_geometry(); Edges in interface interface.view.request.resize function. */
#[repr(C)]
pub enum wlc_resize_edge {
   WLC_RESIZE_EDGE_NONE = 0,
   WLC_RESIZE_EDGE_TOP = 1,
   WLC_RESIZE_EDGE_BOTTOM = 2,
   WLC_RESIZE_EDGE_LEFT = 4,
   WLC_RESIZE_EDGE_TOP_LEFT = 5,
   WLC_RESIZE_EDGE_BOTTOM_LEFT = 6,
   WLC_RESIZE_EDGE_RIGHT = 8,
   WLC_RESIZE_EDGE_TOP_RIGHT = 9,
   WLC_RESIZE_EDGE_BOTTOM_RIGHT = 10,
}

/** Mods in interface.keyboard.key function. */
#[repr(C)]
pub enum wlc_modifier_bit {
   WLC_BIT_MOD_SHIFT = 1<<0,
   WLC_BIT_MOD_CAPS = 1<<1,
   WLC_BIT_MOD_CTRL = 1<<2,
   WLC_BIT_MOD_ALT = 1<<3,
   WLC_BIT_MOD_MOD2 = 1<<4,
   WLC_BIT_MOD_MOD3 = 1<<5,
   WLC_BIT_MOD_LOGO = 1<<6,
   WLC_BIT_MOD_MOD5 = 1<<7,
}

/** Leds in interface.keyboard.key function. */
#[repr(C)]
pub enum wlc_led_bit {
   WLC_BIT_LED_NUM = 1<<0,
   WLC_BIT_LED_CAPS = 1<<1,
   WLC_BIT_LED_SCROLL = 1<<2,
}

/** State in interface.keyboard.key function. */
#[repr(C)]
pub enum wlc_key_state {
   WLC_KEY_STATE_RELEASED = 0,
   WLC_KEY_STATE_PRESSED = 1,
}

/** State in interface.pointer.button function. */
#[repr(C)]
pub enum wlc_button_state {
   WLC_BUTTON_STATE_RELEASED = 0,
   WLC_BUTTON_STATE_PRESSED = 1,
}

/** Axis in interface.pointer.scroll function. */
#[repr(C)]
pub enum wlc_scroll_axis_bit {
   WLC_SCROLL_AXIS_VERTICAL = 1<<0,
   WLC_SCROLL_AXIS_HORIZONTAL = 1<<1,
}

/** Type in interface.touch.touch function */
#[repr(C)]
pub enum wlc_touch_type {
   WLC_TOUCH_DOWN,
   WLC_TOUCH_UP,
   WLC_TOUCH_MOTION,
   WLC_TOUCH_FRAME,
   WLC_TOUCH_CANCEL,
}

/** State of keyboard modifiers in various functions. */
#[repr(C)]
#[derive(Default)]
pub struct wlc_modifiers {
    pub leds: uint32_t,
    pub mods: uint32_t,
}

#[repr(C)]
#[derive(Default)]
pub struct wlc_output {
    /** Output was created. Return false if you want to destroy the output. (e.g. failed to allocate data related to view) */
    pub created: Option<extern "C" fn(wlc_handle) -> bool>,

    /** Output was destroyed. */
    pub destroyed: Option<extern "C" fn(wlc_handle)>,

    /** Output got or lost focus. */
    pub focus: Option<extern "C" fn(wlc_handle, bool)>,

    /** Output resolution changed. */
    pub resolution: Option<extern "C" fn(wlc_handle, *const wlc_size, *const wlc_size)>,
}

#[repr(C)]
#[derive(Default)]
pub struct wlc_request {
    /** Request to set given geometry for view. Apply using wlc_view_set_geometry to agree. */
    pub geometry: Option<extern "C" fn(wlc_handle, *const wlc_geometry)>,

    /** Request to disable or enable the given state for view. Apply using wlc_view_set_state to agree. */
    pub state: Option<extern "C" fn(wlc_handle, wlc_view_state_bit, bool)>,

    /** Request to move itself. Start a interactive move to agree. */
    pub move_req: Option<extern "C" fn(wlc_handle, *const wlc_origin)>,

    /** Request to resize itself with the given edges. Start a interactive resize to agree. */
    pub resize: Option<extern "C" fn(wlc_handle, uint32_t, *const wlc_origin)>,
}

#[repr(C)]
#[derive(Default)]
pub struct wlc_view {
    /** View was created. Return false if you want to destroy the view. (e.g. failed to allocate data related to view) */
    pub created: Option<extern "C" fn(wlc_handle) -> bool>,

    /** View was destroyed. */
    pub destroyed: Option<extern "C" fn(wlc_handle)>,

    /** View got or lost focus. */
    pub focus: Option<extern "C" fn(wlc_handle, bool)>,

    /** View was moved to output. */
    pub move_to_output: Option<extern "C" fn(wlc_handle, wlc_handle, wlc_handle)>,

    pub request: wlc_request,

}

#[repr(C)]
#[derive(Default)]
pub struct wlc_keyboard {
    /** Key event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
    pub key: Option<extern "C" fn(wlc_handle, uint32_t, *const wlc_modifiers, uint32_t, wlc_key_state) -> bool>,
}

#[repr(C)]
#[derive(Default)]
pub struct wlc_pointer {
    /** Button event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
    pub button: Option<extern "C" fn(wlc_handle, uint32_t, *const wlc_modifiers, uint32_t, wlc_button_state, *const wlc_origin) -> bool>,

    /** Scroll event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
    pub scroll: Option<extern "C" fn(wlc_handle, uint32_t, *const wlc_modifiers, uint8_t, [c_double; 2]) -> bool>,

    /** Motion event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
    pub motion: Option<extern "C" fn(wlc_handle, uint32_t, *const wlc_origin) -> bool>,
}

#[repr(C)]
#[derive(Default)]
pub struct wlc_touch {
    /** Touch event was triggered, view handle will be zero if there was no focus. Return true to prevent sending the event to clients. */
    pub touch: Option<extern "C" fn(wlc_handle, uint32_t, *const wlc_modifiers, wlc_touch_type, int32_t, *const wlc_origin) -> bool>,
}

#[repr(C)]
#[derive(Default)]
pub struct wlc_compositor {
    /** Compositor is ready to accept clients. */
    pub ready: Option<extern "C" fn()>,
}

/** Interface struct for communicating with wlc. */
#[repr(C)]
#[derive(Default)]
pub struct wlc_interface {
    pub output: wlc_output,
    pub view: wlc_view,
    pub keyboard: wlc_keyboard,
    pub pointer: wlc_pointer,
    pub touch: wlc_touch,
    pub compositor: wlc_compositor,
}

// Core API
#[link(name = "wlc")]
extern "C" {
    pub fn wlc_view_bring_to_front(view: wlc_handle);
    pub fn wlc_view_focus(view: wlc_handle);
    pub fn wlc_view_set_state(view: wlc_handle, state: wlc_view_state_bit, toggle: bool);
    pub fn wlc_init(interface: *const wlc_interface, argc: c_int, argv: *const *const c_char) -> bool;
    pub fn wlc_run();
}
