use fluvio_smartstream::{smartstream, Record, Result};

#[smartstream(filter)]
pub fn my_filter(_record: &Record) -> Result<bool> {
    unimplemented!()
}

fn main() {}
