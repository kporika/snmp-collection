extern crate vergen;

fn main() {
    let flags = vergen::ConstantsFlags::all();
    // Generate the version.rs file in the Cargo OUT_DIR.
    assert!(vergen::generate_version_rs(flags).is_ok());
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