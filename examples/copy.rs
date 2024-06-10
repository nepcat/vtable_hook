#[derive(Debug)]
#[repr(C)]
pub struct CppClass {
    pub vtable: *const CppClassVTable,
    /* other fields... */
}

impl Default for CppClass {
    fn default() -> Self {
        static VTABLE: CppClassVTable = CppClassVTable {
            foo: foo_bar_original,
            bar: foo_bar_original,
        };

        Self { vtable: &VTABLE }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CppClassVTable {
    pub foo: unsafe extern "system" fn(thisptr: *const CppClass) -> std::os::raw::c_int,
    pub bar: unsafe extern "system" fn(thisptr: *const CppClass) -> std::os::raw::c_int,
    /* other methods... */
}

unsafe extern "system" fn foo_bar_original(_thisptr: *const CppClass) -> std::os::raw::c_int {
    0
}

unsafe extern "system" fn bar_hooked(_thisptr: *const CppClass) -> std::os::raw::c_int {
    1
}

fn main() {
    unsafe {
        /* The same classes, but one is our victim, and the other is unaffected. */
        let mut victim_cpp_class = CppClass::default();
        let unaffected_cpp_class = CppClass::default();

        /* If you can, use Hook over RawHook. It uses Rust's lifetime system
         * to force victim_cpp_class to stay valid while hook is set because
         * you can't drop victim_cpp_class while its being borrowed inside
         * copy::Hook */
        {
            /* Setting up hook */
            /* To get VTable size you can use multiple methods:
             * Calculate it on runtime by passing None as methods_count arg inside Hook::new() (very unsafe!)
             * If your VTable struct implementation is 100% valid, you can use
             * std::mem::size_of::<VTableStruct>() / std::mem::size_of::<usize>() to get
             * methods count at compile time, or just specify it manually: Some(2) */
            let vtable_size = std::mem::size_of::<CppClassVTable>() / std::mem::size_of::<usize>();
            let mut hook =
                vtable_hook::hook::copy::Hook::new(&mut victim_cpp_class, None, Some(vtable_size));
            eprintln!("Hook: {hook:#?}");
            /* Since we are mutually borrowing victim_cpp_class in hook we can't use it directly
             * but we can use hook's reference instead:
             * &hook.item */

            /* Hooks are unset now, let's test that its true */
            eprintln!("-- Hook is disabled -- ");
            eprintln!("victim_cpp_class's hook is_enabled {}", hook.is_enabled());
            {
                let victim_cpp_class = &hook.item;
                eprintln!(
                    "victim_cpp_class bar() result = {}",
                    (victim_cpp_class.vtable.read().bar)(*victim_cpp_class)
                );
            }
            eprintln!(
                "unaffected_cpp_class bar() result = {}",
                (unaffected_cpp_class.vtable.read().bar)(&unaffected_cpp_class)
            );
            /* victim_cpp_class's hook is_enabled false
             * victim_cpp_class bar() result = 0
             * unaffected_cpp_class bar() result = 0 */

            /* Replacing bar inside raw_hook */
            hook.replace_method(1, bar_hooked as _);
            /* Replacing victim_cpp_class's VTable */
            hook.enable();
            /* Testing */
            eprintln!("-- Hook is enabled -- ");
            eprintln!("victim_cpp_class's hook is_enabled {}", hook.is_enabled());
            {
                let victim_cpp_class = &hook.item;
                eprintln!(
                    "victim_cpp_class bar() result = {}",
                    (victim_cpp_class.vtable.read().bar)(*victim_cpp_class)
                );
            }
            eprintln!(
                "unaffected_cpp_class bar() result = {}",
                (unaffected_cpp_class.vtable.read().bar)(&unaffected_cpp_class)
            );
            /* victim_cpp_class's hook is_enabled true
             * victim_cpp_class bar() result = 1
             * unaffected_cpp_class bar() result = 0 */

            /* hook will disable itself on drop() */
        }
        eprintln!("-- Hook is disabled (drop) -- ");
        eprintln!(
            "victim_cpp_class bar() result = {}",
            (victim_cpp_class.vtable.read().bar)(&victim_cpp_class)
        );
        eprintln!(
            "unaffected_cpp_class bar() result = {}",
            (unaffected_cpp_class.vtable.read().bar)(&unaffected_cpp_class)
        );
        /* victim_cpp_class bar() result = 0
         * unaffected_cpp_class bar() result = 0 */
    }
}
