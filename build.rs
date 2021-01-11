extern crate vergen;

use vergen::{ConstantsFlags, generate_cargo_keys};

fn main() {
    // Setup the flags, toggling off the 'SEMVER_FROM_CARGO_PKG' flag
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);

    // Generate the 'cargo:' key output
    generate_cargo_keys(flags).expect("Unable to generate the cargo keys!");
}
/*
SNMP Data Collection - Rust - Actix - Actor Pattern
Main Thread -> DB Server : Start Data Collection
DB Server -> PostgresDB : Get POLLEDDATA
DB Server->Snmp Sender: Get Attribute
Snmp Sender -> Snmp Sender: Create PDU/Send
Snmp Receiver -> Snmp Receiver: Decode PDU
Snmp Receiver -> DB Server: Send Value
Main Thread -> Main Thread  : Wait / Sleep
Main Thread -> DB Server   : Initiate Dump
DB Server -> PostgresDB : Bulk Copy Value(s)
*/