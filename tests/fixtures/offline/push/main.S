.syntax unified

.global main
.type main, %function
main:
  mov r0, 0
  mov r1, 1
  mov r2, 2
  mov r3, 3
  mov r4, 4
  mov r5, 5
  mov r6, 6
  mov r7, 7
  mov r8, 8
  mov r9, 9
  mov r10, 10
  mov r11, 11
  mov r12, 12
  mov lr, 14

  @ PUSH T1
  ldr sp, =_stack_end
  push.N {r0, r3, r7, lr}

  @ PUSH T2
  ldr sp, =_stack_end
  push.W {r2-r8, r10-r12, lr}

  @ PUSH T3
  ldr sp, =(_stack_end - 1)
  push.W {lr}
.size main, . - main

.section .isr_vector, "a", %progbits
g_pfnVectors:
.word _stack_end
.word main
