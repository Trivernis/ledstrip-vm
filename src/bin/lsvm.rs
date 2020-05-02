use ledstrip_vm::runtime::Runtime;
use std::fs::read;
use std::io;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(short = "i", name = "input")]
    input_file: String,

    #[structopt(short = "a", name = "address")]
    ip: String,

    #[structopt(short = "p", name = "port")]
    port: usize,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::from_args();
    let bytecode = read(opts.input_file)?;

    let mut runtime = Runtime::new(&opts.ip, opts.port);
    let start = Instant::now();
    runtime.parse_bytecode(bytecode);
    println!("Parsing took {:?}\n", start.elapsed());

    let start = Instant::now();
    match runtime.run() {
        Ok(code) => println!(
            "Runtime exited with code {} after {:?}",
            code,
            start.elapsed()
        ),
        Err(e) => println!(
            "Runtime exited with error {:?} after {:?}",
            e,
            start.elapsed()
        ),
    }

    Ok(())
}
