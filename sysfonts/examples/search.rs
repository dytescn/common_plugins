use font_kit::source::SystemSource;

fn main() {
    let font = SystemSource::new()
        .select_by_postscript_name("ArialMT")
        .unwrap()
        .load()
        .unwrap();
    println!("Font: {}", font.full_name());
}