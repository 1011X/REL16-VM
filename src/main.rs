// Notes to self
// 
// features to include:
// + labels (@)
// + scheduled branches?
// + register allocation ($)
// + macros
// + assembler directives (:)
// 
// also: y'know those leftover imm bits when rotating? maybe use them
// for something? like idk, xoring afterwards? after all, shifting and
// ANDing bits is a somewhat common pattern analogous to rotating and
// XORing.

#[macro_use]
extern crate logos;

mod vm;
mod assembler;
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
	// TODO: verbosity levels
	// "v" => show cpu info when encountering a debug instruction
	// "vv" => show cpu info at every step
	opts.optflag("v", "verbose", "Log each step the VM takes");
	opts.optflag("", "garbage-stack", "Add garbage stack device to device manager");
	
	let matches = opts.parse(args).unwrap();

	if matches.opt_present("version") {
		println!("rel 0.6.0");
		return;
	}
	
	if matches.opt_present("help") {
		eprintln!("{}", opts.usage(USAGE));
		return;
	}
	
	if let Some(arg) = matches.free.get(0) {
		let reader = BufReader::new(File::open(arg).unwrap());
		let assembly = assembler::parse(reader).unwrap();
		let code: Vec<_> = assembly.into_iter().map(|op| op.into()).collect();
		let logging_enabled = matches.opt_present("verbose");
		let mut dm = vm::DeviceManager::new();
		
		if matches.opt_present("garbage-stack") {
		    dm.add(vm::Device::Stack(Vec::new()));
		}
		
	    let mut cpu = vm::Cpu::new(&code, &mut dm, logging_enabled);
	    cpu.run();
		
		dm.debug_devices();
	} else {
		eprintln!("Error: Missing input file.\n");
		eprintln!("{}", opts.usage(USAGE));
	}
}
