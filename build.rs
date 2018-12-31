use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let srcs=["io.S"];
    let lib_name="myffi";

    for i in 0..srcs.len(){
        let src=Path::new(srcs[i]).file_stem().unwrap().to_str().unwrap();
        Command::new("arm-none-eabi-as").args(&[&format!("src/asm/{}",srcs[i]),"-g","-mthumb","-o"])
                        .arg(&format!("{}/{}.o", out_dir,src))
                        .status().unwrap();
        Command::new("arm-none-eabi-gcc-ar").args(&["crus", &format!("lib{}.a",lib_name), &format!("{}.o",src)])
                        .current_dir(&Path::new(&out_dir))
                        .status().unwrap();
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}",lib_name);
}
