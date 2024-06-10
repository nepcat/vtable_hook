pub mod raw;

#[derive(Debug)]
pub struct Hook<'a, T> {
    pub item: &'a mut T,
    raw: raw::RawHook,
}

impl<'a, T> Hook<'a, T> {
    pub unsafe fn new(
        item: &'a mut T,
        vtable_offset: Option<usize>,
        methods_count: Option<usize>,
    ) -> Self {
        let item_ptr = item as *mut _ as *mut usize;

        /* TODO: More sanity checks. Example:
         * Error when vtable_offset is out of T's struct memory */

        let vtable_offset = vtable_offset.unwrap_or(0);
        let struct_vtable_field_ptr = item_ptr.add(vtable_offset) as *mut crate::RawVTable;
        let vtable = struct_vtable_field_ptr.read_unaligned();
        let vtable_size = methods_count.unwrap_or_else(|| crate::VTable::count_methods_raw(vtable));
        let original_vtable = crate::VTable::new_with_size(vtable, vtable_size);

        let raw = raw::RawHook::new(struct_vtable_field_ptr, Some(original_vtable));
        Self { item, raw }
    }

    /* Wrapped functions */
    pub unsafe fn is_enabled(&self) -> bool {
        self.raw.is_enabled()
    }

    pub unsafe fn enable(&mut self) -> bool {
        self.raw.enable()
    }

    pub unsafe fn disable(&mut self) -> bool {
        self.raw.disable()
    }

    pub unsafe fn replace_method(&mut self, index: usize, our_method: crate::Method) -> Option<()> {
        self.raw.replace_method(index, our_method)
    }

    pub unsafe fn restore_method(&mut self, index: usize) -> Option<()> {
        self.raw.restore_method(index)
    }

    pub unsafe fn restore_all(&mut self) {
        self.raw.restore_all()
    }
}

impl<'a, T> Drop for Hook<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.raw.disable();
        }
    }
}
