RISC-V Simulator written in Rust lang
===============================

- ver.0.4

This is a simple RISC-V simulator and RISC-V Assembler tools.
This simulator supports executing part of RV32IM.

License
========================================

Apache License (Version 2.0)  
http://www.apache.org/licenses/LICENSE-2.0  


Available Instruction
========================================
```text
lui
auipc
addi
slti
sltiu
xori
xori
ori
andi
slli
srli
srai
add
sub
sll
slt
sltu
xor
srl
sra
or
and
lb
lh
lw
lbu
lhu
sb
sh
sw
jal
jalr
beq
bne
blt
bge
bltu
bgeu
csrrw
csrrs
csrrc
csrrwi
csrrsi
csrrci

mul
mulh
mulhsu
mulhu
div
divu
rem
remu
```

### Available register names
```
 0: "zero" or "x0"
 1: "ra" or "x1"
 2: "sp" or "x2"
 3: "gp" or "x3"
 4: "tp" or "x4"
 5: "t0" or "x5"
 6: "t1" or "x6"
 7: "t2" or "x7"
 8: "fp" or "s0" or "x8"
 9: "s1" or "x9"
10: "a0" or "x10"
11: "a1" or "x11"
12: "a2" or "x12"
13: "a3" or "x13"
14: "a4" or "x14"
15: "a5" or "x15"
16: "a6" or "x16"
17: "a7" or "x17"
18: "s2" or "x18"
19: "s3" or "x19"
20: "s4" or "x20"
21: "s5" or "x21"
22: "s6" or "x22"
23: "s7" or "x23"
24: "s8" or "x24"
25: "s9" or "x25"
26: "s10" or "x26"
27: "s11" or "x27"
28: "t3" or "x28"
29: "t4" or "x29"
30: "t5" or "x30"
31: "t6" or "x31"
``` 

Sample
======================================

```text
a = 2 + 3
b = 10
c = b - a
c += 1
d = c << 2

```
Asm code to perform the above calculation and store it in memory at address 0x100 are shown in Code block 1.

```m68k:sample.asm
*code block 1*

addi a0,zero,2
addi a1,a0,3
addi a2,zero,10
sub a3,a2,a1
addi a3,a3,1
slli a4,a3,2
sw a4,256(zero)
halt
```

Execution
======================================
```
cargo run <asm source file> <Options>*
```

```
Positional Arguments:
  src

Options:
  -a, --asm         run assembler
  -o, --output      output assembled binary
  -s, --sim         run risc-v simulator
  -b, --bin         binary data file
  -d, --dbg         debug run
  -m, --memdump     memory dump (dump length)
  -c, --ex_count    count of maximum instruction execute
                    (default: 256)  if you want setting infinity count, set negative number for this parameter.
  --help            display usage information
```

For example, execute under command if source file name is test.asm and you want to execute debug mode and memory dump mode.
```
cargo run test.asm --dbg ---memdump 256
```
or
```
cargo run test.asm -d -m 256
```

How to read debug output
======================================

```
--Output of debug run--
*When -d or --dbg is set.*

Address[**addr**]: **binInst** (**inst** { **operand** })
reg: Reg { f: [**rfData**] }

addr    : Instruction memory address during read instruction.
binInst : Instruction binary.
inst    : Instruction name.
operand : inst's operand.
rfData  : Data stored in the register file.
```

```
--Output of memory dump--
*When -m or --memdump is set.*

[Memory dump mode]
Address[**maddr**   ]: **bin**
Address[**maddr** +1]: **bin**
・
・
・

maddr   : Data memory address.
bin     : Data binary.
```

## Interactive mode (Beta)
If you run without arguments, interactive mode starts.  
There is a dedicated command for interactive mode.
```
.info   : Show register's data.

.exit   : Exit interactive mode.

.faddr  : Executes the instruction in instruction memory 
          pointed to by the current program counter.

.addr++ : Increment the program counter.

.addr-- : Decrement the program counter.
          (However, when the value of the program counter is 4 or more)
```

Update history
======================================
2022/07/13:  Development started in Youtube broadcast.  
2022/11/09:  Implemented Simulator and Assembler function.  
2022/11/13:  First commit version 0.1.  
2022/12/07:  Implemented CSR instruction.  
2023/04/15:  Implemented the setting of maximum execution count.  
2023/04/22:  Implemented Multiplicaton instruction (ver. 0.2).  
2023/04/27:  Fixed assembler parser and sample code.  
2023/04/29:  Add and Implement Interactive mode(Beta, ver. 0.3).  
2023/05/31:  Add library api.  
2023/08/19:  Add ecall, ebreak and (u s m)ret instruction.  
