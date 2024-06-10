#[derive(Debug)]
#[repr(C)]
pub struct CppClass {
    pub vtable: *const CppClassVTable,
    /* other fields... */
}

impl Default for CppClass {
    fn default() -> Self {
        static VTABLE: CppClassVTable = CppClassVTable { foo: foo_original };

        Self { vtable: &VTABLE }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CppClassVTable {
    pub foo: unsafe extern "system" fn(thisptr: *const CppClass) -> std::os::raw::c_int,
    /* other methods... */
}

unsafe extern "system" fn foo_original(_thisptr: *const CppClass) -> std::os::raw::c_int {
    0
}

unsafe extern "system" fn foo_hooked(_thisptr: *const CppClass) -> std::os::raw::c_int {
    1
}

fn main() {
    unsafe {
        /* The same classes, but one is our victim, and the other is unaffected. */
        let mut victim_cpp_class = CppClass::default();
        let unaffected_cpp_class = CppClass::default();

        /* Setting up raw hook */
        let original_vtable = vtable_hook::VTable::new_with_size(victim_cpp_class.vtable as _, 1);
        let mut raw_hook = vtable_hook::hook::copy::raw::RawHook::new(
            &mut victim_cpp_class.vtable as *mut _ as _,
            Some(original_vtable),
        );
        eprintln!("Raw hook: {raw_hook:#?}");

        /* Hooks are unset now, let's test that its true */
        eprintln!("-- Hook is disabled -- ");
        eprintln!(
            "victim_cpp_class's raw_hook is_enabled {}",
            raw_hook.is_enabled()
        );
        eprintln!(
            "victim_cpp_class foo() result = {}",
            (victim_cpp_class.vtable.read().foo)(&victim_cpp_class)
        );
        eprintln!(
            "unaffected_cpp_class foo() result = {}",
            (unaffected_cpp_class.vtable.read().foo)(&unaffected_cpp_class)
        );
        /* victim_cpp_class's raw_hook is_enabled false
         * victim_cpp_class foo() result = 0
         * unaffected_cpp_class foo() result = 0 */

        /* Replacing foo inside raw_hook */
        raw_hook.replace_method(0, foo_hooked as _);
        /* Replacing victim_cpp_class's VTable */
        raw_hook.enable();
        /* Testing */
        eprintln!("-- Hook is enabled -- ");
        eprintln!(
            "victim_cpp_class's raw_hook is_enabled {}",
            raw_hook.is_enabled()
        );
        eprintln!(
            "victim_cpp_class foo() result = {}",
            (victim_cpp_class.vtable.read().foo)(&victim_cpp_class)
        );
        eprintln!(
            "unaffected_cpp_class foo() result = {}",
            (unaffected_cpp_class.vtable.read().foo)(&unaffected_cpp_class)
        );
        /* victim_cpp_class's raw_hook is_enabled true
         * victim_cpp_class foo() result = 1
         * unaffected_cpp_class foo() result = 0 */

        /* Restoring victim_cpp_class's VTable */
        raw_hook.disable();
        /* NOTE: raw_hook's foo method still points to our function,
         * but we have restored original VTable inside victim_cpp_class
         * To restore methods inside raw_hook use raw_hook.restore_method()
         * or raw_hook.restore_all() */
        eprintln!("-- Hook is disabled -- ");
        eprintln!(
            "victim_cpp_class's raw_hook is_enabled {}",
            raw_hook.is_enabled()
        );
        eprintln!(
            "victim_cpp_class foo() result = {}",
            (victim_cpp_class.vtable.read().foo)(&victim_cpp_class)
        );
        eprintln!(
            "unaffected_cpp_class foo() result = {}",
            (unaffected_cpp_class.vtable.read().foo)(&unaffected_cpp_class)
        );
        /* victim_cpp_class's raw_hook is_enabled false
         * victim_cpp_class foo() result = 0
         * unaffected_cpp_class foo() result = 0 */
    }
}
