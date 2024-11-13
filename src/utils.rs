use core::panic;

use crate::lexer::CompilerMode;

pub fn map_compiler_mode(mode: String) -> CompilerMode {
    match mode.as_str() {
        "debug"  => CompilerMode::Debug,
        "normal" => CompilerMode::Normal,
        _        => panic!("Invalid mode: {}", mode.as_str()),
    }
}
