// use std::fs;
// use std::path::{Path, PathBuf};

// fn install_font_on_macos(font_path: &str) -> std::io::Result<()> {
//     let home = home_dir().expect("Failed to get home directory");

//     let font_dir = home.join("Library/Fonts");
//     fs::create_dir_all(&font_dir)?; // 确保目录存在

//     let font_file_name = Path::new(font_path).file_name().unwrap();
//     let dest_path = font_dir.join(font_file_name);

//     fs::copy(font_path, &dest_path)?;
//     println!("✅ Font installed to: {}", dest_path.display());

//     Ok(())
// }

fn main() {
    // let font_path = "MyFont.ttf"; // 替换为你的字体路径

    // if let Err(e) = install_font_on_macos(font_path) {
    //     eprintln!("❌ Font installation failed: {e}");
    // }
}