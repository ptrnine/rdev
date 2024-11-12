extern crate libc;
extern crate x11;
use crate::linux::common::{convert, KEYBOARD};
use crate::linux::keyboard::Keyboard;
use crate::rdev::{Event, ListenError};
use std::ffi::CStr;
use std::mem;
use std::ptr::null;
use x11::xinput2;
use x11::xlib;

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event)>> = None;

pub fn listen<T>(callback: T) -> Result<(), ListenError>
where
    T: FnMut(Event) + 'static,
{
    let keyboard = Keyboard::new().ok_or(ListenError::KeyboardError)?;

    unsafe {
        KEYBOARD = Some(keyboard);
        GLOBAL_CALLBACK = Some(Box::new(callback));

        let disp = xlib::XOpenDisplay(null());
        if disp.is_null() {
            return Err(ListenError::MissingDisplayError);
        }

        let mut xi_opcode: i32 = 0;
        {
            let mut query_event: i32 = 0;
            let mut query_error: i32 = 0;
            let extension_name = CStr::from_bytes_with_nul(b"XInputExtension\0").unwrap();
            if xlib::XQueryExtension(
                disp,
                extension_name.as_ptr(),
                &mut xi_opcode,
                &mut query_event,
                &mut query_error,
            ) == 0
            {
                return Err(ListenError::XRecordExtensionError);
            }
        }

        // Register to receive XInput events
        let root_wnd = xlib::XDefaultRootWindow(disp);
        let mut mask_data: [u8; 5] = mem::zeroed();
        let mut m = xinput2::XIEventMask {
            deviceid: xinput2::XIAllMasterDevices,
            mask_len: 5,
            mask: mask_data.as_mut_ptr(),
        };
        xinput2::XISetMask(&mut mask_data, xinput2::XI_RawKeyPress);
        xinput2::XISetMask(&mut mask_data, xinput2::XI_RawMotion);
        xinput2::XISelectEvents(disp, root_wnd, &mut m, 1);
        xlib::XSync(disp, 0);

        xlib::XkbSelectEventDetails(disp, 0x0100, 2, 1 << 4, 1 << 4);
        //let mut state: xlib::XkbStateRec = mem::zeroed();
        //xlib::XkbGetState(disp, 0x0100, &mut state);
        //let group = state.group;

        loop {
            let mut event: xlib::XEvent = mem::zeroed();
            xlib::XNextEvent(disp, &mut event);
            let cookie = &mut *{ &mut event.generic_event_cookie };

            if xlib::XGetEventData(disp, cookie) == 1 {
                if cookie.type_ == xlib::GenericEvent && cookie.extension == xi_opcode {
                    if cookie.evtype == xinput2::XI_RawKeyPress {
                        let ev = &mut *{ cookie.data as *mut xinput2::XIRawEvent };
                        if let Some(event) =
                            convert(&mut KEYBOARD, ev.detail as u32, xlib::KeyPress, 0.0, 0.0)
                        {
                            if let Some(callback) = &mut GLOBAL_CALLBACK {
                                callback(event);
                            }
                        }
                    }
                    else if cookie.evtype == xinput2::XI_RawMotion {
                        let mut root_ret: u64 = 0;
                        let mut child_ret: u64 = 0;
                        let mut root_x: i32 = 0;
                        let mut root_y: i32 = 0;
                        let mut win_x: i32 = 0;
                        let mut win_y: i32 = 0;
                        let mut mask_ret: u32 = 0;
                        if xlib::XQueryPointer(
                            disp,
                            root_wnd,
                            &mut root_ret,
                            &mut child_ret,
                            &mut root_x,
                            &mut root_y,
                            &mut win_x,
                            &mut win_y,
                            &mut mask_ret
                        ) == 0
                        {
                            continue;
                        }

                        if let Some(event) =
                            convert(&mut KEYBOARD, 0, xlib::MotionNotify, root_x as f64, root_y as f64)
                        {
                            if let Some(callback) = &mut GLOBAL_CALLBACK {
                                callback(event);
                            }
                        }
                    }
                }
                xlib::XFreeEventData(disp, cookie);
            }
        }
    }
}
