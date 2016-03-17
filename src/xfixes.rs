// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{
  c_int,
  c_ulong,
  c_void,
};

use ::xlib::{
  Atom,
  Bool,
  Display,
  Time,
  Window,
  XEvent,
  XID,
};

use ::internal::{
  transmute_union,
};


//
// functions
//


x11_link! { XFixes, ["libXfixes.so", "libXfixes.so.3"],
  pub fn XFixesQueryExtension (dpy: *mut Display, event_base_return: *mut c_int, error_base_return: *mut c_int) -> Bool,
  pub fn XFixesHideCursor (dpy: *mut Display, win: Window) -> c_void,
  pub fn XFixesShowCursor (dpy: *mut Display, win: Window) -> c_void,
  pub fn XFixesSelectSelectionInput (dpy: *mut Display, win: Window, selection: Atom, event_mask: c_ulong) -> c_void,
variadic:
globals:
}


//
// types
//


pub type PointerBarrier = XID;


//
// event structures
//


impl From<XFixesSelectionNotifyEvent> for XEvent {
  fn from(e: XFixesSelectionNotifyEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

#[test]
fn xevent_size_test () {
  assert!(::std::mem::size_of::<XEvent>() >= ::std::mem::size_of::<XFixesSelectionNotifyEvent>());
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFixesSelectionNotifyEvent {
  pub type_: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub subtype: c_int,
  pub owner: Window,
  pub selection: Atom,
  pub timestamp: Time,
  pub selection_timestamp: Time,
}

impl From<XEvent> for XFixesSelectionNotifyEvent {
  fn from(e: XEvent) -> XFixesSelectionNotifyEvent {
    unsafe { transmute_union(&e) }
  }
}


//
// constants
//


pub const XFixesSelectionNotify: c_int = 0;

pub const XFixesSetSelectionOwnerNotify: c_int = 0;
pub const XFixesSelectionWindowDestroyNotify: c_int = 1;
pub const XFixesSelectionClientCloseNotify: c_int = 2;

pub const XFixesSetSelectionOwnerNotifyMask: c_ulong = 1 << 0;
pub const XFixesSelectionWindowDestroyNotifyMask: c_ulong = 1 << 1;
pub const XFixesSelectionClientCloseNotifyMask: c_ulong = 1 << 2;
