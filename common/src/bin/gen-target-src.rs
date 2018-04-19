use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::fs::OpenOptions;

fn main() {
    let mut args = env::args();
    if args.len() != 4 {
        eprintln!("usage: {} TARGET_LIST_PATH TEMPLATE_PATH OUT_PATH", args.nth(0).unwrap());
        process::exit(1);
    }
    
    let _ = args.next();
    let targets_path  = args.next().unwrap();
    let template_path = args.next().unwrap();
    let out_path      = args.next().unwrap();

    let targets_file = File::open(targets_path)
        .expect("failed to open targets_path");

    let mut template_file = File::open(template_path)
        .expect("failed to open template_path");

    let mut template = String::new();
    template_file.read_to_string(&mut template).unwrap();

    for target in BufReader::new(targets_file).lines() {
        let target = target.unwrap();
        let path = format!("{}/{}.rs", out_path, target);
        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect(&format!("unable to write to `{}`", path));
        
        let source = template.replace("###TARGET###", &target);
        file.write_all(source.as_bytes()).unwrap();
    }
}