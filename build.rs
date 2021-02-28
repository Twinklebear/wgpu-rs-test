extern crate shaderc;

use std::{env, process};
use std::fs::{self, File};
use std::path::Path;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let shaders = [Path::new("shaders/triangle.frag"), Path::new("shaders/triangle.vert")];

    let mut compiler = shaderc::Compiler::new().unwrap();

    let out_file_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("embedded_shaders.rs");
    let out_file = File::create(out_file_path).unwrap();
    let mut writer = BufWriter::new(out_file);
    writer.write_all(b"mod shaders {\n").unwrap();

    for s in shaders.iter() {
        println!("cargo:rerun-if-changed={}", s.display());
        let shader_code = fs::read_to_string(s).expect(&format!("Failed to read {}", s.display()));
        let compile_result = compiler
            .compile_into_spirv(
                &shader_code,
                shaderc::ShaderKind::InferFromSource,
                s.to_str().unwrap(),
                "main",
                None,
            );
        if let Ok(binary) = compile_result {
            let var_name = s.file_name().unwrap().to_str().unwrap().replace(".", "_").to_ascii_uppercase();
            let spv = binary.as_binary();
            writer.write_all(format!("pub const {}: [u32; {}] = {:?};\n", var_name, spv.len(), spv).as_bytes()).unwrap();
        } else {
            println!("cargo:warning=Shader '{}' failed to compile due to:", s.display());
            match compile_result.err().unwrap() {
                shaderc::Error::CompilationError(_, msg) => println!("cargo:warning=Compile Error:--\n{}--", msg),
                shaderc::Error::InternalError(msg) => println!("cargo:warning=Internal Error: {}", msg),
                shaderc::Error::InvalidStage(msg) => println!("cargo:warning=Invalid Stage: {}", msg),
                shaderc::Error::InvalidAssembly(msg) => println!("cargo:warning=Invalid Assembly: {}", msg),
                shaderc::Error::NullResultObject(msg) => println!("cargo:warning=Null Result Object: {}", msg),
            }
            
            process::exit(1);
        }
    }

    writer.write_all(b"}\n").unwrap();
}

