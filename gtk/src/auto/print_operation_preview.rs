// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::PageSetup;
use crate::PrintContext;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkPrintOperationPreview")]
    pub struct PrintOperationPreview(Interface<ffi::GtkPrintOperationPreview, ffi::GtkPrintOperationPreviewIface>);

    match fn {
        type_ => || ffi::gtk_print_operation_preview_get_type(),
    }
}

impl PrintOperationPreview {
    pub const NONE: Option<&'static PrintOperationPreview> = None;
}

pub trait PrintOperationPreviewExt: 'static {
    #[doc(alias = "gtk_print_operation_preview_end_preview")]
    fn end_preview(&self);

    #[doc(alias = "gtk_print_operation_preview_is_selected")]
    fn is_selected(&self, page_nr: i32) -> bool;

    #[doc(alias = "gtk_print_operation_preview_render_page")]
    fn render_page(&self, page_nr: i32);

    #[doc(alias = "got-page-size")]
    fn connect_got_page_size<F: Fn(&Self, &PrintContext, &PageSetup) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "ready")]
    fn connect_ready<F: Fn(&Self, &PrintContext) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintOperationPreview>> PrintOperationPreviewExt for O {
    fn end_preview(&self) {
        unsafe {
            ffi::gtk_print_operation_preview_end_preview(self.as_ref().to_glib_none().0);
        }
    }

    fn is_selected(&self, page_nr: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_operation_preview_is_selected(
                self.as_ref().to_glib_none().0,
                page_nr,
            ))
        }
    }

    fn render_page(&self, page_nr: i32) {
        unsafe {
            ffi::gtk_print_operation_preview_render_page(self.as_ref().to_glib_none().0, page_nr);
        }
    }

    fn connect_got_page_size<F: Fn(&Self, &PrintContext, &PageSetup) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn got_page_size_trampoline<
            P: IsA<PrintOperationPreview>,
            F: Fn(&P, &PrintContext, &PageSetup) + 'static,
        >(
            this: *mut ffi::GtkPrintOperationPreview,
            context: *mut ffi::GtkPrintContext,
            page_setup: *mut ffi::GtkPageSetup,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                PrintOperationPreview::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(context),
                &from_glib_borrow(page_setup),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"got-page-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    got_page_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ready<F: Fn(&Self, &PrintContext) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn ready_trampoline<
            P: IsA<PrintOperationPreview>,
            F: Fn(&P, &PrintContext) + 'static,
        >(
            this: *mut ffi::GtkPrintOperationPreview,
            context: *mut ffi::GtkPrintContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                PrintOperationPreview::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(context),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"ready\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    ready_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PrintOperationPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PrintOperationPreview")
    }
}
