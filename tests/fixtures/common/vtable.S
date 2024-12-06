.syntax unified

@ Necessary so the tests can run for the microbit emulator. 
.section .rodata.vtable
.word _stack_end
.word main @ Used instead of Reset Handler
.rept 126
.word 0 
.endr

