// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use std::{ptr, str};
use std::libc::c_float;

use traits::{GtkWidget, GtkContainer};
use gtk::enums::GtkShadowType;
use utils::cast::GTK_FRAME;
use ffi;

pub trait GtkFrame: GtkWidget + GtkContainer {
    fn set_label(&mut self, label: Option<&str>) -> () {
        match label {
            Some(l) => unsafe { l.with_c_str(|c_str| {ffi::gtk_frame_set_label(GTK_FRAME(self.get_widget()), c_str) }) },
            None    => unsafe { ffi::gtk_frame_set_label(GTK_FRAME(self.get_widget()), ptr::null()) }
        };
    }

    fn set_label_widget<T: GtkWidget>(&mut self, label_widget: &T) -> () {
        unsafe {
            ffi::gtk_frame_set_label_widget(GTK_FRAME(self.get_widget()), label_widget.get_widget());
        }
    }

    fn set_label_align(&mut self, x_align: f32, y_align: f32) -> () {
        unsafe {
            ffi::gtk_frame_set_label_align(GTK_FRAME(self.get_widget()), x_align as c_float, y_align as c_float);
        }
    }

    fn get_label_align(&self) -> (f32, f32) {
        let x_align = 0.;
        let y_align = 0.;
        unsafe {
            ffi::gtk_frame_get_label_align(GTK_FRAME(self.get_widget()), &x_align, &y_align);
        }
        (x_align as f32, y_align as f32)
    }

    fn set_shadow_type(&mut self, st_type: GtkShadowType) -> () {
        unsafe {
            ffi::gtk_frame_set_shadow_type(GTK_FRAME(self.get_widget()), st_type);
        }
    }

    fn get_label(&self) -> ~str {
        let c_str = unsafe { ffi::gtk_frame_get_label(GTK_FRAME(self.get_widget())) };
        unsafe {str::raw::from_c_str(c_str)}
    }

    fn gtk_frame_get_shadow_type(&self) -> GtkShadowType {
        unsafe {
            ffi::gtk_frame_get_shadow_type(GTK_FRAME(self.get_widget()))
        }
    }
}
