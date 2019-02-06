// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Object;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;

glib_wrapper! {
    pub struct TableCell(Interface<ffi::AtkTableCell>) @requires Object;

    match fn {
        get_type => || ffi::atk_table_cell_get_type(),
    }
}

pub const NONE_TABLE_CELL: Option<&TableCell> = None;

pub trait TableCellExt: 'static {
    //fn get_column_header_cells(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 9 };

    fn get_column_span(&self) -> i32;

    fn get_position(&self) -> Option<(i32, i32)>;

    fn get_row_column_span(&self) -> Option<(i32, i32, i32, i32)>;

    //fn get_row_header_cells(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 9 };

    fn get_row_span(&self) -> i32;

    fn get_table(&self) -> Option<Object>;
}

impl<O: IsA<TableCell>> TableCellExt for O {
    //fn get_column_header_cells(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 9 } {
    //    unsafe { TODO: call ffi::atk_table_cell_get_column_header_cells() }
    //}

    fn get_column_span(&self) -> i32 {
        unsafe {
            ffi::atk_table_cell_get_column_span(self.as_ref().to_glib_none().0)
        }
    }

    fn get_position(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut row = mem::uninitialized();
            let mut column = mem::uninitialized();
            let ret = from_glib(ffi::atk_table_cell_get_position(self.as_ref().to_glib_none().0, &mut row, &mut column));
            if ret { Some((row, column)) } else { None }
        }
    }

    fn get_row_column_span(&self) -> Option<(i32, i32, i32, i32)> {
        unsafe {
            let mut row = mem::uninitialized();
            let mut column = mem::uninitialized();
            let mut row_span = mem::uninitialized();
            let mut column_span = mem::uninitialized();
            let ret = from_glib(ffi::atk_table_cell_get_row_column_span(self.as_ref().to_glib_none().0, &mut row, &mut column, &mut row_span, &mut column_span));
            if ret { Some((row, column, row_span, column_span)) } else { None }
        }
    }

    //fn get_row_header_cells(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 9 } {
    //    unsafe { TODO: call ffi::atk_table_cell_get_row_header_cells() }
    //}

    fn get_row_span(&self) -> i32 {
        unsafe {
            ffi::atk_table_cell_get_row_span(self.as_ref().to_glib_none().0)
        }
    }

    fn get_table(&self) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_table_cell_get_table(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for TableCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TableCell")
    }
}
