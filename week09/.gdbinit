set architecture riscv:rv64
target remote localhost:1234
set sysroot /usr/riscv64-linux-gnu

set auto-load safe-path /
set debuginfod enabled off
tui new-layout asm {-horizontal src 1 regs 1} 2 status 0 cmd 1
tui enable
layout asm
