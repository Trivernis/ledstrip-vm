use crate::ledstrip_controller::{LedStripController, StateStripCommand};
use crate::registers::{
    Rcb, Rcg, Rcr, Rcs, Register, Rgd, Rgi, Rgl, Rgo, Rgp, RCB, RCG, RCR, RGD, RGI, RGL, RGO, RGP,
};
use crate::tokens::{
    AddToken, ClearToken, CmdToken, CopyToken, DivToken, ExitToken, FromBytecode, GotoToken,
    JeToken, JgToken, JlToken, LabelToken, LoadToken, LshToken, ModToken, MulToken, PauseToken,
    RshToken, SendToken, SetToken, SubToken, Token, WriteToken, T_ADD, T_CLEAR, T_CMD, T_COPY,
    T_DIV, T_EXIT, T_GOTO, T_JE, T_JG, T_JL, T_LABEL, T_LOAD, T_LSH, T_MOD, T_MUL, T_PAUSE, T_RSH,
    T_SEND, T_SET, T_SUB, T_WRITE,
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
    labels: HashMap<u32, usize>,
    pub strip_controller: Rc<RefCell<LedStripController>>,
    exit: Option<u8>,
    current_index: usize,
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
        }
    }

    /// Parses a vector containing the bytecode into a vector of tokens
    /// that can be executed
    pub fn parse_bytecode(&mut self, bytecode: Vec<u8>) {
        let mut code_iter = bytecode.iter();

        while let Some(instruction) = code_iter.next() {
            match *instruction {
                T_EXIT => self
                    .text
                    .borrow_mut()
                    .push(Box::new(ExitToken::from_bytecode(&[
                        instruction,
                        code_iter.next().unwrap(),
                    ]))),
                T_SET => self
                    .text
                    .borrow_mut()
                    .push(Box::new(SetToken::from_bytecode(&[
                        instruction,
                        code_iter.next().unwrap(),
                        code_iter.next().unwrap(),
                    ]))),
                T_COPY => self
                    .text
                    .borrow_mut()
                    .push(Box::new(CopyToken::from_bytecode(&[
                        instruction,
                        code_iter.next().unwrap(),
                        code_iter.next().unwrap(),
                    ]))),
                T_LOAD => self.text.borrow_mut().push(Box::new(LoadToken)),
                T_CLEAR => self
                    .text
                    .borrow_mut()
                    .push(Box::new(ClearToken::from_bytecode(&[
                        instruction,
                        code_iter.next().unwrap(),
                    ]))),
                T_WRITE => self.text.borrow_mut().push(Box::new(WriteToken)),
                T_LABEL => self.text.borrow_mut().push(Box::new(LabelToken)),
                T_GOTO => self.text.borrow_mut().push(Box::new(GotoToken)),
                T_ADD => self.text.borrow_mut().push(Box::new(AddToken)),
                T_SUB => self.text.borrow_mut().push(Box::new(SubToken)),
                T_MUL => self.text.borrow_mut().push(Box::new(MulToken)),
                T_DIV => self.text.borrow_mut().push(Box::new(DivToken)),
                T_MOD => self.text.borrow_mut().push(Box::new(ModToken)),
                T_LSH => self.text.borrow_mut().push(Box::new(LshToken)),
                T_RSH => self.text.borrow_mut().push(Box::new(RshToken)),
                T_JG => self.text.borrow_mut().push(Box::new(JgToken)),
                T_JL => self.text.borrow_mut().push(Box::new(JlToken)),
                T_JE => self.text.borrow_mut().push(Box::new(JeToken)),
                T_PAUSE => self.text.borrow_mut().push(Box::new(PauseToken)),
                T_CMD => self.text.borrow_mut().push(Box::new(CmdToken)),
                T_SEND => self.text.borrow_mut().push(Box::new(SendToken)),
                _ => panic!("unknown instruction {}", instruction),
            };
        }
    }

    /// Executes the text stored in the runtime
    pub fn run(&mut self) -> io::Result<u8> {
        let text_ref = self.text.clone();
        let text = text_ref.borrow_mut();
        while self.current_index < text.len() {
            let token = text.get(self.current_index).unwrap();
            token.invoke(self)?;

            if let Some(code) = self.exit {
                self.strip_controller
                    .borrow_mut()
                    .set_state(StateStripCommand::Off)
                    .unwrap();
                return Ok(0);
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
