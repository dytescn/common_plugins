use std::ffi::CString;
use std::os::raw::c_char;

// 安装字体
#[unsafe(no_mangle)]
pub extern "C" fn install_font(font_path: *const c_char) -> *const c_char {
    let s = CString::new("Hello from Rust DLL!").unwrap();
    s.into_raw() // 返回给 C，调用方负责释放
}

// 导出字体
#[unsafe(no_mangle)]
pub extern "C" fn export_font(s: *mut c_char, font_path: *const c_char) -> *const c_char{
    // if s.is_null() { return; }
    // unsafe { CString::from_raw(s) };
    let s = CString::new("Hello from Rust DLL!").unwrap();
    s.into_raw() // 返回给 C，调用方负责释放
}

// 获取系统字体
#[unsafe(no_mangle)]
pub extern "C" fn system_fonts(s: *mut c_char) -> *const c_char {
    // if s.is_null() { return; }
    // unsafe { CString::from_raw(s) };
    let s = CString::new("Hello from Rust DLL!").unwrap();
    s.into_raw() // 返回给
}
// ==================== cyctron FFI 包装 ====================
// cyctron 按 <method>_<symbol> 查找符号；POST + FFI-Symbol 需要 post_* 前缀。

fn write_json_out(json_out: *mut c_char, json: &str) -> i32 {
    if json_out.is_null() {
        return 1;
    }
    let bytes = json.as_bytes();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), json_out as *mut u8, bytes.len());
        *json_out.add(bytes.len()) = 0;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn post_system_fonts(_json_in: *const c_char, json_out: *mut c_char) -> i32 {
    write_json_out(
        json_out,
        "{\"code\":200,\"data\":{\"fonts\":[]},\"msg\":\"success\"}",
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn post_install_font(_json_in: *const c_char, json_out: *mut c_char) -> i32 {
    write_json_out(
        json_out,
        "{\"code\":200,\"data\":{\"installed\":true},\"msg\":\"success\"}",
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn post_export_font(_json_in: *const c_char, json_out: *mut c_char) -> i32 {
    write_json_out(
        json_out,
        "{\"code\":200,\"data\":{\"exported\":true},\"msg\":\"success\"}",
    )
}
