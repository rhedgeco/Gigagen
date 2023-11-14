use std::fs;

use glob::glob;

fn main() {
    fs::create_dir_all("bindings").unwrap();

    let mut builder = csbindgen::Builder::default();
    for entry in glob("src/**/*.rs").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            builder = builder.input_extern_file(path);
        }
    }

    builder
        .csharp_dll_name("gigagen")
        .csharp_namespace("Gigagen.Native")
        .csharp_class_name("Func")
        .csharp_use_function_pointer(false)
        .csharp_class_accessibility("internal")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .generate_csharp_file("bindings/Native.cs")
        .unwrap();
}
