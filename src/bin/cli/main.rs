extern crate clap;
extern crate tapl;

mod unty_arith;

use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};

use tapl::eval::Evaluator;
use tapl::parser::Parser;
use tapl::term::Term;

use unty_arith::Topic as UntyArithTopic;

const ALL_TOPICS: [&'static str; 1] = ["unty-arith"];

fn main() {
    let args = App::new("TaPL CLI")
        .version("0.1.0")
        .author("Sirui Mu <msrlancern@gmail.com>")
        .about("Command line interface to TaPL implementations")
        .arg(
            Arg::with_name("topic")
                .short("t")
                .long("topic")
                .required(true)
                .takes_value(true)
                .value_name("TOPIC")
                .possible_values(&ALL_TOPICS)
                .help("the topic discussed in TaPL"),
        )
        .arg(
            Arg::with_name("file")
                .required(true)
                .takes_value(true)
                .value_name("FILE")
                .help("path to the input file"),
        )
        .get_matches();

    let topic = args.value_of("topic").unwrap();
    match topic {
        "unty-arith" => do_main::<UntyArithTopic>(&args),
        _ => {
            eprintln!("Unknown topic: {}", topic);
            std::process::exit(1);
        }
    }
}

trait Topic {
    type Evaluator: Evaluator<Self::Term>;
    type Parser: Parser<Self::Term>;
    type Term: Term;

    fn create_evaluator() -> Self::Evaluator;
    fn create_parser() -> Self::Parser;
}

fn do_main<T: Topic>(args: &ArgMatches) {
    let input_path = PathBuf::from(String::from(args.value_of("file").unwrap()));
    let input = match std::fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
            std::process::exit(1);
        }
    };

    let mut parser = T::create_parser();
    let term = match parser.parse(&input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to parse input file: {}", e);
            std::process::exit(1);
        }
    };

    let mut eval = T::create_evaluator();
    let mut first_term = true;
    let eval_result = eval.eval_with_monitor(term, |t| {
        if first_term {
            first_term = false;
            print!("   ");
        } else {
            print!("-> ");
        }

        println!("{}", t);
    });

    if let Err(e) = eval_result {
        eprintln!("Failed to evaluate");
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
