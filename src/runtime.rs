use crate::ledstrip_controller::LedStripController;
use crate::registers::{Rcb, Rcg, Rcr, Rcs, Rgd, Rgi, Rgl, Rgo, Rgp};
use crate::tokens::Token;
use std::cell::RefCell;
use std::collections::HashMap;
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
        }
    }
}
