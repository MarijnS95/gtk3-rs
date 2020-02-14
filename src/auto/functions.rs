// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use gdk_pixbuf;
use gdk_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use libc;
use pango;
use std::mem;
use std::ptr;
use Atom;
use Display;
use Event;
use EventType;
use ModifierType;
use Screen;
use Visual;
use Window;
use WindowState;

//#[cfg_attr(feature = "v3_16", deprecated)]
//pub fn add_option_entries_libgtk_only(group: /*Ignored*/&glib::OptionGroup) {
//    unsafe { TODO: call gdk_sys:gdk_add_option_entries_libgtk_only() }
//}

pub fn beep() {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_beep();
    }
}

pub fn error_trap_pop() -> i32 {
    assert_initialized_main_thread!();
    unsafe { gdk_sys::gdk_error_trap_pop() }
}

pub fn error_trap_pop_ignored() {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_error_trap_pop_ignored();
    }
}

pub fn error_trap_push() {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_error_trap_push();
    }
}

pub fn events_get_angle(event1: &mut Event, event2: &mut Event) -> Option<f64> {
    assert_initialized_main_thread!();
    unsafe {
        let mut angle = mem::MaybeUninit::uninit();
        let ret = from_glib(gdk_sys::gdk_events_get_angle(
            event1.to_glib_none_mut().0,
            event2.to_glib_none_mut().0,
            angle.as_mut_ptr(),
        ));
        let angle = angle.assume_init();
        if ret {
            Some(angle)
        } else {
            None
        }
    }
}

pub fn events_get_center(event1: &mut Event, event2: &mut Event) -> Option<(f64, f64)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut x = mem::MaybeUninit::uninit();
        let mut y = mem::MaybeUninit::uninit();
        let ret = from_glib(gdk_sys::gdk_events_get_center(
            event1.to_glib_none_mut().0,
            event2.to_glib_none_mut().0,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
        ));
        let x = x.assume_init();
        let y = y.assume_init();
        if ret {
            Some((x, y))
        } else {
            None
        }
    }
}

pub fn events_get_distance(event1: &mut Event, event2: &mut Event) -> Option<f64> {
    assert_initialized_main_thread!();
    unsafe {
        let mut distance = mem::MaybeUninit::uninit();
        let ret = from_glib(gdk_sys::gdk_events_get_distance(
            event1.to_glib_none_mut().0,
            event2.to_glib_none_mut().0,
            distance.as_mut_ptr(),
        ));
        let distance = distance.assume_init();
        if ret {
            Some(distance)
        } else {
            None
        }
    }
}

pub fn events_pending() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(gdk_sys::gdk_events_pending()) }
}

pub fn flush() {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_flush();
    }
}

pub fn get_display_arg_name() -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(gdk_sys::gdk_get_display_arg_name()) }
}

pub fn get_program_class() -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(gdk_sys::gdk_get_program_class()) }
}

pub fn get_show_events() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(gdk_sys::gdk_get_show_events()) }
}

//pub fn init_check(argv: /*Unimplemented*/Vec<GString>) -> bool {
//    unsafe { TODO: call gdk_sys:gdk_init_check() }
//}

pub fn keyval_convert_case(symbol: u32) -> (u32, u32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut lower = mem::MaybeUninit::uninit();
        let mut upper = mem::MaybeUninit::uninit();
        gdk_sys::gdk_keyval_convert_case(symbol, lower.as_mut_ptr(), upper.as_mut_ptr());
        let lower = lower.assume_init();
        let upper = upper.assume_init();
        (lower, upper)
    }
}

pub fn keyval_from_name(keyval_name: &str) -> u32 {
    assert_initialized_main_thread!();
    unsafe { gdk_sys::gdk_keyval_from_name(keyval_name.to_glib_none().0) }
}

pub fn keyval_is_lower(keyval: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(gdk_sys::gdk_keyval_is_lower(keyval)) }
}

pub fn keyval_is_upper(keyval: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(gdk_sys::gdk_keyval_is_upper(keyval)) }
}

pub fn keyval_to_lower(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { gdk_sys::gdk_keyval_to_lower(keyval) }
}

pub fn keyval_to_upper(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { gdk_sys::gdk_keyval_to_upper(keyval) }
}

#[cfg_attr(feature = "v3_22", deprecated)]
pub fn list_visuals() -> Vec<Visual> {
    assert_initialized_main_thread!();
    unsafe { FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_list_visuals()) }
}

pub fn notify_startup_complete() {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_notify_startup_complete();
    }
}

pub fn notify_startup_complete_with_id(startup_id: &str) {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_notify_startup_complete_with_id(startup_id.to_glib_none().0);
    }
}

pub fn pango_context_get() -> Option<pango::Context> {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(gdk_sys::gdk_pango_context_get()) }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
pub fn pango_context_get_for_display(display: &Display) -> Option<pango::Context> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(gdk_sys::gdk_pango_context_get_for_display(
            display.to_glib_none().0,
        ))
    }
}

pub fn pango_context_get_for_screen(screen: &Screen) -> Option<pango::Context> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(gdk_sys::gdk_pango_context_get_for_screen(
            screen.to_glib_none().0,
        ))
    }
}

//pub fn pango_layout_line_get_clip_region(line: &pango::LayoutLine, x_origin: i32, y_origin: i32, index_ranges: &[i32], n_ranges: i32) -> Option<cairo::Region> {
//    unsafe { TODO: call gdk_sys:gdk_pango_layout_line_get_clip_region() }
//}

//pub fn parse_args(argv: /*Unimplemented*/Vec<GString>) {
//    unsafe { TODO: call gdk_sys:gdk_parse_args() }
//}

pub fn pixbuf_get_from_surface(
    surface: &cairo::Surface,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
) -> Option<gdk_pixbuf::Pixbuf> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gdk_sys::gdk_pixbuf_get_from_surface(
            mut_override(surface.to_glib_none().0),
            src_x,
            src_y,
            width,
            height,
        ))
    }
}

#[cfg_attr(feature = "v3_16", deprecated)]
pub fn pre_parse_libgtk_only() {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_pre_parse_libgtk_only();
    }
}

pub fn property_delete<P: IsA<Window>>(window: &P, property: &Atom) {
    skip_assert_initialized!();
    unsafe {
        gdk_sys::gdk_property_delete(window.as_ref().to_glib_none().0, property.to_glib_none().0);
    }
}

pub fn property_get<P: IsA<Window>>(
    window: &P,
    property: &Atom,
    type_: &Atom,
    offset: libc::c_ulong,
    length: libc::c_ulong,
    pdelete: i32,
) -> Option<(Atom, i32, Vec<u8>)> {
    skip_assert_initialized!();
    unsafe {
        let mut actual_property_type = Atom::uninitialized();
        let mut actual_format = mem::MaybeUninit::uninit();
        let mut actual_length = mem::MaybeUninit::uninit();
        let mut data = ptr::null_mut();
        let ret = from_glib(gdk_sys::gdk_property_get(
            window.as_ref().to_glib_none().0,
            property.to_glib_none().0,
            type_.to_glib_none().0,
            offset,
            length,
            pdelete,
            actual_property_type.to_glib_none_mut().0,
            actual_format.as_mut_ptr(),
            actual_length.as_mut_ptr(),
            &mut data,
        ));
        let actual_format = actual_format.assume_init();
        if ret {
            Some((
                actual_property_type,
                actual_format,
                FromGlibContainer::from_glib_full_num(data, actual_length.assume_init() as usize),
            ))
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "v3_22", deprecated)]
pub fn query_depths() -> Vec<i32> {
    assert_initialized_main_thread!();
    unsafe {
        let mut depths = ptr::null_mut();
        let mut count = mem::MaybeUninit::uninit();
        gdk_sys::gdk_query_depths(&mut depths, count.as_mut_ptr());
        FromGlibContainer::from_glib_none_num(depths, count.assume_init() as usize)
    }
}

//#[cfg_attr(feature = "v3_22", deprecated)]
//pub fn query_visual_types(visual_types: /*Unimplemented*/CArray TypeId { ns_id: 1, id: 99 }) -> i32 {
//    unsafe { TODO: call gdk_sys:gdk_query_visual_types() }
//}

pub fn selection_convert<P: IsA<Window>>(
    requestor: &P,
    selection: &Atom,
    target: &Atom,
    time_: u32,
) {
    skip_assert_initialized!();
    unsafe {
        gdk_sys::gdk_selection_convert(
            requestor.as_ref().to_glib_none().0,
            selection.to_glib_none().0,
            target.to_glib_none().0,
            time_,
        );
    }
}

pub fn selection_owner_get(selection: &Atom) -> Option<Window> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(gdk_sys::gdk_selection_owner_get(selection.to_glib_none().0)) }
}

pub fn selection_owner_get_for_display(display: &Display, selection: &Atom) -> Option<Window> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(gdk_sys::gdk_selection_owner_get_for_display(
            display.to_glib_none().0,
            selection.to_glib_none().0,
        ))
    }
}

pub fn selection_owner_set<P: IsA<Window>>(
    owner: Option<&P>,
    selection: &Atom,
    time_: u32,
    send_event: bool,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gdk_sys::gdk_selection_owner_set(
            owner.map(|p| p.as_ref()).to_glib_none().0,
            selection.to_glib_none().0,
            time_,
            send_event.to_glib(),
        ))
    }
}

pub fn selection_owner_set_for_display<P: IsA<Window>>(
    display: &Display,
    owner: Option<&P>,
    selection: &Atom,
    time_: u32,
    send_event: bool,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(gdk_sys::gdk_selection_owner_set_for_display(
            display.to_glib_none().0,
            owner.map(|p| p.as_ref()).to_glib_none().0,
            selection.to_glib_none().0,
            time_,
            send_event.to_glib(),
        ))
    }
}

pub fn selection_send_notify<P: IsA<Window>>(
    requestor: &P,
    selection: &Atom,
    target: &Atom,
    property: &Atom,
    time_: u32,
) {
    skip_assert_initialized!();
    unsafe {
        gdk_sys::gdk_selection_send_notify(
            requestor.as_ref().to_glib_none().0,
            selection.to_glib_none().0,
            target.to_glib_none().0,
            property.to_glib_none().0,
            time_,
        );
    }
}

pub fn selection_send_notify_for_display<P: IsA<Window>>(
    display: &Display,
    requestor: &P,
    selection: &Atom,
    target: &Atom,
    property: &Atom,
    time_: u32,
) {
    skip_assert_initialized!();
    unsafe {
        gdk_sys::gdk_selection_send_notify_for_display(
            display.to_glib_none().0,
            requestor.as_ref().to_glib_none().0,
            selection.to_glib_none().0,
            target.to_glib_none().0,
            property.to_glib_none().0,
            time_,
        );
    }
}

pub fn set_allowed_backends(backends: &str) {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_set_allowed_backends(backends.to_glib_none().0);
    }
}

pub fn set_double_click_time(msec: u32) {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_set_double_click_time(msec);
    }
}

pub fn set_program_class(program_class: &str) {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_set_program_class(program_class.to_glib_none().0);
    }
}

pub fn set_show_events(show_events: bool) {
    assert_initialized_main_thread!();
    unsafe {
        gdk_sys::gdk_set_show_events(show_events.to_glib());
    }
}

pub fn synthesize_window_state<P: IsA<Window>>(
    window: &P,
    unset_flags: WindowState,
    set_flags: WindowState,
) {
    skip_assert_initialized!();
    unsafe {
        gdk_sys::gdk_synthesize_window_state(
            window.as_ref().to_glib_none().0,
            unset_flags.to_glib(),
            set_flags.to_glib(),
        );
    }
}

pub fn test_render_sync<P: IsA<Window>>(window: &P) {
    skip_assert_initialized!();
    unsafe {
        gdk_sys::gdk_test_render_sync(window.as_ref().to_glib_none().0);
    }
}

pub fn test_simulate_button<P: IsA<Window>>(
    window: &P,
    x: i32,
    y: i32,
    button: u32,
    modifiers: ModifierType,
    button_pressrelease: EventType,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(gdk_sys::gdk_test_simulate_button(
            window.as_ref().to_glib_none().0,
            x,
            y,
            button,
            modifiers.to_glib(),
            button_pressrelease.to_glib(),
        ))
    }
}

pub fn test_simulate_key<P: IsA<Window>>(
    window: &P,
    x: i32,
    y: i32,
    keyval: u32,
    modifiers: ModifierType,
    key_pressrelease: EventType,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(gdk_sys::gdk_test_simulate_key(
            window.as_ref().to_glib_none().0,
            x,
            y,
            keyval,
            modifiers.to_glib(),
            key_pressrelease.to_glib(),
        ))
    }
}

pub fn text_property_to_utf8_list_for_display(
    display: &Display,
    encoding: &Atom,
    format: i32,
    text: &[u8],
) -> (i32, Vec<GString>) {
    skip_assert_initialized!();
    let length = text.len() as i32;
    unsafe {
        let mut list = ptr::null_mut();
        let ret = gdk_sys::gdk_text_property_to_utf8_list_for_display(
            display.to_glib_none().0,
            encoding.to_glib_none().0,
            format,
            text.to_glib_none().0,
            length,
            &mut list,
        );
        (ret, FromGlibPtrContainer::from_glib_full(list))
    }
}

pub fn unicode_to_keyval(wc: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { gdk_sys::gdk_unicode_to_keyval(wc) }
}

pub fn utf8_to_string_target(str: &str) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(gdk_sys::gdk_utf8_to_string_target(str.to_glib_none().0)) }
}
