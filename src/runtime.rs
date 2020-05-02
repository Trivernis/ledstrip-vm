use crate::ledstrip_controller::LedStripController;
use crate::ledstrip_controller::ProgramStripCommand::RedGradual;
use crate::registers::{
    Rcb, Rcg, Rcr, Rcs, Register, Rgd, Rgi, Rgl, Rgo, Rgp, RCB, RCG, RCR, RCS, RGD, RGI, RGL, RGO,
    RGP,
};
use crate::tokens::Token;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

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
    text: Vec<Box<dyn Token>>,
    labels: HashMap<u32, u128>,
    strip_controller: Rc<RefCell<LedStripController>>,
    exit: Option<u8>,
    current_index: u128,
}

impl Runtime {
    pub fn new(ip: &str, port: usize) -> Self {
        let controller = LedStripController::new(ip, port)
            .expect("failed to establish a connection to the led strip");
        let mut controller = Rc::new(RefCell::new(controller));

        Self {
            rcs: Rcs::new(controller.clone()),
            rcr: Rcr::new(controller.clone()),
            rcg: Rcg::new(controller.clone()),
            rcb: Rcb::new(controller.clone()),
            rgd: Rgd::new(),
            rgp: Rgp::new(),
            rgi: Rgi::new(),
            rgo: Rgo::new(),
            rgl: Rgl::new(),
            memory: HashMap::new(),
            text: Vec::new(),
            labels: HashMap::new(),
            strip_controller: controller,
            exit: None,
            current_index: 0,
        }
    }

    pub fn exit(&mut self, code: u8) {
        self.exit = Some(code);
    }

    /*
    pub fn get_register<T>(&mut self, code: u8) -> Option<&mut impl Register<T>> {
        match code {
            RCS => Some(&mut self.rcs),
            RCR => Some(&mut self.rcr),
            RCG => Some(&mut self.rcg),
            RCB => Some(&mut self.rcb),
            RGD => Some(&mut self.rgd),
            RGP => Some(&mut self.rgp),
            RGI => Some(&mut self.rgi),
            RGO => Some(&mut self.rgo),
            RGL => Some(&mut self.rgl),
            _ => None,
        }
    }*/

    pub fn get_1byte_register(&mut self, code: u8) -> Option<Box<&mut dyn Register<u8>>> {
        match code {
            RCR => Some(Box::new(&mut self.rcr)),
            RCG => Some(Box::new(&mut self.rcg)),
            RCB => Some(Box::new(&mut self.rcb)),
            _ => None,
        }
    }

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
        self.labels.insert(id, self.current_index + 1);
    }

    pub fn jump(&mut self, label: u32) -> io::Result<()> {
        self.current_index = *self
            .labels
            .get(&label)
            .expect(&format!("The label {} does not exist", label));

        Ok(())
    }
}
