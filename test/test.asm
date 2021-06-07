    .equ	CONSTANT, 0xdeadbeef
    .equ	UART_BASE, 0x40003080

loop:
    j loop

1:
	auipc	a0, %pcrel_hi(msg + 1)
	addi	a0, a0, %pcrel_lo(1b)

	lui	a0, %hi(msg + 1)
	addi	a0, a0, %lo(msg + 1)

    lw	a0, var1
	fld	fa0, var2, t0
	sw	a0, var3, t0
	fsd	fa0, var4, t0

	li	a0, CONSTANT

	lui	a0, %hi(UART_BASE)
	addi	a0, a0, %lo(UART_BASE)
    call	func1
	tail	func2
	jump	func3, t0

    j 1b