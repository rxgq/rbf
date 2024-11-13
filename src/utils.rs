use core::panic;

use crate::lexer::CompilerMode;

pub fn map_compiler_mode(mode: String) -> CompilerMode {
    match mode.as_str() {
        "-d" => CompilerMode::Debug,
        "-n" => CompilerMode::Normal,
        _    => panic!("Invalid mode: {}", mode.as_str()),
    }
}
