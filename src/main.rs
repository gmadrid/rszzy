mod rszzy;

use anyhow::Error;
use fehler::throws;
use rszzy::ZMachine;
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rszzy")]
struct Opt {
    #[structopt(parse(from_os_str))]
    story_file: std::path::PathBuf,
}

#[throws]
fn main() {
    let opt = Opt::from_args();
    let file = File::open(&opt.story_file)?;
    let zmachine = ZMachine::from_reader(file)?;
    zmachine.run()?
}
