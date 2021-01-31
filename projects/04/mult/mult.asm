// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.

(BEGIN)
    @R2
    M=0
    @R0
    D=M
    @END
    D;JEQ
    @R1
    D=M
    @i
    M=D
    @s
    @POSITIVE
    D;JGT
    @NEGATIVE
    D;JLT
    @END
    D;JGT

(RESUME)    

(LOOP)
    @i
    D=M
    @ADD
    D;JGT
    @OUTPUTCHECK
    0;JMP

(ADD)
    @R0
    D=M
    @i
    M=M-1
    @R2
    M=D+M
    @LOOP
    0;JMP

(POSITIVE)
    @s
    M=1
    @RESUME
    0;JMP

(NEGATIVE)
    @s
    M=-1
    @i
    D=M
    M=-D
    @RESUME
    0;JMP

(OUTPUTCHECK)
    @s
    D=M
    @MAKEITNEGATIVE
    D;JLT
    @END
    0;JMP

(MAKEITNEGATIVE)
    @R2
    M=-M
    @END
    0;JMP

(END)
    @END
    0;JMP
