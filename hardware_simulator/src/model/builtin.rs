use crate::model::Chip;
use crate::parser::Interface;
use crate::BusRange;
use std::iter::once;

pub fn get_builtin(name: &str) -> Option<Box<dyn Chip>> {
    match name {
        "Nand" => Some(Box::new(Nand)),
        _ => None,
    }
}

#[derive(Clone)]
struct Nand;
impl Chip for Nand {
    fn interface(&self) -> Interface {
        Interface {
            com_in: [
                ("a".to_string(), BusRange { start: 0, end: 0 }),
                ("b".to_string(), BusRange { start: 1, end: 1 }),
            ]
            .into_iter()
            .collect(),
            com_out: once(("out".to_string(), BusRange { start: 1, end: 1 })).collect(),
            seq_in: Default::default(),
            seq_out: Default::default(),
        }
    }

    fn clock(&mut self) {
        // nothing
    }
    fn eval(&mut self, pins: &[bool]) -> Vec<bool> {
        vec![!(pins[0] && pins[1])]
    }
}
