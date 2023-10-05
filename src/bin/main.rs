extern crate clap;
use clap::Parser;
use blocks::*;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Export {
    Scad,
    Tuples,
    Analysis
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    live: Option<usize>,

    #[arg(short, long)]
    shuffles: Option<usize>,

    #[arg(short, long)]
    length: usize,

    #[arg(short, long)]
    export: Export,

    #[arg(short, long)]
    norender: bool,

    #[arg(short, long)]
    bernoulli: Option<f64>,
}

fn main() {
    let args = Args::parse();

    let mut pfm = Polyform::new(args.length, if let Some(p) = args.bernoulli {
        Dist::Bernoulli(p)
    } else {
        Dist::Uniform
    });

    // if you specify both, you'll get a pre-shuffled polyform so the less interesting shuffles
    // happen quickly

    if let Some(render_step) = args.live {
        // TODO: don't ignore the export type in render shuffle mode
        pfm.render_shuffle(render_step, args.shuffles);
    } else {
        match args.shuffles {
            Some(shuffles) => {
                pfm.shuffle(shuffles);
                
                if !args.norender {
                    // technically does n+1 shuffles, there's an easy fix here but it's not super important
                    println!("{}", match args.export { 
                        Export::Scad => pfm.export_scad(),
                        Export::Tuples => pfm.export(),
                        Export::Analysis => pfm.export_analysis()
                    });
                    pfm.render_shuffle(1, Some(1));
                } else {
                    println!("{}", match args.export { 
                        Export::Scad => pfm.export_scad(),
                        Export::Tuples => pfm.export(),
                        Export::Analysis => pfm.export_analysis()
                    });
                }
            },
            None => {
                eprintln!("Use --shuffles <count> to supply the number of shuffles.");
            }
        }
    }
}
