#[macro_use] extern crate clap;
use clap::Parser;
use blocks::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    render: Option<usize>,

    #[arg(short, long)]
    shuffles: Option<usize>,

    #[arg(short, long)]
    length: usize,

    #[arg(short, long)]
    export: bool,
}

fn main() {
    let args = Args::parse();

    let mut pfm = Polyform::new(args.length);

    // if you specify both, you'll get a pre-shuffled polyform so the less interesting shuffles
    // happen quickly

    if let Some(render_step) = args.render {
        pfm.render_shuffle(render_step, args.shuffles);
    } else {
        println!("Please use the --render <render_step> flag if you would like a visualization");
        match args.shuffles {
            Some(shuffles) => {
                pfm.shuffle(shuffles);
            },
            None => {
                loop {
                    pfm.shuffle(usize::max_value());
                }
            }
        }
        println!("{}", pfm.export_scad());
    }

}
