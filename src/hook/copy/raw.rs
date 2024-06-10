#[derive(Debug)]
pub struct RawHook {
    /* Struct's VTable field ptr */
    struct_vtable_field_ptr: *mut crate::RawVTable,
    /* Original VTable */
    original_vtable: crate::VTable,
    /* Our VTable */
    our_vtable: Vec<crate::Method>,
}

impl RawHook {
    pub unsafe fn new(
        struct_vtable_field_ptr: *mut crate::RawVTable,
        original_vtable: Option<crate::VTable>,
    ) -> Self {
        let original_vtable = match original_vtable {
            Some(some) => some,
            None => crate::VTable::new(struct_vtable_field_ptr.read_unaligned()),
        };

        let our_vtable = original_vtable.as_slice().to_vec();

        Self {
            struct_vtable_field_ptr,
            original_vtable,
            our_vtable,
        }
    }

    pub unsafe fn is_enabled(&self) -> bool {
        let current_vtable_ptr = self.struct_vtable_field_ptr.read_unaligned();
        std::ptr::addr_eq(current_vtable_ptr, self.our_vtable.as_ptr())
    }

    pub unsafe fn enable(&mut self) -> bool {
        if self.is_enabled() {
            return false;
        }

        self.struct_vtable_field_ptr
            .replace(self.our_vtable.as_mut_ptr());

        true
    }

    pub unsafe fn disable(&mut self) -> bool {
        if !self.is_enabled() {
            return false;
        }

        self.struct_vtable_field_ptr
            .replace(self.original_vtable.begin);

        true
    }

    pub unsafe fn replace_method(&mut self, index: usize, our_method: crate::Method) -> Option<()> {
        let item = self.our_vtable.get_mut(index)?;
        *item = our_method;

        Some(())
    }

    pub unsafe fn restore_method(&mut self, index: usize) -> Option<()> {
        let item = self.our_vtable.get_mut(index)?;
        let original_method = self.original_vtable.as_slice().get(index)?;
        *item = *original_method;

        Some(())
    }

    pub unsafe fn restore_all(&mut self) {
        let original_methods = self.original_vtable.as_slice();
        for (index, item) in self.our_vtable.iter_mut().enumerate() {
            let Some(original_method) = original_methods.get(index) else {
                continue;
            };

            *item = *original_method;
        }
    }
}
