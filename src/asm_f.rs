extern crate regex;

//use std::default;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::io::prelude::*;

use regex::Regex;

pub fn one_line_asm(line: &str) -> u32 {
	let mut map = HashMap::new();
	map.insert("zero", 0);
	map.insert("ra", 1);
	map.insert("sp", 2);
	map.insert("gp", 3);
	map.insert("tp", 4);
	map.insert("t0", 5);
	map.insert("t1", 6);
	map.insert("t2", 7);
	map.insert("s0", 8);
	map.insert("fp", 8);
	map.insert("s1", 9);
	map.insert("a0", 10);
	map.insert("a1", 11);
	map.insert("a2", 12);
	map.insert("a3", 13);
	map.insert("a4", 14);
	map.insert("a5", 15);
	map.insert("a6", 16);
	map.insert("a7", 17);
	map.insert("s2", 18);
	map.insert("s3", 19);
	map.insert("s4", 20);
	map.insert("s5", 21);
	map.insert("s6", 22);
	map.insert("s7", 23);
	map.insert("s8", 24);
	map.insert("s9", 25);
	map.insert("s10", 26);
	map.insert("s11", 27);
	map.insert("t3", 28);
	map.insert("t4", 29);
	map.insert("t5", 30);
	map.insert("t6", 31);
	map.insert("x0", 0);
	map.insert("x1", 1);
	map.insert("x2", 2);
	map.insert("x3", 3);
	map.insert("x4", 4);
	map.insert("x5", 5);
	map.insert("x6", 6);
	map.insert("x7", 7);
	map.insert("x8", 8);
	map.insert("x9", 9);
	map.insert("x10", 10);
	map.insert("x11", 11);
	map.insert("x12", 12);
	map.insert("x13", 13);
	map.insert("x14", 14);
	map.insert("x15", 15);
	map.insert("x16", 16);
	map.insert("x17", 17);
	map.insert("x18", 18);
	map.insert("x19", 19);
	map.insert("x20", 20);
	map.insert("x21", 21);
	map.insert("x22", 22);
	map.insert("x23", 23);
	map.insert("x24", 24);
	map.insert("x25", 25);
	map.insert("x26", 26);
	map.insert("x27", 27);
	map.insert("x28", 28);
	map.insert("x29", 29);
	map.insert("x30", 30);
	map.insert("x31", 31);
	let reg = map;

	let spl: Vec<&str> = line.split(' ').collect();
	let mut sv: Vec<&str> = vec![spl[0]];
	if spl.len() > 1 {
		let mut opr: Vec<&str> = spl[1].split(',').collect();
		sv.append(&mut opr);
	}

	if sv.len() > 4 {
		panic!("sv");
	}

	// pseudo instruction predecode
	match sv[0] {
		"J" | "j" => {
			// j offset -> jal x0,offset
			sv[0]	=	"jal";
			sv[2]	=	sv[1];
			sv[1]	=	"x0";
		},
		"JR" | "jr" => {
			// jr rs -> jalr x0,rs,0
			sv[0]	=	"jalr";
			sv[2]	=	sv[1];
			sv[1]	=	"x0";
			sv.push("0");
		},
		"BEQZ" | "beqz" => {
			// beqz rs,offset -> beq rs,x0,offset
			sv[0]	=	"beq";
			sv.push(sv[2]);
			sv[2]	=	"x0";
		},
		"BNEZ" | "bnez" => {
			// bnez rs,offset -> bne rs,x0,offset
			sv[0]	=	"bne";
			sv.push(sv[2]);
			sv[2]	=	"x0";
		},
		"BGT" | "bgt" => {
			// bgt rs,rt,offset -> blt rt,rs,offset
			sv[0]	=	"blt";
			let tmp = sv[1];
			sv[1]	=	sv[2];
			sv[2]	=	tmp;
		},
		"BLE" | "ble" => {
			// ble rs,rt,offset -> bge rt,rs,offset
			sv[0]	=	"bge";
			let tmp = sv[1];
			sv[1]	=	sv[2];
			sv[2]	=	tmp;
		},
		"LI" | "li" => {
			// li rd,imm -> addi rd,x0,imm
			sv[0]	=	"addi";
			sv.push(sv[2]);
			sv[2]	=	"x0";
		},
		"MV" | "mv" => {
			// mv rd,rs -> addi rd, rs,0
			sv[0]	=	"addi";
			sv.push("0");
		},
		_ => {},
	}

	match sv[0] {
		"LUI" | "lui" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x37;
				let rd: u32 = reg[sv[1]] << 7;
				let imm: u32 = sv[2].parse::<u32>().unwrap() & 0xFFFFF000;

				let bin = imm+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"AUIPC" | "auipc" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x17;
				let rd: u32 = reg[sv[1]] << 7;
				let imm: u32 = sv[2].parse::<u32>().unwrap() & 0xFFFFF000;

				let bin = imm+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"ADD" | "add" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x0 << 12;
				let funct7: u32 = 0x0 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SUB" | "sub" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x0 << 12;
				let funct7: u32 = 0x20 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"OR" | "or" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x6 << 12;
				let funct7: u32 = 0x0 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"AND" | "and" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x7 << 12;
				let funct7: u32 = 0x0 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"XOR" | "xor" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x4 << 12;
				let funct7: u32 = 0x0 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SLL" | "sll" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x1 << 12;
				let funct7: u32 = 0x0 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SRL" | "srl" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x5 << 12;
				let funct7: u32 = 0x0 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SRA" | "sra" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x5 << 12;
				let funct7: u32 = 0x20 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SLT" | "slt" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x2 << 12;
				let funct7: u32 = 0x0 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SLTU" | "sltu" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x3 << 12;
				let funct7: u32 = 0x0 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},

		"ADDI" | "addi" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x0 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"ORI" | "ori" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x6 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"ANDI" | "andi" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x7 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"XORI" | "xori" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x4 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SLLI" | "slli" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x1 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SRLI" | "srli" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x5 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SRAI" | "srai" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x5 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let shumt = imm << 20;

				let funct7: u32 = 0x20 << 25;

				let bin = funct7+shumt+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SLTI" | "slti" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x2 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SLTIU" | "sltiu" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x13;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x3 << 12;
				let imm: u32 = sv[3].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},

		"BEQ" | "beq" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x63;
				let rs1: u32 = reg[sv[1]] << 15;
				let rs2: u32 = reg[sv[2]] << 20;

				let funct3: u32 = 0x0 << 12;
				let imm: u32 = sv[3].parse().unwrap();

				let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
				let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"BNE" | "bne" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x63;
				let rs1: u32 = reg[sv[1]] << 15;
				let rs2: u32 = reg[sv[2]] << 20;

				let funct3: u32 = 0x1 << 12;
				let imm: u32 = sv[3].parse().unwrap();

				let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
				let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"BLT" | "blt" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x63;
				let rs1: u32 = reg[sv[1]] << 15;
				let rs2: u32 = reg[sv[2]] << 20;

				let funct3: u32 = 0x4 << 12;
				let imm: u32 = sv[3].parse().unwrap();

				let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
				let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"BGE" | "bge" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x63;
				let rs1: u32 = reg[sv[1]] << 15;
				let rs2: u32 = reg[sv[2]] << 20;

				let funct3: u32 = 0x5 << 12;
				let imm: u32 = sv[3].parse().unwrap();

				let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
				let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"BLTU" | "bltu" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x63;
				let rs1: u32 = reg[sv[1]] << 15;
				let rs2: u32 = reg[sv[2]] << 20;

				let funct3: u32 = 0x6 << 12;
				let imm: u32 = sv[3].parse().unwrap();

				let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
				let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"BGEU" | "bgeu" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x63;
				let rs1: u32 = reg[sv[1]] << 15;
				let rs2: u32 = reg[sv[2]] << 20;

				let funct3: u32 = 0x7 << 12;
				let imm: u32 = sv[3].parse().unwrap();

				let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
				let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},

		"LW" | "lw" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x3;
				let rd: u32 = reg[sv[1]] << 7;

				let tmp1: Vec<&str> = sv[2].split('(').collect();
				let tmp2: Vec<&str> = tmp1[1].split(')').collect();
				let rs1: u32 = reg[tmp2[0]] << 15;

				let funct3: u32 = 0x2 << 12;
				let imm: u32 = tmp1[0].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"LH" | "lh" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x3;
				let rd: u32 = reg[sv[1]] << 7;

				let tmp1: Vec<&str> = sv[2].split('(').collect();
				let tmp2: Vec<&str> = tmp1[1].split(')').collect();
				let rs1: u32 = reg[tmp2[0]] << 15;

				let funct3: u32 = 0x1 << 12;
				let imm: u32 = tmp1[0].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"LB" | "lb" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x3;
				let rd: u32 = reg[sv[1]] << 7;

				let tmp1: Vec<&str> = sv[2].split('(').collect();
				let tmp2: Vec<&str> = tmp1[1].split(')').collect();
				let rs1: u32 = reg[tmp2[0]] << 15;

				let funct3: u32 = 0x0 << 12;
				let imm: u32 = tmp1[0].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"LHU" | "lhu" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x3;
				let rd: u32 = reg[sv[1]] << 7;

				let tmp1: Vec<&str> = sv[2].split('(').collect();
				let tmp2: Vec<&str> = tmp1[1].split(')').collect();
				let rs1: u32 = reg[tmp2[0]] << 15;

				let funct3: u32 = 0x5 << 12;
				let imm: u32 = tmp1[0].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"LBU" | "lbu" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x3;
				let rd: u32 = reg[sv[1]] << 7;

				let tmp1: Vec<&str> = sv[2].split('(').collect();
				let tmp2: Vec<&str> = tmp1[1].split(')').collect();
				let rs1: u32 = reg[tmp2[0]] << 15;

				let funct3: u32 = 0x4 << 12;
				let imm: u32 = tmp1[0].parse().unwrap();
				let imms = imm << 20;

				let bin = imms+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SW" | "sw" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x23;
				let rs2: u32 = reg[sv[1]] << 20;

				let tmp1: Vec<&str> = sv[2].split('(').collect();
				let tmp2: Vec<&str> = tmp1[1].split(')').collect();
				let rs1: u32 = reg[tmp2[0]] << 15;

				let funct3: u32 = 0x2 << 12;
				let imm: u32 = tmp1[0].parse().unwrap();
				let rd: u32 = (imm & 0x1F) << 7;

				let funct7: u32 = (imm >> 5) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SH" | "sh" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x23;
				let rs2: u32 = reg[sv[1]] << 20;

				let tmp1: Vec<&str> = sv[2].split('(').collect();
				let tmp2: Vec<&str> = tmp1[1].split(')').collect();
				let rs1: u32 = reg[tmp2[0]] << 15;

				let funct3: u32 = 0x1 << 12;
				let imm: u32 = tmp1[0].parse().unwrap();
				let rd: u32 = (imm & 0x1F) << 7;

				let funct7: u32 = (imm >> 5) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"SB" | "sb" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x23;
				let rs2: u32 = reg[sv[1]] << 20;

				let tmp1: Vec<&str> = sv[2].split('(').collect();
				let tmp2: Vec<&str> = tmp1[1].split(')').collect();
				let rs1: u32 = reg[tmp2[0]] << 15;

				let funct3: u32 = 0x0 << 12;
				let imm: u32 = tmp1[0].parse().unwrap();
				let rd: u32 = imm & 0x1F << 7;

				let funct7: u32 = (imm >> 5) << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},

		"JAL" | "jal" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x6F;
				let rd: u32 = reg[sv[1]] << 7;

				let imm: u32 = sv[2].parse().unwrap();

				let imm1: u32 = imm & 0xFF000;
				let imm2: u32 = ((imm & 0x7FE) + ((imm & 0x800) >> 10) + ((imm & 0x100000) >> 9)) << 20;

				let bin = imm2+imm1+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"JALR" | "jalr" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
				let op: u32 = 0x67;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;

				let funct3: u32 = 0x0 << 12;
				let imm: u32 = sv[3].parse().unwrap();

				let imm1: u32 = imm << 20;

				let bin = imm1+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"CSRRW" | "csrrw" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x73;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[3]] << 15;

				let funct3: u32 = 0x1 << 12;
				let c: u32 = sv[2].parse().unwrap();
				let csr: u32 = c << 20;

				let bin = csr+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"CSRRS" | "csrrs" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x73;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[3]] << 15;

				let funct3: u32 = 0x2 << 12;
				let c: u32 = sv[2].parse().unwrap();
				let csr: u32 = c << 20;

				let bin = csr+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"CSRRC" | "csrrc" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x73;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[3]] << 15;

				let funct3: u32 = 0x3 << 12;
				let c: u32 = sv[2].parse().unwrap();
				let csr: u32 = c << 20;

				let bin = csr+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"CSRRWI" | "csrrwi" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x73;
				let rd: u32 = reg[sv[1]] << 7;

				let i: u32 = sv[3].parse().unwrap();
				let imm: u32 = i << 15;

				let funct3: u32 = 0x5 << 12;
				let c: u32 = sv[2].parse().unwrap();
				let csr: u32 = c << 20;

				let bin = csr+imm+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"CSRRSI" | "csrrsi" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x73;
				let rd: u32 = reg[sv[1]] << 7; 

				let i: u32 = sv[3].parse().unwrap();
				let imm: u32 = i << 15;

				let funct3: u32 = 0x6 << 12;
				let c: u32 = sv[2].parse().unwrap();
				let csr: u32 = c << 20;

				let bin = csr+imm+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"CSRRCI" | "csrrci" => {
			if reg.contains_key(sv[1]) {
				let op: u32 = 0x73;
				let rd: u32 = reg[sv[1]] << 7;

				let i: u32 = sv[3].parse().unwrap();
				let imm: u32 = i << 15;

				let funct3: u32 = 0x7 << 12;
				let c: u32 = sv[2].parse().unwrap();
				let csr: u32 = c << 20;

				let bin = csr+imm+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		
		"ECALL" | "ecall" => {
			0x00000073 as u32
		},
		"EBREAK" | "ebreak" => {
			0x00100073 as u32
		},

		"URET" | "uret" => {
			0x00200073 as u32
		},
		"SRET" | "sret" => {
			0x10200073 as u32
		},
		"MRET" | "mret" => {
			0x20200073 as u32
		},

		"FENCE" | "fence" => {
			let pred: u32 = sv[1].parse().expect("pred parameter can't be parse to numerical.");
			let succ: u32 = sv[2].parse().expect("succ parameter can't be parse to numerical.");

			let bin: u32 = 0x0000000F + ((pred & 0xF) << 24) + ((succ & 0xF) << 20);
			bin
		},

		"FENCE.I" | "fence.i" => {
			0x0000100F as u32
		}

		"MUL" | "mul" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x0 << 12;
				let funct7: u32 = 0x1 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"MULH" | "mulh" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x1 << 12;
				let funct7: u32 = 0x1 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"MULHSU" | "mulhsu" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x2 << 12;
				let funct7: u32 = 0x1 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"MULHU" | "mulhu" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x3 << 12;
				let funct7: u32 = 0x1 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"DIV" | "div" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x4 << 12;
				let funct7: u32 = 0x1 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"DIVU" | "divu" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x5 << 12;
				let funct7: u32 = 0x1 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"REM" | "rem" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x6 << 12;
				let funct7: u32 = 0x1 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},
		"REMU" | "remu" => {
			if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
				let op: u32 = 0x33;
				let rd: u32 = reg[sv[1]] << 7;
				let rs1: u32 = reg[sv[2]] << 15;
				let rs2: u32 = reg[sv[3]] << 20;

				let funct3: u32 = 0x7 << 12;
				let funct7: u32 = 0x1 << 25;

				let bin = funct7+rs2+rs1+funct3+rd+op;
				bin
			} else {
				let bin: u32 = 0;
				bin
			}
		},

		"NOP" | "nop" => {
			let op: u32 = 0x13;
			let rd: u32 = 0 << 7;
			let rs1: u32 = 0 << 15;

			let funct3: u32 = 0x0 << 12;
			let imm: u32 = 0x0 << 20;
			let bin = imm+rs1+funct3+rd+op;
			bin
		},
		"HALT" | "halt" => {
			let op: u32 = 0x7F;
			let bin = op;
			bin
		},
		_ => {
			let bin: u32 = 0;
			bin
		}
	}
}

/*
fn regtype() -> HashMap<str, i32> {
	let mut map = HashMap::new();
	map.insert("zero", 0);
	map.insert("ra", 1);
	map.insert("sp", 2);
	map.insert("gp", 3);
	map.insert("tp", 4);
	map.insert("t0", 5);
	map.insert("t1", 6);
	map.insert("t2", 7);
	map.insert("s0", 8);
	map.insert("fp", 8);
	map.insert("s1", 9);
	map.insert("a0", 10);
	map.insert("a1", 11);
	map.insert("a2", 12);
	map.insert("a3", 13);
	map.insert("a4", 14);
	map.insert("a5", 15);
	map.insert("a6", 16);
	map.insert("a7", 17);
	map.insert("s2", 18);
	map.insert("s3", 19);
	map.insert("s4", 20);
	map.insert("s5", 21);
	map.insert("s6", 22);
	map.insert("s7", 23);
	map.insert("s8", 24);
	map.insert("s9", 25);
	map.insert("s10", 26);
	map.insert("s11", 27);
	map.insert("t3", 28);
	map.insert("t4", 29);
	map.insert("t5", 30);
	map.insert("t6", 31);
}
*/

#[allow(dead_code, unused_must_use)]
pub fn asm(path: &str, out: &str) {
	let file = File::open(path).unwrap();
	let reader = BufReader::new(file);
	let mut writer = File::create(out).unwrap();

	let mut label: Vec<(String, u32)> = Vec::new();
	let mut addr_count: u32 = 0;

	let mut map = HashMap::new();
	map.insert("zero", 0);
	map.insert("ra", 1);
	map.insert("sp", 2);
	map.insert("gp", 3);
	map.insert("tp", 4);
	map.insert("t0", 5);
	map.insert("t1", 6);
	map.insert("t2", 7);
	map.insert("s0", 8);
	map.insert("fp", 8);
	map.insert("s1", 9);
	map.insert("a0", 10);
	map.insert("a1", 11);
	map.insert("a2", 12);
	map.insert("a3", 13);
	map.insert("a4", 14);
	map.insert("a5", 15);
	map.insert("a6", 16);
	map.insert("a7", 17);
	map.insert("s2", 18);
	map.insert("s3", 19);
	map.insert("s4", 20);
	map.insert("s5", 21);
	map.insert("s6", 22);
	map.insert("s7", 23);
	map.insert("s8", 24);
	map.insert("s9", 25);
	map.insert("s10", 26);
	map.insert("s11", 27);
	map.insert("t3", 28);
	map.insert("t4", 29);
	map.insert("t5", 30);
	map.insert("t6", 31);
	map.insert("x0", 0);
	map.insert("x1", 1);
	map.insert("x2", 2);
	map.insert("x3", 3);
	map.insert("x4", 4);
	map.insert("x5", 5);
	map.insert("x6", 6);
	map.insert("x7", 7);
	map.insert("x8", 8);
	map.insert("x9", 9);
	map.insert("x10", 10);
	map.insert("x11", 11);
	map.insert("x12", 12);
	map.insert("x13", 13);
	map.insert("x14", 14);
	map.insert("x15", 15);
	map.insert("x16", 16);
	map.insert("x17", 17);
	map.insert("x18", 18);
	map.insert("x19", 19);
	map.insert("x20", 20);
	map.insert("x21", 21);
	map.insert("x22", 22);
	map.insert("x23", 23);
	map.insert("x24", 24);
	map.insert("x25", 25);
	map.insert("x26", 26);
	map.insert("x27", 27);
	map.insert("x28", 28);
	map.insert("x29", 29);
	map.insert("x30", 30);
	map.insert("x31", 31);
	let reg = map;
	
	let re = Regex::new(r"\ *#.*").unwrap();
	let le = Regex::new(r"(\ *)([a-zA-Z]+)(:)").unwrap();
	for line in reader.lines() {
		let rline = line.unwrap();
		#[allow(unused_assignments)]
		let mut line: String = String::new();

		if let Some(caps) = le.captures(&rline) {
			label.push(((&caps[2]).to_string(), addr_count));
			continue;
		}

		if let Some(caps) = re.captures(&rline) {
			line = rline.replace(caps.get(0).unwrap().as_str(), "");
		} else {
			line = rline;
		}

		let spl: Vec<&str> = line.split(' ').collect();
		let mut sv: Vec<&str> = vec![spl[0]];
		if spl.len() > 1 {
			let mut opr: Vec<&str> = spl[1].split(',').collect();
			sv.append(&mut opr);
		}

		if sv.len() > 4 {
			panic!("sv");
		}

		// pseudo instruction predecode
		match sv[0] {
			"J" | "j" => {
				// j offset -> jal x0,offset
				sv[0]	=	"jal";
				sv[2]	=	sv[1];
				sv[1]	=	"x0";
			},
			"JR" | "jr" => {
				// jr rs -> jalr x0,rs,0
				sv[0]	=	"jalr";
				sv[2]	=	sv[1];
				sv[1]	=	"x0";
				sv.push("0");
			},
			"BEQZ" | "beqz" => {
				// beqz rs,offset -> beq rs,x0,offset
				sv[0]	=	"beq";
				sv.push(sv[2]);
				sv[2]	=	"x0";
			},
			"BNEZ" | "bnez" => {
				// bnez rs,offset -> bne rs,x0,offset
				sv[0]	=	"bne";
				sv.push(sv[2]);
				sv[2]	=	"x0";
			},
			"BGT" | "bgt" => {
				// bgt rs,rt,offset -> blt rt,rs,offset
				sv[0]	=	"blt";
				let tmp = sv[1];
				sv[1]	=	sv[2];
				sv[2]	=	tmp;
			},
			"BLE" | "ble" => {
				// ble rs,rt,offset -> bge rt,rs,offset
				sv[0]	=	"bge";
				let tmp = sv[1];
				sv[1]	=	sv[2];
				sv[2]	=	tmp;
			},
			"LI" | "li" => {
				// li rd,imm -> addi rd,x0,imm
				sv[0]	=	"addi";
				sv.push(sv[2]);
				sv[2]	=	"x0";
			},
			"MV" | "mv" => {
				// mv rd,rs -> addi rd, rs,0
				sv[0]	=	"addi";
				sv.push("0");
			},
			"" => {continue;}
			_ => {},
		}

		match sv[0] {
			"LUI" | "lui" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x37;
					let rd: u32 = reg[sv[1]] << 7;
					let imm: u32 = sv[2].parse::<u32>().unwrap() & 0xFFFFF000;

					let bin = imm+rd+op;
					//println!("{:>08x}",bin);
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"AUIPC" | "auipc" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x17;
					let rd: u32 = reg[sv[1]] << 7;
					let imm: u32 = sv[2].parse::<u32>().unwrap() & 0xFFFFF000;

					let bin = imm+rd+op;
					//println!("{:>08x}",bin);
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"ADD" | "add" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x0 << 12;
					let funct7: u32 = 0x0 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					//println!("{:>08x}",bin);
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SUB" | "sub" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x0 << 12;
					let funct7: u32 = 0x20 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"OR" | "or" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x6 << 12;
					let funct7: u32 = 0x0 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"AND" | "and" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x7 << 12;
					let funct7: u32 = 0x0 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"XOR" | "xor" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x4 << 12;
					let funct7: u32 = 0x0 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SLL" | "sll" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x1 << 12;
					let funct7: u32 = 0x0 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SRL" | "srl" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x5 << 12;
					let funct7: u32 = 0x0 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SRA" | "sra" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x5 << 12;
					let funct7: u32 = 0x20 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SLT" | "slt" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x2 << 12;
					let funct7: u32 = 0x0 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SLTU" | "sltu" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x3 << 12;
					let funct7: u32 = 0x0 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},

			"ADDI" | "addi" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x0 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"ORI" | "ori" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x6 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"ANDI" | "andi" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x7 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"XORI" | "xori" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x4 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SLLI" | "slli" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x1 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SRLI" | "srli" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x5 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SRAI" | "srai" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x5 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let shumt = imm << 20;

					let funct7: u32 = 0x20 << 25;

					let bin = funct7+shumt+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SLTI" | "slti" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x2 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SLTIU" | "sltiu" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x13;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x3 << 12;
					let imm: u32 = sv[3].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},

			"BEQ" | "beq" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x63;
					let rs1: u32 = reg[sv[1]] << 15;
					let rs2: u32 = reg[sv[2]] << 20;

					let funct3: u32 = 0x0 << 12;
					let imm: u32 = if let Some((_, i)) = label.clone().iter().find(|(x, _)| x == sv[3]) {
						*i
					} else {
						sv[3].parse().unwrap()
					};
					//let imm: u32 = sv[3].parse().unwrap();

					let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
					let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"BNE" | "bne" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x63;
					let rs1: u32 = reg[sv[1]] << 15;
					let rs2: u32 = reg[sv[2]] << 20;

					let funct3: u32 = 0x1 << 12;
					let imm: u32 = if let Some((_, i)) = label.clone().iter().find(|(x, _)| x == sv[3]) {
						*i
					} else {
						sv[3].parse().unwrap()
					};
					//let imm: u32 = sv[3].parse().unwrap();

					let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
					let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"BLT" | "blt" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x63;
					let rs1: u32 = reg[sv[1]] << 15;
					let rs2: u32 = reg[sv[2]] << 20;

					let funct3: u32 = 0x4 << 12;
					let imm: u32 = if let Some((_, i)) = label.clone().iter().find(|(x, _)| x == sv[3]) {
						*i
					} else {
						sv[3].parse().unwrap()
					};
					//let imm: u32 = sv[3].parse().unwrap();

					let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
					let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"BGE" | "bge" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x63;
					let rs1: u32 = reg[sv[1]] << 15;
					let rs2: u32 = reg[sv[2]] << 20;

					let funct3: u32 = 0x5 << 12;
					let imm: u32 = if let Some((_, i)) = label.clone().iter().find(|(x, _)| x == sv[3]) {
						*i
					} else {
						sv[3].parse().unwrap()
					};
					//let imm: u32 = sv[3].parse().unwrap();

					let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
					let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"BLTU" | "bltu" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x63;
					let rs1: u32 = reg[sv[1]] << 15;
					let rs2: u32 = reg[sv[2]] << 20;

					let funct3: u32 = 0x6 << 12;
					let imm: u32 = if let Some((_, i)) = label.clone().iter().find(|(x, _)| x == sv[3]) {
						*i
					} else {
						sv[3].parse().unwrap()
					};
					//let imm: u32 = sv[3].parse().unwrap();

					let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
					let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"BGEU" | "bgeu" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x63;
					let rs1: u32 = reg[sv[1]] << 15;
					let rs2: u32 = reg[sv[2]] << 20;

					let funct3: u32 = 0x7 << 12;
					let imm: u32 = if let Some((_, i)) = label.clone().iter().find(|(x, _)| x == sv[3]) {
						*i
					} else {
						sv[3].parse().unwrap()
					};
					//let imm: u32 = sv[3].parse().unwrap();

					let rd: u32 = ((imm & 0x1E) + ((imm & 0x800) >> 12)) << 7;
					let funct7: u32 = (((imm & 0x1E) >> 5) + ((imm & 0x1000) >> 5)) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},

			"LW" | "lw" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x3;
					let rd: u32 = reg[sv[1]] << 7;

					let tmp1: Vec<&str> = sv[2].split('(').collect();
					let tmp2: Vec<&str> = tmp1[1].split(')').collect();
					let rs1: u32 = reg[tmp2[0]] << 15;

					let funct3: u32 = 0x2 << 12;
					let imm: u32 = tmp1[0].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"LH" | "lh" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x3;
					let rd: u32 = reg[sv[1]] << 7;

					let tmp1: Vec<&str> = sv[2].split('(').collect();
					let tmp2: Vec<&str> = tmp1[1].split(')').collect();
					let rs1: u32 = reg[tmp2[0]] << 15;

					let funct3: u32 = 0x1 << 12;
					let imm: u32 = tmp1[0].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"LB" | "lb" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x3;
					let rd: u32 = reg[sv[1]] << 7;

					let tmp1: Vec<&str> = sv[2].split('(').collect();
					let tmp2: Vec<&str> = tmp1[1].split(')').collect();
					let rs1: u32 = reg[tmp2[0]] << 15;

					let funct3: u32 = 0x0 << 12;
					let imm: u32 = tmp1[0].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"LHU" | "lhu" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x3;
					let rd: u32 = reg[sv[1]] << 7;

					let tmp1: Vec<&str> = sv[2].split('(').collect();
					let tmp2: Vec<&str> = tmp1[1].split(')').collect();
					let rs1: u32 = reg[tmp2[0]] << 15;

					let funct3: u32 = 0x5 << 12;
					let imm: u32 = tmp1[0].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"LBU" | "lbu" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x3;
					let rd: u32 = reg[sv[1]] << 7;

					let tmp1: Vec<&str> = sv[2].split('(').collect();
					let tmp2: Vec<&str> = tmp1[1].split(')').collect();
					let rs1: u32 = reg[tmp2[0]] << 15;

					let funct3: u32 = 0x4 << 12;
					let imm: u32 = tmp1[0].parse().unwrap();
					let imms = imm << 20;

					let bin = imms+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SW" | "sw" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x23;
					let rs2: u32 = reg[sv[1]] << 20;

					let tmp1: Vec<&str> = sv[2].split('(').collect();
					let tmp2: Vec<&str> = tmp1[1].split(')').collect();
					let rs1: u32 = reg[tmp2[0]] << 15;

					let funct3: u32 = 0x2 << 12;
					let imm: u32 = tmp1[0].parse().unwrap();
					let rd: u32 = (imm & 0x1F) << 7;

					let funct7: u32 = (imm >> 5) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SH" | "sh" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x23;
					let rs2: u32 = reg[sv[1]] << 20;

					let tmp1: Vec<&str> = sv[2].split('(').collect();
					let tmp2: Vec<&str> = tmp1[1].split(')').collect();
					let rs1: u32 = reg[tmp2[0]] << 15;

					let funct3: u32 = 0x1 << 12;
					let imm: u32 = tmp1[0].parse().unwrap();
					let rd: u32 = (imm & 0x1F) << 7;

					let funct7: u32 = (imm >> 5) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"SB" | "sb" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x23;
					let rs2: u32 = reg[sv[1]] << 20;

					let tmp1: Vec<&str> = sv[2].split('(').collect();
					let tmp2: Vec<&str> = tmp1[1].split(')').collect();
					let rs1: u32 = reg[tmp2[0]] << 15;

					let funct3: u32 = 0x0 << 12;
					let imm: u32 = tmp1[0].parse().unwrap();
					let rd: u32 = imm & 0x1F << 7;

					let funct7: u32 = (imm >> 5) << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},

			"JAL" | "jal" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x6F;
					let rd: u32 = reg[sv[1]] << 7;

					let imm: u32 = if let Some((_, i)) = label.clone().iter().find(|(x, _)| x == sv[2]) {
						*i
					} else {
						sv[2].parse().unwrap()
					};
					//let imm: u32 = sv[2].parse().unwrap();

					let imm1: u32 = imm & 0xFF000;
					let imm2: u32 = ((imm & 0x7FE) + ((imm & 0x800) >> 10) + ((imm & 0x100000) >> 9)) << 20;

					let bin = imm2+imm1+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"JALR" | "jalr" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) {
					let op: u32 = 0x67;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;

					let funct3: u32 = 0x0 << 12;
					let imm: u32 = if let Some((_, i)) = label.clone().iter().find(|(x, _)| x == sv[3]) {
						*i
					} else {
						sv[3].parse().unwrap()
					};
					//let imm: u32 = sv[3].parse().unwrap();

					let imm1: u32 = imm << 20;

					let bin = imm1+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"CSRRW" | "csrrw" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x73;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[3]] << 15;

					let funct3: u32 = 0x1 << 12;
					let c: u32 = sv[2].parse().unwrap();
					let csr: u32 = c << 20;

					let bin = csr+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"CSRRS" | "csrrs" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x73;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[3]] << 15;

					let funct3: u32 = 0x2 << 12;
					let c: u32 = sv[2].parse().unwrap();
					let csr: u32 = c << 20;

					let bin = csr+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"CSRRC" | "csrrc" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x73;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[3]] << 15;

					let funct3: u32 = 0x3 << 12;
					let c: u32 = sv[2].parse().unwrap();
					let csr: u32 = c << 20;

					let bin = csr+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"CSRRWI" | "csrrwi" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x73;
					let rd: u32 = reg[sv[1]] << 7;

					let i: u32 = sv[3].parse().unwrap();
					let imm: u32 = i << 15;

					let funct3: u32 = 0x5 << 12;
					let c: u32 = sv[2].parse().unwrap();
					let csr: u32 = c << 20;

					let bin = csr+imm+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"CSRRSI" | "csrrsi" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x73;
					let rd: u32 = reg[sv[1]] << 7; 

					let i: u32 = sv[3].parse().unwrap();
					let imm: u32 = i << 15;

					let funct3: u32 = 0x6 << 12;
					let c: u32 = sv[2].parse().unwrap();
					let csr: u32 = c << 20;

					let bin = csr+imm+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"CSRRCI" | "csrrci" => {
				if reg.contains_key(sv[1]) {
					let op: u32 = 0x73;
					let rd: u32 = reg[sv[1]] << 7;

					let i: u32 = sv[3].parse().unwrap();
					let imm: u32 = i << 15;

					let funct3: u32 = 0x7 << 12;
					let c: u32 = sv[2].parse().unwrap();
					let csr: u32 = c << 20;

					let bin = csr+imm+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},

			"MUL" | "mul" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x0 << 12;
					let funct7: u32 = 0x1 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"MULH" | "mulh" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x1 << 12;
					let funct7: u32 = 0x1 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"MULHSU" | "mulhsu" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x2 << 12;
					let funct7: u32 = 0x1 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"MULHU" | "mulhu" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x3 << 12;
					let funct7: u32 = 0x1 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"DIV" | "div" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x4 << 12;
					let funct7: u32 = 0x1 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"DIVU" | "divu" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x5 << 12;
					let funct7: u32 = 0x1 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"REM" | "rem" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x6 << 12;
					let funct7: u32 = 0x1 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},
			"REMU" | "remu" => {
				if reg.contains_key(sv[1]) && reg.contains_key(sv[2]) && reg.contains_key(sv[3]) {
					let op: u32 = 0x33;
					let rd: u32 = reg[sv[1]] << 7;
					let rs1: u32 = reg[sv[2]] << 15;
					let rs2: u32 = reg[sv[3]] << 20;

					let funct3: u32 = 0x7 << 12;
					let funct7: u32 = 0x1 << 25;

					let bin = funct7+rs2+rs1+funct3+rd+op;
					let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
				}
			},

			"NOP" | "nop" => {
				let op: u32 = 0x13;
				let rd: u32 = 0 << 7;
				let rs1: u32 = 0 << 15;

				let funct3: u32 = 0x0 << 12;
				let imm: u32 = 0x0 << 20;
				let bin = imm+rs1+funct3+rd+op;
				let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
			},
			"HALT" | "halt" => {
				let op: u32 = 0x7F;
				let bin = op;
				let s = format!("{:>08x}\n", bin);
					let f: &[u8] = s.as_bytes();
					writer.write_all(f);
			},

			"" => {
				continue;
			}
			_ => {

			}
		}
		addr_count += 4;
	}
}