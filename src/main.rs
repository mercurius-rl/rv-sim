mod parts;
mod asm_f;

use parts::*;
use argh::FromArgs;

#[derive(FromArgs)]
/// Reach new heights.
struct Args {

	#[argh(positional)]
    src: String,

	/// run assembler
	#[argh(switch, short = 'a')]
	asm: bool,

	/// output assembled binary
	#[argh(option, short = 'o', default = "String::from(\"a.out\")")]
	output: String,

	/// run risc-v simulator
	#[argh(switch, short = 's')]
	sim: bool,

	/// binary data file
	#[argh(option, short = 'b', default = "String::from(\"a.out\")")]
	bin: String,

	/// debug run
	#[argh(switch, short = 'd')]
	dbg: bool,

	/// memory dump
	#[argh(switch, short = 'm')]
	memdump: bool,
}

fn main() {
	let mut ag: Args = argh::from_env();

	if ag.asm == ag.sim {
		if ag.output != ag.bin {
			println!("Fixed binary data path...");
			ag.output = ag.bin.clone();
		}
		asm_f::asm(&ag.src, &ag.output);
		let mut vm = VMachine::new(1024);

		vm.cpu.binread(&ag.bin);
		if ag.dbg {
			vm.start_dbg();
		} else {
			vm.start();
		}

		if ag.memdump {
			let d: u32 = vm.cpu.memory.read(0);
			println!("[Memory dump mode]");
			println!("Address[{:08x}]: {:08x} ",0, d);

			for i in 1..30 {
				let d: u32 = vm.cpu.memory.read(i*4);
				println!("Address[{:08x}]: {:08x} ",i*4, d);
			}
		}
	} else {
		if ag.asm {
			asm_f::asm(&ag.src, &ag.output);
		}

		if ag.sim {
			let mut vm = VMachine::new(1024);

			vm.cpu.binread(&ag.bin);
			if ag.dbg {
				vm.start_dbg();
			} else {
				vm.start();
			}

			if ag.memdump {
				let d: u32 = vm.cpu.memory.read(0);
				println!("[Memory dump mode]");
				println!("Address[{:08x}]: {:08x} ",0, d);

				for i in 1..30 {
					let d: u32 = vm.cpu.memory.read(i*4);
					println!("Address[{:08x}]: {:08x} ",0, d);
				}
			}
		}
	}
}
