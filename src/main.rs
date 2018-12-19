extern crate getopts;

mod vm;
mod asm;
mod isa;

use std::env;
use std::io::BufReader;
use std::fs::File;
use getopts::Options;


static USAGE: &str = "\
Usage:
    rel [--version] [--help]
    rel [--verbose] <file>";


fn main() {
	let args = env::args().skip(1);
	
	let mut opts = Options::new();
	opts.optflag("V", "version", "Print program version");
	opts.optflag("h", "help",    "Print this help menu");
	opts.optflag("v", "verbose", "Log each step the VM takes");
	opts.optflag("", "garbage-stack", "Add garbage stack device to device manager");
	
	let matches = opts.parse(args).unwrap();

	if matches.opt_present("version") {
		println!("rel 0.3.1");
		return;
	}
	
	if matches.opt_present("help") {
		eprintln!("{}", opts.usage(USAGE));
		return;
	}
	
	if let Some(arg) = matches.free.get(0) {
		let reader = BufReader::new(File::open(arg).unwrap());
		let code = asm::parse(reader).unwrap();
		let logging_enabled = matches.opt_present("verbose");
		let mut dm = vm::DeviceManager::new();
		
		if matches.opt_present("garbage-stack") {
		    dm.add(vm::Device::Stack(Vec::new()));
		}
		
	    let mut cpu = vm::Cpu::new(&code, &mut dm, logging_enabled);
	    cpu.run();
		
		dm.debug_devices();
	}
	else {
		eprintln!("Error: Missing input file.\n");
		eprintln!("{}", opts.usage(USAGE));
	}
}
