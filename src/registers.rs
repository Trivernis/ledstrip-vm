use crate::ledstrip_controller::{LedStripController, StateStripCommand};
use std::cell::RefCell;
use std::rc::Rc;

pub const RCS: u8 = 0x01;
pub const RCR: u8 = 0x02;
pub const RCG: u8 = 0x03;
pub const RCB: u8 = 0x04;
pub const RGD: u8 = 0x05;
pub const RGP: u8 = 0x06;
pub const RGI: u8 = 0x07;
pub const RGO: u8 = 0x08;
pub const RGL: u8 = 0x09;

/// Maps a register name to the bytecode value
pub fn get_register_code_by_name(name: &str) -> Option<u8> {
    let item = [
        ("rcs", RCS),
        ("rcr", RCR),
        ("rcg", RCG),
        ("rcb", RCB),
        ("rgd", RGD),
        ("rgp", RGP),
        ("rgi", RGI),
        ("rgo", RGO),
        ("rgl", RGL),
    ]
    .iter()
    .find(|(reg, _)| *reg == name);

    if let Some(item) = item {
        Some((*item).1)
    } else {
        println!("Unknown register: {}", name);
        None
    }
}

pub trait Register<T> {
    fn set(&mut self, value: T);
    fn get(&self) -> T;
}

#[derive(Debug, Clone)]
pub struct Rcs {
    value: bool,
    strip_controller: Rc<RefCell<LedStripController>>,
}

#[derive(Debug, Clone)]
pub struct Rcr {
    value: u8,
}

#[derive(Debug, Clone)]
pub struct Rcg {
    value: u8,
}

#[derive(Debug, Clone)]
pub struct Rcb {
    value: u8,
}

#[derive(Debug, Clone)]
pub struct Rgd {
    value: u32,
}

#[derive(Debug, Clone)]
pub struct Rgp {
    value: u32,
}

#[derive(Debug, Clone)]
pub struct Rgi {
    value: u32,
}

#[derive(Debug, Clone)]
pub struct Rgo {
    value: u32,
}

#[derive(Debug, Clone)]
pub struct Rgl {
    value: u32,
}

impl Rcs {
    pub fn new(strip_controller: Rc<RefCell<LedStripController>>) -> Self {
        Self {
            value: false,
            strip_controller,
        }
    }
}

impl Rcr {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Rcg {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Rcb {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Rgd {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Rgp {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Rgi {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Rgo {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Rgl {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Register<bool> for Rcs {
    fn set(&mut self, value: bool) {
        self.value = value;
        let state = if value {
            StateStripCommand::On
        } else {
            StateStripCommand::Off
        };
        self.strip_controller
            .borrow_mut()
            .set_state(state)
            .expect("failed to set state");
    }

    fn get(&self) -> bool {
        self.value
    }
}

impl Register<u8> for Rcr {
    fn set(&mut self, value: u8) {
        self.value = value;
    }

    fn get(&self) -> u8 {
        self.value
    }
}

impl Register<u8> for Rcg {
    fn set(&mut self, value: u8) {
        self.value = value;
    }

    fn get(&self) -> u8 {
        self.value
    }
}

impl Register<u8> for Rcb {
    fn set(&mut self, value: u8) {
        self.value = value;
    }

    fn get(&self) -> u8 {
        self.value
    }
}

impl Register<u32> for Rgd {
    fn set(&mut self, value: u32) {
        self.value = value
    }

    fn get(&self) -> u32 {
        self.value
    }
}

impl Register<u32> for Rgp {
    fn set(&mut self, value: u32) {
        self.value = value;
    }

    fn get(&self) -> u32 {
        self.value
    }
}

impl Register<u32> for Rgi {
    fn set(&mut self, value: u32) {
        self.value = value;
    }

    fn get(&self) -> u32 {
        self.value
    }
}

impl Register<u32> for Rgo {
    fn set(&mut self, value: u32) {
        self.value = value;
    }

    fn get(&self) -> u32 {
        self.value
    }
}

impl Register<u32> for Rgl {
    fn set(&mut self, value: u32) {
        self.value = value;
    }

    fn get(&self) -> u32 {
        self.value
    }
}
