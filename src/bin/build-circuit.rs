use circom_witnesscalc::build_circuit::Args;
use std::env;
use std::path::PathBuf;

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let mut circuit_file: Option<String> = None;
    let mut graph_file: Option<String> = None;
    let mut link_libraries: Vec<PathBuf> = Vec::new();
    let mut inputs_file: Option<String> = None;
    let mut print_unoptimized = false;
    let mut print_debug = false;

    let usage = |err_msg: &str| -> String {
        eprintln!("{}", err_msg);
        eprintln!("Usage: {} <circuit_file> <graph_file> [-l <link_library>]* [-i <inputs_file.json>] [-print-unoptimized] [-v]", args[0]);
        std::process::exit(1);
    };

    while i < args.len() {
        if args[i] == "-l" {
            i += 1;
            if i >= args.len() {
                usage("missing argument for -l");
            }
            link_libraries.push(args[i].clone().into());
        } else if args[i].starts_with("-l") {
            link_libraries.push(args[i][2..].to_string().into())
        } else if args[i] == "-i" {
            i += 1;
            if i >= args.len() {
                usage("missing argument for -i");
            }
            if inputs_file.is_none() {
                inputs_file = Some(args[i].clone());
            } else {
                usage("multiple inputs files");
            }
        } else if args[i].starts_with("-i") {
            if inputs_file.is_none() {
                inputs_file = Some(args[i][2..].to_string());
            } else {
                usage("multiple inputs files");
            }
        } else if args[i] == "-print-unoptimized" {
            print_unoptimized = true;
        } else if args[i] == "-v" {
            print_debug = true;
        } else if args[i].starts_with("-") {
            let message = format!("unknown argument: {}", args[i]);
            usage(&message);
        } else if circuit_file.is_none() {
            circuit_file = Some(args[i].clone());
        } else if graph_file.is_none() {
            graph_file = Some(args[i].clone());
        } else {
            usage(format!("unexpected argument: {}", args[i]).as_str());
        }
        i += 1;
    }

    Args {
        circuit_file: circuit_file.unwrap_or_else(|| usage("missing circuit file")),
        inputs_file,
        graph_file: graph_file.unwrap_or_else(|| usage("missing graph file")),
        link_libraries,
        print_unoptimized,
        print_debug,
    }
}

fn main() {
    // TODO: Fix this
    // let args = parse_args();
    // let version = "2.1.9";
    // let bytes = build_circuit_flow(&args, version);
    //
    // fs::write(&args.graph_file, bytes).unwrap();
    //
    // println!("circuit graph saved to file: {}", &args.graph_file)
}
