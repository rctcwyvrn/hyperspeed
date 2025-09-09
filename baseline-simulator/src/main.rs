use clap::Parser;
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use types::{Addr, Asm, BundledProgram, CompareOp, Imm, Register, RegisterOrPC};

#[derive(Debug)]
struct SimulatorState {
    mem: Vec<u8>,
    reg_file: HashMap<Register, u64>,
    last_compare: bool,
    pc: usize,
}

impl SimulatorState {
    fn new() -> SimulatorState {
        SimulatorState {
            mem: Vec::new(),
            reg_file: HashMap::new(),
            // CR-someday lilin: this should probably be an option None but im lazy
            last_compare: false,
            pc: 0,
        }
    }

    fn reg_read(&self, reg: Register) -> u64 {
        match self.reg_file.get(&reg) {
            None => panic!(
                "Read uninitialized register, undefined behavior (reg = {:?}, regfile = {:?})",
                reg, self.reg_file
            ),
            Some(v) => v.clone(),
        }
    }

    fn reg_write(&mut self, reg: Register, val: u64) {
        self.reg_file.insert(reg, val);
    }

    fn mem_read_u8(&self, offset: usize) -> u8 {
        match self.mem.get(offset) {
            None => panic!(
                "Read uninitialized memory, undefined behavior (addr = {:?}, memory= {:?})",
                offset, self.mem
            ),
            Some(v) => v.clone(),
        }
    }

    fn mem_read_le(&self, addr: Addr) -> u64 {
        let base = addr as usize;
        (self.mem_read_u8(base) as u64)
            + ((self.mem_read_u8(base + 8) as u64) << 8)
            + ((self.mem_read_u8(base + 16) as u64) << 16)
            + ((self.mem_read_u8(base + 32) as u64) << 32)
            + ((self.mem_read_u8(base + 40) as u64) << 40)
            + ((self.mem_read_u8(base + 48) as u64) << 48)
            + ((self.mem_read_u8(base + 56) as u64) << 56)
    }

    fn mem_write_u8(&mut self, offset: usize, val: u8) {
        if self.mem.len() <= offset {
            let mut new_capacity = Vec::new();
            for _ in 0..(offset - self.mem.len() + 1 + 8) {
                new_capacity.push(0)
            }
            self.mem.append(&mut new_capacity);
        }
        let ptr = self.mem.get_mut(offset).unwrap();
        *ptr = val
    }

    fn mem_write_le(&mut self, addr: Addr, val: u64) {
        let base = addr as usize;
        self.mem_write_u8(base, (val & 0xFF) as u8);
        self.mem_write_u8(base + 8, ((val >> 8) & 0xFF) as u8);
        self.mem_write_u8(base + 16, ((val >> 16) & 0xFF) as u8);
        self.mem_write_u8(base + 32, ((val >> 24) & 0xFF) as u8);
        self.mem_write_u8(base + 40, ((val >> 32) & 0xFF) as u8);
        self.mem_write_u8(base + 48, ((val >> 48) & 0xFF) as u8);
        self.mem_write_u8(base + 56, ((val >> 56) & 0xFF) as u8);
    }
}

fn read_imm(imm: Imm, state: &SimulatorState) -> u64 {
    match imm {
        Imm::Reg(reg) => state.reg_read(reg),
        Imm::Const(val) => val,
    }
}

fn interpret_asm(instr: Asm, state: &mut SimulatorState) {
    match instr {
        Asm::Add { lhr, rhr, rr } => {
            state.reg_write(rr, read_imm(lhr, state) + read_imm(rhr, state));
        }
        Asm::Sub { lhr, rhr, rr } => {
            state.reg_write(rr, read_imm(lhr, state) - read_imm(rhr, state));
        }
        Asm::Mul { lhr, rhr, rr } => {
            state.reg_write(rr, read_imm(lhr, state) * read_imm(rhr, state));
        }
        Asm::Jump { addr } => {
            state.pc = match addr {
                RegisterOrPC::Reg(reg) => state.reg_read(reg) as usize,
                RegisterOrPC::PC(val) => val as usize,
            }
        }
        Asm::Compare { lhr, rhr, op } => {
            let lhs = read_imm(lhr, state);
            let rhs = read_imm(rhr, state);
            state.last_compare = match op {
                CompareOp::Leq => lhs <= rhs,
                CompareOp::Lt => lhs < rhs,
                CompareOp::Eq => lhs == rhs,
                CompareOp::Neq => lhs != rhs,
                CompareOp::Gt => lhs > rhs,
                CompareOp::Geq => lhs >= rhs,
            };
        }
        Asm::JumpConditional {
            true_addr,
            false_addr,
            more_likely_branch: _,
        } => {
            let addr = if state.last_compare {
                true_addr
            } else {
                false_addr
            };
            state.pc = match addr {
                RegisterOrPC::Reg(reg) => state.reg_read(reg) as usize,
                RegisterOrPC::PC(val) => val as usize,
            }
        }
        Asm::MemRead { addr, dest } => {
            state.reg_write(dest, state.mem_read_le(addr));
        }
        Asm::MemWrite { addr, src } => {
            state.mem_write_le(addr, read_imm(src, state));
        }
    }
}

// CR-soon lilin: there should be a static check to make sure that we arent emitting multiple jumps
// in one bundle
fn execute(program: BundledProgram) {
    dbg!("[+] Initializing");
    let mut state = SimulatorState::new();
    while state.pc < program.len() {
        let pc_before = state.pc;
        let bundle = &program[state.pc];
        dbg!("===");
        dbg!("[~] Executing bundle", &bundle.label);
        dbg!("[-] State before", &state);
        let mut instrs = bundle.instrs.clone();
        instrs.shuffle(&mut rng());
        for instr in instrs {
            interpret_asm(instr, &mut state);
        }

        // is this jank? pretty jank
        if state.pc == pc_before {
            state.pc += 1
        }
        dbg!("[-] State after", &state);
        dbg!("---");
    }
    println!("Final state");
    println!("{:?}", &state)
}

#[derive(Parser, Debug)]
struct Args {
    /// Filename
    #[arg(long)]
    program: String,
}

fn main() {
    let args = Args::parse();
    let path = std::path::Path::new(&args.program);
    let program = std::fs::read_to_string(path).unwrap();
    let program: BundledProgram = serde_json::from_str(&program).unwrap();
    execute(program);
}
