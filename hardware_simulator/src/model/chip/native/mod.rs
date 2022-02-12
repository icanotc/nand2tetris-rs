pub mod build;
mod edge_set;

use crate::bus_range::BusRange;
use crate::model::chip::{Chip, ChipObject};
use crate::model::parser::Interface;
use petgraph::Graph;

#[derive(Clone)]
pub enum ConnEdge {
    Combinatorial {
        in_range: BusRange,
        out_range: BusRange,
        buf: Vec<bool>,
    },
    Sequential {
        in_range: BusRange,
        out_range: BusRange,
        waiting: Vec<bool>,
        buf: Vec<bool>,
    },
}

#[derive(Clone)]
pub struct NativeChip {
    pub conn_graph: Graph<Chip, ConnEdge>,
    pub interface: Interface,
}

impl ChipObject for NativeChip {
    fn interface(&self) -> Interface {
        self.interface.clone()
    }

    fn clock(&mut self) {
        todo!()
    }

    fn eval(&mut self, _: &[bool]) -> Vec<bool> {
        todo!()
    }

    fn chip_clone(&self) -> Box<dyn ChipObject> {
        Box::new(self.clone())
    }
}
