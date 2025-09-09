use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Register {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Imm {
    Reg(Register),
    Const(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RegisterOrPC {
    Reg(Register),
    PC(u64),
}

pub type Addr = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CompareOp {
    Leq,
    Lt,
    Eq,
    Neq,
    Gt,
    Geq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BranchChoice {
    TrueBranch,
    FalseBranch,
    NeitherBranch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Asm {
    Add {
        /// left-hand-register, right-hand-register, and result-register
        lhr: Imm,
        rhr: Imm,
        rr: Register,
    },
    Sub {
        lhr: Imm,
        rhr: Imm,
        rr: Register,
    },
    Mul {
        lhr: Imm,
        rhr: Imm,
        rr: Register,
    },
    Jump {
        addr: RegisterOrPC,
    },
    Compare {
        lhr: Imm,
        rhr: Imm,
        op: CompareOp,
    },
    JumpConditional {
        true_addr: RegisterOrPC,
        false_addr: RegisterOrPC,
        more_likely_branch: BranchChoice,
    },
    MemRead {
        addr: Addr,
        dest: Register,
    },
    MemWrite {
        addr: Addr,
        src: Imm,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundledAsm {
    pub label: String,
    pub instrs: Vec<Asm>,
}
pub type BundledProgram = Vec<BundledAsm>;
