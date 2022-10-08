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
}

fn main() {
    let args = Args::parse();

    let mut pfm = Polyform::new(args.length);

    // if you specify both, you'll get a pre-shuffled polyform so the less interesting shuffles
    // happen quickly
    if let Some(shuffles) = args.shuffles {
        pfm.shuffle(shuffles);
    }

    if let Some(shuffles_per_render) = args.render {
        pfm.render_shuffle(shuffles_per_render);
        return;
    }

    pfm.render();
}
