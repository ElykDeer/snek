#[allow(dead_code)]
#[cfg(target_os = "emscripten")]
pub mod emscripten {
    use std::os::raw::c_uint;

    extern "C" {
        pub fn emscripten_sleep(ms: c_uint);
        pub fn emscripten_run_script(script: *const u8);
        pub fn emscripten_get_element_css_size(
            target: *const u8,
            width: *mut f64,
            height: *mut f64,
        ) -> i32;
    }

    pub fn sleep(ms: u32) {
        unsafe {
            emscripten_sleep(ms);
        }
    }

    pub fn exec(script: &str) {
        unsafe {
            emscripten_run_script(script.as_ptr());
        }
    }

    pub fn get_canvas_size() -> (u32, u32) {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            emscripten_get_element_css_size("canvas\0".as_ptr(), &mut width, &mut height);
        }
        (width as u32, height as u32)
    }

    pub mod fs {
        use std::os::raw::{c_int, c_void};
        use std::ptr::null;

        #[allow(non_camel_case_types)]
        type em_arg_callback_func = unsafe extern "C" fn(*const c_void);

        extern "C" {
            pub fn emscripten_idb_async_store(
                db_name: *const u8,
                file_id: *const u8,
                ptr: *const c_void,
                num: c_int,
                arg: *const c_void,
                onstore: *const em_arg_callback_func,
                onerror: *const em_arg_callback_func,
            );

            // TODO : Need to free pbuffer
            pub fn emscripten_idb_load(
                db_name: *const u8,
                file_id: *const u8,
                pbuffer: *const *const c_void,
                pnum: *const c_int,
                perror: *const c_int,
            );
        }

        static DB_NAME: &'static str = "gamedata";
        static FILE_NAME: &'static str = "save.dat";

        pub fn save(data: &str) {
            unsafe {
                emscripten_idb_async_store(
                    DB_NAME.as_ptr(),
                    FILE_NAME.as_ptr(),
                    data.as_ptr() as *const _,
                    data.len() as i32,
                    null(),
                    null::<em_arg_callback_func>(),
                    null::<em_arg_callback_func>(),
                );
            }
        }

        pub fn get_save_data() -> String {
            let mut data = null();
            let mut len: i32 = 0;

            unsafe {
                emscripten_idb_load(
                    DB_NAME.as_ptr(),
                    FILE_NAME.as_ptr(),
                    &mut data as *mut _,
                    &mut len as *mut _,
                    null(),
                );

                // TODO : This is an intentional memory leak because I have no idea who's `free` I'm supposed to call - stdlib's? Rust's? JavaScript's?
                let intermediate_result =
                    String::from_raw_parts(data as *mut _, len as usize, len as usize);
                let result = intermediate_result.clone();
                std::mem::forget(intermediate_result);
                result
            }
        }
    }
}
