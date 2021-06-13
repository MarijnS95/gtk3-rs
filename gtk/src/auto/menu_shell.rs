// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::Container;
use crate::DirectionType;
use crate::MenuDirectionType;
use crate::MenuItem;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkMenuShell")]
    pub struct MenuShell(Object<ffi::GtkMenuShell, ffi::GtkMenuShellClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_menu_shell_get_type(),
    }
}

pub const NONE_MENU_SHELL: Option<&MenuShell> = None;

pub trait MenuShellExt: 'static {
    #[doc(alias = "gtk_menu_shell_activate_item")]
    fn activate_item<P: IsA<Widget>>(&self, menu_item: &P, force_deactivate: bool);

    #[doc(alias = "gtk_menu_shell_append")]
    fn append<P: IsA<MenuItem>>(&self, child: &P);

    #[doc(alias = "gtk_menu_shell_bind_model")]
    fn bind_model<P: IsA<gio::MenuModel>>(
        &self,
        model: Option<&P>,
        action_namespace: Option<&str>,
        with_separators: bool,
    );

    #[doc(alias = "gtk_menu_shell_cancel")]
    fn cancel(&self);

    #[doc(alias = "gtk_menu_shell_deactivate")]
    fn deactivate(&self);

    #[doc(alias = "gtk_menu_shell_deselect")]
    fn deselect(&self);

    #[doc(alias = "gtk_menu_shell_get_parent_shell")]
    #[doc(alias = "get_parent_shell")]
    fn parent_shell(&self) -> Option<Widget>;

    #[doc(alias = "gtk_menu_shell_get_selected_item")]
    #[doc(alias = "get_selected_item")]
    fn selected_item(&self) -> Option<Widget>;

    #[doc(alias = "gtk_menu_shell_get_take_focus")]
    #[doc(alias = "get_take_focus")]
    fn takes_focus(&self) -> bool;

    #[doc(alias = "gtk_menu_shell_insert")]
    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32);

    #[doc(alias = "gtk_menu_shell_prepend")]
    fn prepend<P: IsA<Widget>>(&self, child: &P);

    #[doc(alias = "gtk_menu_shell_select_first")]
    fn select_first(&self, search_sensitive: bool);

    #[doc(alias = "gtk_menu_shell_select_item")]
    fn select_item<P: IsA<Widget>>(&self, menu_item: &P);

    #[doc(alias = "gtk_menu_shell_set_take_focus")]
    fn set_take_focus(&self, take_focus: bool);

    #[doc(alias = "activate-current")]
    fn connect_activate_current<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_current(&self, force_hide: bool);

    #[doc(alias = "cancel")]
    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancel(&self);

    #[doc(alias = "cycle-focus")]
    fn connect_cycle_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cycle_focus(&self, direction: DirectionType);

    #[doc(alias = "deactivate")]
    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "insert")]
    fn connect_insert<F: Fn(&Self, &Widget, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "move-current")]
    fn connect_move_current<F: Fn(&Self, MenuDirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_current(&self, direction: MenuDirectionType);

    #[doc(alias = "move-selected")]
    fn connect_move_selected<F: Fn(&Self, i32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "selection-done")]
    fn connect_selection_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "take-focus")]
    fn connect_take_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuShell>> MenuShellExt for O {
    fn activate_item<P: IsA<Widget>>(&self, menu_item: &P, force_deactivate: bool) {
        unsafe {
            ffi::gtk_menu_shell_activate_item(
                self.as_ref().to_glib_none().0,
                menu_item.as_ref().to_glib_none().0,
                force_deactivate.into_glib(),
            );
        }
    }

    fn append<P: IsA<MenuItem>>(&self, child: &P) {
        unsafe {
            ffi::gtk_menu_shell_append(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn bind_model<P: IsA<gio::MenuModel>>(
        &self,
        model: Option<&P>,
        action_namespace: Option<&str>,
        with_separators: bool,
    ) {
        unsafe {
            ffi::gtk_menu_shell_bind_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                action_namespace.to_glib_none().0,
                with_separators.into_glib(),
            );
        }
    }

    fn cancel(&self) {
        unsafe {
            ffi::gtk_menu_shell_cancel(self.as_ref().to_glib_none().0);
        }
    }

    fn deactivate(&self) {
        unsafe {
            ffi::gtk_menu_shell_deactivate(self.as_ref().to_glib_none().0);
        }
    }

    fn deselect(&self) {
        unsafe {
            ffi::gtk_menu_shell_deselect(self.as_ref().to_glib_none().0);
        }
    }

    fn parent_shell(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_shell_get_parent_shell(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn selected_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_shell_get_selected_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn takes_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_shell_get_take_focus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_menu_shell_insert(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    fn prepend<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_menu_shell_prepend(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn select_first(&self, search_sensitive: bool) {
        unsafe {
            ffi::gtk_menu_shell_select_first(
                self.as_ref().to_glib_none().0,
                search_sensitive.into_glib(),
            );
        }
    }

    fn select_item<P: IsA<Widget>>(&self, menu_item: &P) {
        unsafe {
            ffi::gtk_menu_shell_select_item(
                self.as_ref().to_glib_none().0,
                menu_item.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_take_focus(&self, take_focus: bool) {
        unsafe {
            ffi::gtk_menu_shell_set_take_focus(
                self.as_ref().to_glib_none().0,
                take_focus.into_glib(),
            );
        }
    }

    fn connect_activate_current<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_current_trampoline<
            P: IsA<MenuShell>,
            F: Fn(&P, bool) + 'static,
        >(
            this: *mut ffi::GtkMenuShell,
            force_hide: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(force_hide),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-current\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_current_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate_current(&self, force_hide: bool) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("activate-current", &[&force_hide])
                .unwrap()
        };
    }

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<P: IsA<MenuShell>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMenuShell,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cancel(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("cancel", &[])
                .unwrap()
        };
    }

    fn connect_cycle_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cycle_focus_trampoline<
            P: IsA<MenuShell>,
            F: Fn(&P, DirectionType) + 'static,
        >(
            this: *mut ffi::GtkMenuShell,
            direction: ffi::GtkDirectionType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(direction),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_focus(&self, direction: DirectionType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("cycle-focus", &[&direction])
                .unwrap()
        };
    }

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn deactivate_trampoline<P: IsA<MenuShell>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMenuShell,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deactivate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deactivate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert<F: Fn(&Self, &Widget, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn insert_trampoline<
            P: IsA<MenuShell>,
            F: Fn(&P, &Widget, i32) + 'static,
        >(
            this: *mut ffi::GtkMenuShell,
            child: *mut ffi::GtkWidget,
            position: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(child),
                position,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_move_current<F: Fn(&Self, MenuDirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_current_trampoline<
            P: IsA<MenuShell>,
            F: Fn(&P, MenuDirectionType) + 'static,
        >(
            this: *mut ffi::GtkMenuShell,
            direction: ffi::GtkMenuDirectionType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(direction),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-current\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_current_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_current(&self, direction: MenuDirectionType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("move-current", &[&direction])
                .unwrap()
        };
    }

    fn connect_move_selected<F: Fn(&Self, i32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_selected_trampoline<
            P: IsA<MenuShell>,
            F: Fn(&P, i32) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkMenuShell,
            distance: libc::c_int,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &MenuShell::from_glib_borrow(this).unsafe_cast_ref(),
                distance,
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selection_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_done_trampoline<P: IsA<MenuShell>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMenuShell,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-done\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    selection_done_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_take_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_take_focus_trampoline<
            P: IsA<MenuShell>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMenuShell,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&MenuShell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::take-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_take_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MenuShell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MenuShell")
    }
}
