extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main(){
    #[cfg(target_os = "windows")]
    let bindings = {
        println!("cargo:rustc-link-search={}/windows/lib",std::env::var("CARGO_MANIFEST_DIR").unwrap());

        #[cfg(target_pointer_width = "32")]
        println!("cargo:rustc-link-lib=msc");
    
        #[cfg(target_pointer_width = "64")]
        println!("cargo:rustc-link-lib=msc_x64");
    
        println!("cargo:rerun-if-changed={}/windows/include/wrapper.h",std::env::var("CARGO_MANIFEST_DIR").unwrap());
    
        bindgen::Builder::default()
            .header(format!("{}/windows/include/wrapper.h",std::env::var("CARGO_MANIFEST_DIR").unwrap()))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings")
    
    };
    #[cfg(target_os = "linux")]
    let bindings = {
        #[cfg(target_pointer_width = "32")]
        println!("cargo:rustc-link-search={}/linux/libs/x86",std::env::var("CARGO_MANIFEST_DIR").unwrap());

        #[cfg(target_pointer_width = "64")]
        println!("cargo:rustc-link-search={}/linux/libs/x64",std::env::var("CARGO_MANIFEST_DIR").unwrap());

        println!("cargo:rustc-link-lib=msc");
    
        println!("cargo:rerun-if-changed={}/linux/include/wrapper.h",std::env::var("CARGO_MANIFEST_DIR").unwrap());
    
        bindgen::Builder::default()
            .header(format!("{}/linux/include/wrapper.h",std::env::var("CARGO_MANIFEST_DIR").unwrap()))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings")
    
    };
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}