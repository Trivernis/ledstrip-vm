use ledstrip_vm::runtime::Runtime;
use std::fs::read;
use std::io;
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
    runtime.parse_bytecode(bytecode);

    match runtime.run() {
        Ok(code) => println!("Runtime exited with code {}", code),
        Err(e) => println!("Runtime exited with error {:?}", e),
    }

    Ok(())
}
