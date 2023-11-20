use blocks::*;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String
}


fn main() {
    let args = Args::parse();

    let analysis = fs::read(args.file).expect("Couldn't read file");
    let analysis_string = &std::str::from_utf8(&analysis).expect("Expected UTF8 Encoding"); 
    let pfm = match Polycube::import_analysis(&analysis_string) {
        Ok(pfm_res) => pfm_res.1,
        Err(e) => {
            eprintln!("Error {}", e);
            return;
        }
    };

    println!("{}", pfm.insertable_locations.len());

}
