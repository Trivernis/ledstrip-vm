use crate::ledstrip_controller::{LedStripController, StateStripCommand};
use crate::registers::{
    Rcb, Rcg, Rcr, Rcs, Register, Rgd, Rgi, Rgl, Rgo, Rgp, RCB, RCG, RCR, RGD, RGI, RGL, RGO, RGP,
};
use crate::tokens::{
    AddToken, AndToken, ClearToken, CmdToken, CopyToken, DebugToken, DivToken, ExitToken,
    FromBytecode, GotoToken, JeToken, JgToken, JlToken, LabelToken, LoadToken, LshToken, ModToken,
    MulToken, NotToken, NrtToken, OrToken, PauseToken, PowToken, PrintToken, RshToken, SendToken,
    SetToken, SubToken, Token, WriteToken, XorToken, T_ADD, T_AND, T_CLEAR, T_CMD, T_COPY, T_DEBUG,
    T_DIV, T_EXIT, T_GOTO, T_JE, T_JG, T_JL, T_LABEL, T_LOAD, T_LSH, T_MOD, T_MUL, T_NOT, T_NRT,
    T_OR, T_PAUSE, T_POW, T_PRINT, T_RSH, T_SEND, T_SET, T_SUB, T_WRITE, T_XOR,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

#[derive(Clone)]
pub struct Runtime {
    pub rcs: Rcs,
    pub rcr: Rcr,
    pub rcg: Rcg,
    pub rcb: Rcb,
    pub rgd: Rgd,
    pub rgp: Rgp,
    pub rgi: Rgi,
    pub rgo: Rgo,
    pub rgl: Rgl,
    pub memory: HashMap<u32, u32>,
    text: Rc<RefCell<Vec<Box<dyn Token>>>>,
    pub labels: HashMap<u32, usize>,
    pub strip_controller: Rc<RefCell<LedStripController>>,
    exit: Option<u8>,
    current_index: usize,
    debug: bool,
}

impl Runtime {
    pub fn new(ip: &str, port: usize) -> Self {
        let controller = LedStripController::new(ip, port)
            .expect("failed to establish a connection to the led strip");
        let controller = Rc::new(RefCell::new(controller));

        Self {
            rcs: Rcs::new(controller.clone()),
            rcr: Rcr::new(),
            rcg: Rcg::new(),
            rcb: Rcb::new(),
            rgd: Rgd::new(),
            rgp: Rgp::new(),
            rgi: Rgi::new(),
            rgo: Rgo::new(),
            rgl: Rgl::new(),
            memory: HashMap::new(),
            text: Rc::new(RefCell::new(Vec::new())),
            labels: HashMap::new(),
            strip_controller: controller,
            exit: None,
            current_index: 0,
            debug: false,
        }
    }

    /// Sets debug to the specified value
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Parses a vector containing the bytecode into a vector of tokens
    /// that can be executed
    pub fn parse_bytecode(&mut self, bytecode: Vec<u8>) {
        let mut code_iter = bytecode.iter();
        let mut text = self.text.borrow_mut();
        let mut index = 0;

        while let Some(instruction) = code_iter.next() {
            match *instruction {
                T_EXIT => text.push(Box::new(ExitToken::from_bytecode(&[
                    instruction,
                    code_iter.next().unwrap(),
                ]))),
                T_SET => text.push(Box::new(SetToken::from_bytecode(&[
                    instruction,
                    code_iter.next().unwrap(),
                    code_iter.next().unwrap(),
                ]))),
                T_COPY => text.push(Box::new(CopyToken::from_bytecode(&[
                    instruction,
                    code_iter.next().unwrap(),
                    code_iter.next().unwrap(),
                ]))),
                T_LOAD => text.push(Box::new(LoadToken)),
                T_CLEAR => text.push(Box::new(ClearToken::from_bytecode(&[
                    instruction,
                    code_iter.next().unwrap(),
                ]))),
                T_WRITE => text.push(Box::new(WriteToken)),
                T_LABEL => {
                    let token = LabelToken::from_bytecode(&[
                        instruction,
                        code_iter.next().unwrap(),
                        code_iter.next().unwrap(),
                        code_iter.next().unwrap(),
                        code_iter.next().unwrap(),
                    ]);
                    self.labels.insert(token.value, index);
                    text.push(Box::new(token));
                }
                T_GOTO => text.push(Box::new(GotoToken)),
                T_DEBUG => text.push(Box::new(DebugToken)),
                T_PRINT => text.push(Box::new(PrintToken::from_bytecode(&[
                    instruction,
                    code_iter.next().unwrap(),
                ]))),
                T_ADD => text.push(Box::new(AddToken)),
                T_SUB => text.push(Box::new(SubToken)),
                T_MUL => text.push(Box::new(MulToken)),
                T_DIV => text.push(Box::new(DivToken)),
                T_MOD => text.push(Box::new(ModToken)),
                T_LSH => text.push(Box::new(LshToken)),
                T_RSH => text.push(Box::new(RshToken)),
                T_AND => text.push(Box::new(AndToken)),
                T_OR => text.push(Box::new(OrToken)),
                T_NOT => text.push(Box::new(NotToken)),
                T_XOR => text.push(Box::new(XorToken)),
                T_POW => text.push(Box::new(PowToken)),
                T_NRT => text.push(Box::new(NrtToken)),
                T_JG => text.push(Box::new(JgToken)),
                T_JL => text.push(Box::new(JlToken)),
                T_JE => text.push(Box::new(JeToken)),
                T_PAUSE => text.push(Box::new(PauseToken)),
                T_CMD => text.push(Box::new(CmdToken)),
                T_SEND => text.push(Box::new(SendToken)),
                _ => panic!("unknown instruction {}", instruction),
            };
            index += 1;
        }
    }

    /// Executes the text stored in the runtime
    pub fn run(&mut self) -> io::Result<u8> {
        let text_ref = self.text.clone();
        let text = text_ref.borrow_mut();
        while self.current_index < text.len() {
            let token = text.get(self.current_index).unwrap();
            token.invoke(self)?;
            if self.debug {
                println!("{:?}", token);
            }

            if let Some(code) = self.exit {
                self.strip_controller
                    .borrow_mut()
                    .set_state(StateStripCommand::Off)
                    .unwrap();
                return Ok(code);
            }

            self.current_index += 1;
        }

        self.strip_controller
            .borrow_mut()
            .set_state(StateStripCommand::Off)
            .unwrap();
        Ok(0)
    }

    /// Exists the program with a specified error code
    pub fn exit(&mut self, code: u8) {
        self.exit = Some(code);
    }

    /// Returns the 1byte register referenced by the code
    pub fn get_1byte_register(&mut self, code: u8) -> Option<Box<&mut dyn Register<u8>>> {
        match code {
            RCR => Some(Box::new(&mut self.rcr)),
            RCG => Some(Box::new(&mut self.rcg)),
            RCB => Some(Box::new(&mut self.rcb)),
            _ => None,
        }
    }

    /// Returns the 4byte register referenced by the code
    pub fn get_4byte_register(&mut self, code: u8) -> Option<Box<&mut dyn Register<u32>>> {
        match code {
            RGD => Some(Box::new(&mut self.rgd)),
            RGP => Some(Box::new(&mut self.rgp)),
            RGI => Some(Box::new(&mut self.rgi)),
            RGO => Some(Box::new(&mut self.rgo)),
            RGL => Some(Box::new(&mut self.rgl)),
            _ => None,
        }
    }

    /// Creates a new label at the current position
    pub fn create_label(&mut self, id: u32) {
        self.labels.insert(id, self.current_index);
    }

    /// Jumps to a specified label
    pub fn jump(&mut self, label: u32) -> io::Result<()> {
        self.current_index = *self
            .labels
            .get(&label)
            .expect(&format!("The label {} does not exist", label));

        Ok(())
    }
}
