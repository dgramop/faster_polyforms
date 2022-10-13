#[macro_use] extern crate clap;
use clap::Parser;
use blocks::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    live: Option<usize>,

    #[arg(short, long)]
    shuffles: Option<usize>,

    #[arg(short, long)]
    length: usize,

    #[arg(short, long)]
    export: bool,

    #[arg(short, long)]
    norender: bool,
}

fn main() {
    let args = Args::parse();

    let mut pfm = Polyform::new(args.length);

    // if you specify both, you'll get a pre-shuffled polyform so the less interesting shuffles
    // happen quickly

    if let Some(render_step) = args.live {
        pfm.render_shuffle(render_step, args.shuffles);
    } else {
        match args.shuffles {
            Some(shuffles) => {
                pfm.shuffle(shuffles);
            },
            None => {
                eprintln!("Use --shuffles <count> to supply the number of shuffles.");
            }
        }
        if !args.norender {
            // technically does n+1 shuffles, there's an easy fix here but it's not super important
            pfm.render_shuffle(1, Some(1))
        }
    }

}
