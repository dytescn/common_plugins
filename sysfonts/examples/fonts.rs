use font_kit::source::SystemSource;
use font_kit::handle::Handle;
use std::fs;
use std::path::Path;

fn main() {
    let output_dir = Path::new("./exported_fonts");
    fs::create_dir_all(output_dir).unwrap();

    let fonts = SystemSource::new().all_fonts().unwrap();

    for font_handle in fonts {
        // 尝试加载字体获取信息
        if let Ok(font) = font_handle.load() {
            let name = font.full_name();
            if name == "Marlett" {
                match &font_handle {
                    Handle::Path { path, font_index: _ } => {
                        if let Some(file_name) = path.file_name() {
                            let dest_path = output_dir.join(file_name);
                            if let Err(e) = fs::copy(path, &dest_path) {
                                eprintln!("Failed to copy {}: {}", path.display(), e);
                            } else {
                                println!("Exported: {} -> {}", path.display(), dest_path.display());
                            }
                        }
                    }
                    Handle::Memory { .. } => {
                        println!("Marlett font is in memory, cannot export file");
                    }
                }
            }
        }
    }
}
