#![allow(clippy::missing_safety_doc, clippy::mut_from_ref)]

pub mod hook;

pub type RawVTable = *const Method;
pub type Method = *mut unsafe fn();

#[derive(Debug, Clone)]
pub struct VTable {
    pub begin: RawVTable,
    pub size: usize,
}

impl VTable {
    pub unsafe fn new(vtable: RawVTable) -> Self {
        Self::new_with_size(vtable, Self::count_methods_raw(vtable))
    }

    pub unsafe fn new_with_size(vtable: RawVTable, size: usize) -> Self {
        Self {
            begin: vtable,
            size,
        }
    }

    pub unsafe fn count_methods_raw(mut vtable: RawVTable) -> usize {
        /* Try to dynamically count amount of methods in a VTable, very unsafe. */
        let mut size = 0;
        while !std::ptr::read(vtable).is_null() {
            vtable = vtable.add(1);
            size += 1;
        }
        size
    }

    pub unsafe fn as_slice(&self) -> &[Method] {
        std::slice::from_raw_parts(self.begin, self.size)
    }

    // TODO
    /*pub unsafe fn as_mut_slice(&self) -> &mut [Method] {
        std::slice::from_raw_parts_mut(self.begin, self.size)
    }*/
}
