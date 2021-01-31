use structopt::StructOpt;

mod model;
use model::assembler::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), String> {    
    let args = Cli::from_args();
    let mut assembler: Assembler = create_assembler(&args.path);
    match assembler.run() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e))
    }
}
