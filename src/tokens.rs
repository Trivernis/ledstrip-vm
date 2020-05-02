const T_NOP: u8 = 0x00;
const T_EXIT: u8 = 0x01;
const T_SET: u8 = 0x02;
const T_COPY: u8 = 0x03;
const T_LOAD: u8 = 0x04;
const T_CLEAR: u8 = 0x05;
const T_WRITE: u8 = 0x06;
const T_LABEL: u8 = 0x07;
const T_GOTO: u8 = 0x08;
const T_ADD: u8 = 0x10;
const T_SUB: u8 = 0x11;
const T_MUL: u8 = 0x12;
const T_DIV: u8 = 0x13;
const T_MOD: u8 = 0x14;
const T_LSH: u8 = 0x15;
const T_RSH: u8 = 0x16;
const T_JG: u8 = 0x20;
const T_JL: u8 = 0x21;
const T_JE: u8 = 0x22;
const T_PAUSE: u8 = 0xF0;
const T_CMD: u8 = 0xF1;

pub trait Token {
    fn to_bytecode(&self) -> Vec<u8>;
}

pub trait FromBytecode {
    fn from_bytecode(code: &[u8]) -> Self;
}

pub struct NopToken;

impl Token for NopToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_NOP]
    }
}

impl FromBytecode for NopToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct ExitToken {
    pub register: u8,
}

impl Token for ExitToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_EXIT, self.register]
    }
}

impl FromBytecode for ExitToken {
    fn from_bytecode(code: &[u8]) -> Self {
        Self { register: code[1] }
    }
}

pub struct SetToken {
    pub value: u8,
    pub register: u8,
}

impl Token for SetToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_SET, self.value, self.register]
    }
}

impl FromBytecode for SetToken {
    fn from_bytecode(code: &[u8]) -> Self {
        Self {
            value: code[1],
            register: code[2],
        }
    }
}

pub struct CopyToken {
    pub register_1: u8,
    pub register_2: u8,
}

impl Token for CopyToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_COPY, self.register_1, self.register_2]
    }
}

impl FromBytecode for CopyToken {
    fn from_bytecode(code: &[u8]) -> Self {
        Self {
            register_1: code[1],
            register_2: code[2],
        }
    }
}

pub struct LoadToken;

impl Token for LoadToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_LOAD]
    }
}

impl FromBytecode for LoadToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct ClearToken {
    pub register: u8,
}

impl Token for ClearToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_CLEAR, self.register]
    }
}

impl FromBytecode for ClearToken {
    fn from_bytecode(code: &[u8]) -> Self {
        Self { register: code[1] }
    }
}

pub struct WriteToken;

impl Token for WriteToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_WRITE]
    }
}

impl FromBytecode for WriteToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct LabelToken;

impl Token for LabelToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_LABEL]
    }
}

impl FromBytecode for LabelToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct GotoToken;

impl Token for GotoToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_GOTO]
    }
}

impl FromBytecode for GotoToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct AddToken;

impl Token for AddToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_ADD]
    }
}

impl FromBytecode for AddToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct SubToken;

impl Token for SubToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_SUB]
    }
}

impl FromBytecode for SubToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct MulToken;

impl Token for MulToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_MUL]
    }
}

impl FromBytecode for MulToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct DivToken;

impl Token for DivToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_DIV]
    }
}

impl FromBytecode for DivToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct ModToken;

impl Token for ModToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_MOD]
    }
}

impl FromBytecode for ModToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct LshToken;

impl Token for LshToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_LSH]
    }
}

impl FromBytecode for LshToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct RshToken;

impl Token for RshToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_RSH]
    }
}

impl FromBytecode for RshToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct JgToken;

impl Token for JgToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_JG]
    }
}

impl FromBytecode for JgToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct JlToken;

impl Token for JlToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_JL]
    }
}

impl FromBytecode for JlToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct JeToken;

impl Token for JeToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_JE]
    }
}

impl FromBytecode for JeToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct PauseToken;

impl Token for PauseToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_PAUSE]
    }
}

impl FromBytecode for PauseToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}

pub struct CmdToken;

impl Token for CmdToken {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![T_CMD]
    }
}

impl FromBytecode for CmdToken {
    fn from_bytecode(_: &[u8]) -> Self {
        Self
    }
}