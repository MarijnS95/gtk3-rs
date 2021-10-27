// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;

use crate::TreeView;

pub trait TreeViewImpl: ContainerImpl {}

unsafe impl<T: ContainerImpl> IsSubclassable<T> for TreeView {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
    }
}
