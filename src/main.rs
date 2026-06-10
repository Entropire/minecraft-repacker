use clap::Parser;
use std::process;
use minecraft_repacker::repack;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    pack_path: String,
    target_version: String
}

fn main() {
    let args = Args::parse();

    repack(&args.pack_path, &args.target_version).unwrap_or_else(|err| {
        eprint!("{}", err);
        process::exit(1)
    });
}

