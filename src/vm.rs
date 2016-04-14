use instr::Op;
use instr::decode;

use std::mem;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn vm(file_path: &Path) {
	let mut input = File::open(file_path).unwrap();
	
	// dir: false is forward, true is reverse mode
	let mut dir = false;
	let mut br: u16 = 0;
	let mut pc: u16 = 0;
	
	let mut ir: u16;
	let mut reg = [0_u16; 16];
	let mut data_mem = [0_u16; 65536];
	let mut program_mem = [0_u16; 65536];
	
	reg[15] = 0xFFFF; // use r15 as stack pointer
	
	// read file contents into mem
	let mut buffer = [0_u8; 2];
	for i in 0.. {
		match input.read(&mut buffer) {
			Ok(0) => break,
			
			Ok(1) => panic!("Got incomplete instruction."),
			
			Ok(2) => if i < program_mem.len() {
				program_mem[i] = (buffer[0] as u16) << 8 | buffer[1] as u16;
			} else {
				panic!("Binary is too big for memory!");
			},
			
			Ok(_) => unreachable!(),
			
			Err(e) => panic!("{}", e),
		}
	}
	
	loop {
		// fetch
		ir = program_mem[pc as usize];
		
		if dir {
			pc += if br == 0 { 1 } else { br };
		}
		else {
			pc -= if br == 0 { 1 } else { br };
		}
		
		// execute
		match decode(ir) {
			Op::Immediate(r, v) => reg[r] ^= v as u16,
			
			Op::Exchange(r, addr) => mem::swap(&mut reg[r], &mut data_mem[addr]),
			
			Op::Halt => break,
			
			
			Op::Not(a) => reg[a] = !reg[a],
			
			Op::RotateLeft(a) => reg[a] = reg[a].rotate_left(1),
			Op::RotateRight(a) => reg[a] = reg[a].rotate_right(1),
			
			Op::Increment(a) => reg[a] = reg[a].wrapping_add(1),
			Op::Decrement(a) => reg[a] = reg[a].wrapping_sub(1),
			
			Op::Push(r) => {
				let mut sp = reg[15] as usize;
				sp -= 1;
				mem::swap(&mut reg[r], &mut data_mem[sp]);
				reg[15] = sp as u16;
			}
			
			Op::Pop(r) => {
				let mut sp = reg[15] as usize;
				mem::swap(&mut reg[r], &mut data_mem[sp]);
				sp += 1;
				reg[15] = sp as u16;
			}
			
			Op::SwapBr(r) => mem::swap(&mut br, &mut reg[r]),
			
			Op::RevSwapBr(r) => {
				mem::swap(&mut br, &mut reg[r]);
				dir ^= true;
			}
			
			
			
			Op::Swap(a, b) => reg.swap(a, b),
			
			Op::CNot(a, b) => reg[b] ^= reg[a],
			
			Op::CAdd(a, b) => reg[b] += reg[a],
			
			Op::CSub(a, b) => reg[b] -= reg[a],
			
			
			Op::CCNot(a, b, c) => if a != c && b != c {
				reg[c] ^= reg[a] & reg[b];
			} else {
				panic!("ERROR (line {}): Control register in CCNot instruction used again in last parameter.", pc - 1);
			},
			
			Op::CSwap(a, b, c) => if a != c && a != b {
				let s = (reg[b] ^ reg[c]) & reg[a];
				reg[b] ^= s;
				reg[c] ^= s;
			} else {
				panic!("ERROR (line {}): Control register in CSwap instruction used again in second or last parameter.", pc - 1);
			},
			
			Op::Branch(off) => br += off,
			
			Op::BrGEZ(r, off) => br +=
				if (reg[r] as i16) >= 0 { off as u16 }
				else { 0 },
			
			Op::BrLZ(r, off) => br +=
				if (reg[r] as i16) < 0 { off as u16 }
				else { 0 },
			
			Op::BrEven(r, off) => br +=
				if reg[r] % 2 == 0 { off as u16 }
				else { 0 },
			
			Op::BrOdd(r, off) => br +=
				if reg[r] % 2 != 0 { off as u16 }
				else { 0 },
			
			/*
			Read(c) => {
				let mut buf = [0_u8, 0];
				std::io::stdin().read(&mut buf).expect("Couldn't read from stdin.");
				reg[0] = ((buf[0] as u16) << 8) | buf[1] as u16;
			}
			
			Write(c) => print!("{}", (reg[0] as u8) as char),
			*/
		}
		
		// debugging code
		print!("PC = 0x{:04X}: ", pc);
		
		println!("{:<17}", format!("{:?}", decode(ir)));
		
		print!("SP = 0x{:04X}: ", reg[15]);
		for (i, &val) in data_mem[reg[15] as usize..].iter().enumerate() {
			print!("0x{:04X}", val);
			
			// is not last item
			if i != data_mem.len() - data_mem[15] as usize - 1 {
				print!(", ");
			}
		}
		println!("");
		
		print!("Registers: [");
		for (i, &r) in reg[..reg.len() - 1].iter().enumerate() {
			print!("0x{:04X}", r);
			
			// is not last item
			if i != reg.len() - 2 {
				print!(", ");
			}
		}
		println!("]\n");
	}
}
