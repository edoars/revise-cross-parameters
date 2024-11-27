use clap::Parser;
use rayon::ThreadPoolBuilder;
use revise_cross_parameters::{attack, attack_new};

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// Prime order of finite field Fp
    #[arg(short)]
    p: i64,

    /// Number of parallel repetitions
    #[arg(short)]
    t: i64,

    /// Fixed-weight parameter for the second challenge
    #[arg(short)]
    w: i64,

    /// Number of threads (default all)
    #[structopt(long)]
    threads: Option<usize>,

    /// Do not show a progress bar
    #[structopt(long)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    if let Some(threads) = args.threads {
        ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .unwrap();
    }

    println!("Estimating complexity of original attack...");
    let (ts, comp_cross) = attack(args.t, args.w, args.p, args.quiet);
    println!(
        "Original attack has a cost of {:.2} bits",
        comp_cross.to_f32()
    );
    println!("Original attack is optimized for t* = {}", ts);

    println!();

    println!("Estimating complexity of our attack...");
    let (ts_our, aa, comp_our) = attack_new(args.t, args.w, args.p, args.quiet);
    println!("Our attack has a cost of {:.2} bits", comp_our.to_f32());
    println!(
        "Our attack is optimized for t* = {} and alpha = {}",
        ts_our, aa
    );
}
