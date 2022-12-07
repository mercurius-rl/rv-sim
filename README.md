RISC-V Simulator written in Rust lang
===============================

- ver.0.1

This is a simple RISC-V simulator and RISC-V Assembler tools.
This simulator supports executing part of RV32I.

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

addi a0 zero 2
addi a1 a0 3
addi a2 zero 10
sub a3 a2 a1
addi a3 a3 1
slli a4 a3 2
sw a4 256(zero)
halt
```

Execution
======================================
```
cargo run "asm source file" "Options"
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

Update history
======================================
2022/07/13:  Development started in Youtube broadcast.  
2022/11/09:  Implemented Simulator and Assembler function.  
2022/11/13:  First commit version 0.1.  
2022/12/07:  Implement CSR instruction.  
