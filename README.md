# REL-16 Virtual Machine

Current version: 0.0.1

REL-16 stands for Reversible/Entropy-Less 16-bit, and is an instruction set architecture (ISA) designed for reversibility of operations. This virtual machine allows for emulation of such an architecture as an example of [reversible computing](https://en.wikipedia.org/wiki/Reversible_computing "Wikipedia - Reversible computing"), and is intended as a proof-of-concept.

## Instruction Set

The details of this ISA can be found in the ISA-SPEC.md file on this directory.

## Installation

To intall the VM you'll first need the [rust compiler and cargo](https://www.rust-lang.org/ "Rust Homepage"). Then, download the project however you want (e.g. GitHub's "Download ZIP" button, `git clone`, etc.) and decompress it. Open a terminal, go to the resulting directory, and run

	cargo install

## Usage

After installing, you can run the VM from the terminal using

	rel16-vm FILE

where FILE is a binary file containing the instructions to be loaded and executed. Such a file can be generated by the [REL-16 assembler](https://github.com/1011X/REL16-Assembler).

## License

The default license is GPLv3, but an MIT license is available for commercial uses at a price. Contact this repository's owner if you're interested.
