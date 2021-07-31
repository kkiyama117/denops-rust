fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    run --manifest-path=../../denops-builder/Cargo.toml
    let mut plugins = Vec::new();
    for entry in WalkDir::new("src/plugins") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
            plugins.push(file_name);
        }
    }

    plugins.sort();

    let out = Path::new(&out_dir).join("gen.rs");
    let mut out = File::create(&out).unwrap();

    for file_name in &plugins {
        let _ = write!(out, "pub mod {};\n", file_name);
        let _ = write!(out, "pub use {}::*;\n", file_name);
    }
}
