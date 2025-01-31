extern crate core;

mod common;
mod v6;
mod v7;
mod constants;
mod fixed_parameters;
mod variable_parameters;

include!(concat!(env!("OUT_DIR"), "/enumerations.rs"));

pub use common::parse;
pub use common::parse_v6 as parse_v6_pdus;
pub use common::parse_v7 as parse_v7_pdus;
pub use common::Serialize;
pub use common::*;

pub use fixed_parameters::*;
pub use variable_parameters::VariableParameters;

pub use common::errors::*;

/*
TODO:
- Common function for calculating body length based on header data (pdu_length - header_length); now at several places
- Dead-reckoning algorithms

TESTS:
- Build Other PDU
- Build EntityState PDU
- Reading EntityState PDU
- DIS v7 header, PduStatus fields

*/