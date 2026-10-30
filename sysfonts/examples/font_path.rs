use font_kit::{handle::Handle, source::SystemSource};
fn main(){
    let source = SystemSource::new();
    let handle = source.select_by_postscript_name("ArialMT").unwrap();
    if let Handle::Path { path, .. } = handle {
        println!("Path: {:?}", path);
    } 
}