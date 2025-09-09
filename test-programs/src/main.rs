use std::fs;
use types::Register::*;
use types::{Addr, Asm, BundledAsm, BundledProgram, CompareOp, Imm, Register, RegisterOrPC};

fn to_file(program: BundledProgram, path: String) {
    fs::write(path, serde_json::to_string(&program).unwrap()).unwrap();
}

fn singleton(asm: Asm) -> BundledAsm {
    BundledAsm {
        label: "singleton".to_string(),
        instrs: vec![asm],
    }
}

fn bundle(instrs: Vec<Asm>, label: &str) -> BundledAsm {
    BundledAsm {
        label: label.to_string(),
        instrs,
    }
}

fn r(r: Register) -> Imm {
    Imm::Reg(r)
}

fn v(v: u64) -> Imm {
    Imm::Const(v)
}

fn add(a: Imm, b: Imm, c: Register) -> Asm {
    Asm::Add {
        lhr: a,
        rhr: b,
        rr: c,
    }
}
fn write_linear_adds() {
    let program = vec![
        singleton(add(v(1), v(1), R1)),
        singleton(add(v(2), v(2), R2)),
        singleton(add(r(R1), r(R2), R3)),
    ];
    to_file(program, "./tests/linear_adds.lpl".to_string());
}

fn write_bundled_adds() {
    let program = vec![
        bundle(vec![add(v(1), v(1), R1), add(v(2), v(2), R2)], "adds"),
        singleton(add(r(R1), r(R2), R3)),
    ];
    to_file(program, "./tests/bundled_adds.lpl".to_string());
}

fn main() {
    write_linear_adds();
    write_bundled_adds();
    // write_fun_math_linear();
    // write_fun_math_bundled();
}
