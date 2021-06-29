extern crate rspirv;
extern crate spirv_cross;
extern crate spirv_headers as spirv;
use shaderc;

// OpenGL
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use spirv_cross::glsl;

// HLSL used by DX
#[cfg(target_os = "windows")]
use spirv_cross::hlsl;

// Metal (Metal Shading Language)
#[cfg(target_os = "macos")]
use spirv_cross::msl;

use spirv_cross::ErrorCode;

use std::{fs::File, io::Write};

// #[cfg_attr(target_os="macos", feature = "metal")]

fn example(module: spirv_cross::spirv::Module) -> Result<(), ErrorCode> {
    #[cfg(target_os = "windows")]
    {
        // Compile to HLSL
        let mut ast = spirv_cross::spirv::Ast::<hlsl::Target>::parse(&module)?;
        match ast.compile() {
            Ok(result) => {
                // std::fs::cre
                let mut file = File::create("out/out.hlsl").expect("");
                file.write_all(result.as_bytes()).expect("");
            }
            Err(code) => eprintln!("ERROR: {:?}", code),
        }
    }

    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        // Compile to GLSL
        let mut ast = spirv_cross::spirv::Ast::<glsl::Target>::parse(&module)?;
        match ast.compile() {
            Ok(result) => {
                // std::fs::cre
                let mut file = File::create("out/out.glsl").expect("");
                file.write_all(result.as_bytes()).expect("");
            }
            Err(code) => eprintln!("ERROR: {:?}", code),
        }
    }

    // Compile to MSL
    #[cfg(target_os = "macos")]
    {
        let mut ast = spirv_cross::spirv::Ast::<msl::Target>::parse(&module)?;

        match ast.compile() {
            Ok(result) => {
                let mut file = File::create("out/out.metal").expect("");
                file.write_all(result.as_bytes()).expect("");
            }
            Err(code) => eprintln!("ERROR: {:?}", code),
        }
    }

    Ok(())
}
fn main() {
    let b = include_str!("./shader.vert");

    let mut compiler = shaderc::Compiler::new().unwrap();
    let options = shaderc::CompileOptions::new().unwrap();

    // Compile the GLSL into Spir-V Binary
    let result = compiler
        .compile_into_spirv(
            b,
            shaderc::ShaderKind::Vertex,
            "shader.vert",
            "main",
            Some(&options),
        )
        .expect("Compilation Error:\n");

    // Vulkan - Spir-v assembly
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        let asm = compiler
            .compile_into_spirv_assembly(
                b,
                shaderc::ShaderKind::Vertex,
                "shader.vert",
                "main",
                Some(&options),
            )
            .unwrap();

        let mut sp = File::create("out/out.spvasm").unwrap();
        sp.write_all(asm.as_text().as_bytes()).unwrap();
    }

    // println!("u8\n\n{:?}\n\n", b);

    example(spirv_cross::spirv::Module::from_words(result.as_binary())).unwrap();
}
