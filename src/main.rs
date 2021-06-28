extern crate rspirv;
extern crate spirv_cross;
extern crate spirv_headers as spirv;
use shaderc;
use spirv_cross::{glsl, hlsl, msl, ErrorCode};
use std::{fs::File, io::Write};

fn example(module: spirv_cross::spirv::Module) -> Result<(), ErrorCode> {
    // Compile to HLSL
    let mut ast = spirv_cross::spirv::Ast::<hlsl::Target>::parse(&module)?;
    match ast.compile() {
        Ok(result) => {
            // std::fs::cre
            let mut file = File::create("src/out/out.hlsl").expect("");
            file.write_all(result.as_bytes()).expect("");
        }
        Err(code) => eprintln!("ERROR: {:?}", code),
    }

    // Compile to GLSL
    let mut ast = spirv_cross::spirv::Ast::<glsl::Target>::parse(&module)?;
    match ast.compile() {
        Ok(result) => {
            // std::fs::cre
            let mut file = File::create("src/out/out.glsl").expect("");
            file.write_all(result.as_bytes()).expect("");
        }
        Err(code) => eprintln!("ERROR: {:?}", code),
    }

    // Compile to MSL
    let mut ast = spirv_cross::spirv::Ast::<msl::Target>::parse(&module)?;

    match ast.compile() {
        Ok(result) => {
            let mut file = File::create("src/out/out.metal").expect("");
            file.write_all(result.as_bytes()).expect("");
        }
        Err(code) => eprintln!("ERROR: {:?}", code),
    }

    Ok(())
}

fn main() {
    let b = include_str!("./shader.vert");

    let mut compiler = shaderc::Compiler::new().unwrap();
    let options = shaderc::CompileOptions::new().unwrap();
    let result = compiler
        .compile_into_spirv(
            b,
            shaderc::ShaderKind::Vertex,
            "shader.vert",
            "main",
            Some(&options),
        )
        .expect("Compilation Error:\n");

    let asm = compiler
        .compile_into_spirv_assembly(
            b,
            shaderc::ShaderKind::Vertex,
            "shader.vert",
            "main",
            Some(&options),
        )
        .unwrap();

    let mut sp = File::create("src/out/out.spv").unwrap();
    sp.write_all(asm.as_text().as_bytes()).unwrap();

    // println!("u8\n\n{:?}\n\n", b);

    example(spirv_cross::spirv::Module::from_words(result.as_binary())).unwrap();
}
