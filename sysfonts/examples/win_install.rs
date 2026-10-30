// use std::path::{Path, PathBuf};
// use std::fs;
// use windows::{
//     core::PCWSTR,
//     Win32::{
//         Graphics::Gdi::{AddFontResourceExW, ADD_FONT_RESOURCE_ENUM},
//         System::Registry::{RegSetValueExW, HKEY_LOCAL_MACHINE, RegOpenKeyExW, RegCloseKey, HKEY, KEY_SET_VALUE, REG_SZ},
//         Storage::FileSystem::{CopyFileW},
//     },
// };

// fn to_wide(s: &str) -> Vec<u16> {
//     use std::os::windows::prelude::*;
//     std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
// }

// fn install_font_windows(font_src: &str) -> windows::core::Result<()> {
//     let font_name = Path::new(font_src).file_name().unwrap().to_str().unwrap();
//     let font_dst = format!(r"C:\Windows\Fonts\{}", font_name);

//     // 1. 拷贝字体文件
//     unsafe {
//         CopyFileW(PCWSTR::from_raw(to_wide(font_src).as_ptr()), PCWSTR::from_raw(to_wide(&font_dst).as_ptr()), false)?;
//     }

//     // 2. 添加到注册表
//     let reg_path = to_wide(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts");
//     let mut hkey: HKEY = HKEY::default();

//     unsafe {
//         RegOpenKeyExW(
//             HKEY_LOCAL_MACHINE,
//             PCWSTR::from_raw(reg_path.as_ptr()),
//             0,
//             KEY_SET_VALUE,
//             &mut hkey,
//         )?;

//         let value_name = to_wide(&format!("{} (TrueType)", font_name.trim_end_matches(".ttf"))); // 注册表显示名
//         let data = to_wide(font_name);
//         RegSetValueExW(
//             hkey,
//             PCWSTR::from_raw(value_name.as_ptr()),
//             0,
//             REG_SZ,
//             Some(bytemuck::cast_slice(&data)),
//         )?;
//         RegCloseKey(hkey);
//     }

//     // 3. 告知系统加载字体（立即生效）
//     unsafe {
//         let count: i32 = AddFontResourceExW(
//             PCWSTR::from_raw(to_wide(&font_dst).as_ptr()),
//             ADD_FONT_RESOURCE_ENUM(0x10), // FR_PRIVATE
//             std::ptr::null_mut(),
//         );
//         if count == 0 {
//             println!("Font copied, but not immediately added.");
//         } else {
//             println!("Font successfully installed and added.");
//         }
//     }

//     Ok(())
// }

fn main() {
    // match install_font_windows("MyFont.ttf") {
    //     Ok(_) => println!("Font installation complete."),
    //     Err(e) => eprintln!("Error: {e:?}"),
    // }
}
