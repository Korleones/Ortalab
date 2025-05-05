#![allow(unused)]
use std::{
    env, error::Error, fs::File, io::{stdin, Read}, path::{Path, PathBuf}
};
use ortalab::poker_base::CardFilter;
use clap::Parser;
use ortalib::{Chips, Mult, Round,Suit};
use ortalab::poker_base::*;

#[derive(Parser)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round);

    println!("{}", (chips * mult).floor());
    
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

fn score(round: Round) -> (Chips, Mult) {
     
    let result: Result<(Chips, Mult), Box<dyn std::error::Error>> = calculate_score_based_on_poker_hand(round);

    match result {
        Ok((chips, mult)) => {
           (chips, mult)
        }
        Err(e) => {
            (0.0,0.0)
        }
    }
}
