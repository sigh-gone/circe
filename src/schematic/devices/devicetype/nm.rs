//! device definition for mosfets (MXXXX)
// .model n1 nmos level=54 version=4.8.2
// .model p1 pmos level=54 version=4.8.2
// port order: d g s b
// followed by model name

// MXXXXXXX nd ng ns nb mname <m = val > <l = val > <w = val >
// + < ad = val > < as = val > < pd = val > < ps = val > < nrd = val >
// + < nrs = val > <off > < ic = vds , vgs , vbs > < temp =t >

// need a way to define models once in the netlist

use super::super::params;
use super::{Graphics, Port};
use crate::schematic::interactable::Interactable;
use crate::transforms::{SSBox, SSPoint, VSPoint};
use lazy_static::lazy_static;

pub const ID_PREFIX: &str = "M";

lazy_static! {
    static ref DEFAULT_GRAPHICS: Graphics = Graphics {
        pts: vec![
            vec![VSPoint::new(0.00, 1.50), VSPoint::new(0.00, -1.50),],
            vec![VSPoint::new(1.00, -1.00), VSPoint::new(2.00, -1.50),],
            vec![VSPoint::new(2.00, 3.00), VSPoint::new(2.00, 1.50),],
            vec![VSPoint::new(2.00, -1.50), VSPoint::new(2.00, -3.00),],
            vec![VSPoint::new(0.00, -1.50), VSPoint::new(2.00, -1.50),],
            vec![VSPoint::new(2.00, 1.50), VSPoint::new(0.00, 1.50),],
            vec![VSPoint::new(-0.50, 0.00), VSPoint::new(-2.00, 0.00),],
            vec![VSPoint::new(0.00, 0.00), VSPoint::new(2.00, 0.00),],
            vec![VSPoint::new(-0.50, 1.50), VSPoint::new(-0.50, -1.50),],
            vec![VSPoint::new(2.00, -1.50), VSPoint::new(1.00, -2.00),],
        ],
        cirarcs: vec![
        ],
        ports: vec![
            Port {
                name: "0".to_string(),
                offset: SSPoint::new(2, 3),
                interactable: Interactable::default()
            },
            Port {
                name: "1".to_string(),
                offset: SSPoint::new(-2, 0),
                interactable: Interactable::default()
            },
            Port {
                name: "2".to_string(),
                offset: SSPoint::new(2, -3),
                interactable: Interactable::default()
            },
            Port {
                name: "3".to_string(),
                offset: SSPoint::new(2, 0),
                interactable: Interactable::default()
            },
        ],
        bounds: SSBox::new(SSPoint::new(-2, -3), SSPoint::new(2, 3)),
    };
 }

#[derive(Debug, Clone)]
pub enum ParamM {
    Raw(params::Raw),
}
impl Default for ParamM {
    fn default() -> Self {
        ParamM::Raw(params::Raw::new(String::from("")))
    }
}
impl ParamM {
    pub fn summary(&self) -> String {
        match self {
            ParamM::Raw(s) => {
                format!("mosn {}", s.raw.clone())
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct M {
    pub params: ParamM,
    pub graphics: &'static Graphics,
}
impl M {
    pub fn new() -> M {
        M {
            params: ParamM::default(),
            graphics: &DEFAULT_GRAPHICS,
        }
    }
}
