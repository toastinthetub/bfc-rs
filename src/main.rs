// main.rs

pub const RED_ANSII: &str = "\x1b[31m";
pub const GREEN_ANSII: &str = "\x1b[32m";
pub const RESET_ANSII: &str = "\x1b[0m";

mod ast;
mod emitter;
mod lexer;
mod parser;

use std::fs;
use std::process::{Command, exit};

use clap::{Arg, ArgAction, Command as ClapCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = ClapCommand::new("bfc-rs")
        .version("0.1")
        .author("honklespornus")
        .about("compiles brainfuck to NASM x86-64 assembly")
        .arg(
            Arg::new("input")
                .help("input brainfuck file")
                .required(true)
                .num_args(1)
                .short('i')
                .long("input"),
        )
        .arg(
            Arg::new("asm")
                .help("output assembly filename")
                .num_args(1)
                .short('a')
                .long("asm")
                .default_value("out.asm"),
        )
        .arg(
            Arg::new("exe")
                .help("output executable filename")
                .num_args(1)
                .short('e')
                .long("exe")
                .default_value("out_executable"),
        )
        .arg(
            Arg::new("feelinglazy")
                .help("automatically assemble and link if we can")
                .long("feelinglazy")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // this is an error message, the call to format!() just needs to be outside the panic handler
    let ok_compiler_i_made_it_a_string: String = format!(
        "{}ya do need to input a file for this to work{}",
        RED_ANSII, RESET_ANSII
    );

    let input = matches
        .get_one::<String>("input")
        .expect(&ok_compiler_i_made_it_a_string);

    let asm_out = matches.get_one::<String>("asm").unwrap();
    let exe_out = matches.get_one::<String>("exe").unwrap();
    let feeling_lazy = matches.get_flag("feelinglazy");

    let source: String = fs::read_to_string(input)?;
    let tokens = lexer::tokenize(&source)?;
    let ast = parser::parse(&tokens)?;
    let asm = emitter::emit(&ast);
    fs::write(asm_out, &asm)?;

    println!(
        "{}assembly written to {}{}",
        GREEN_ANSII, asm_out, RESET_ANSII
    );

    if feeling_lazy {
        // check for nasm
        if !command_exists("nasm") {
            eprintln!(
                "{}nasm (compiler-compiler) not found in PATH. install nasm or run without --feelinglazy{}",
                RED_ANSII, RESET_ANSII
            );
            exit(1);
        }
        // check for ld
        if !command_exists("ld") {
            eprintln!(
                "{}ld (linker) not found in PATH. install binutils or run without --feelinglazy{}",
                RED_ANSII, RESET_ANSII
            );
            exit(1);
        }
        // run nasm
        println!("running: nasm -f elf64 {} -o out.o", asm_out);
        let nasm_status = Command::new("nasm")
            .args(["-f", "elf64", asm_out, "-o", "out.o"])
            .status()?;
        if !nasm_status.success() {
            eprintln!("{}nasm failed{}", RED_ANSII, RESET_ANSII);
            exit(1);
        }
        // run ld
        println!("running: ld out.o -o {}", exe_out);
        let ld_status = Command::new("ld").args(["out.o", "-o", exe_out]).status()?;
        if !ld_status.success() {
            eprintln!("{}ld failed{}", RED_ANSII, RESET_ANSII);
            exit(1);
        }

        println!(
            "{}executable created: {}{}",
            GREEN_ANSII, exe_out, RESET_ANSII
        );
        println!("run with: ./{}", exe_out);
    } else {
        println!(
            "to assemble and link, run:\n\
        nasm -f elf64 {} -o out.o\n\
        ld out.o -o executable_name\n\
        ./executable_name",
            asm_out
        );
    }

    Ok(())
}

fn command_exists(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}
