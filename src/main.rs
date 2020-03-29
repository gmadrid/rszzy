mod rszzy;

use std::fs::File;
use rszzy::ZMachine;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rszzy")]
struct Opt {
    #[structopt(parse(from_os_str))]
    story_file: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let file = File::open(&opt.story_file)?;
    let zmachine = ZMachine::from_reader(file)?;
    zmachine.run()
}
