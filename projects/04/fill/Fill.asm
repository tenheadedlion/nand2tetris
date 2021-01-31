// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(INITIATION)
    @24576
    D=A
    @SCROPENCLOSE
    M=D
    @color
    M=0
    @addr

(LISTEN)
    @KBD
    D=M
    @CLEAR
    D;JEQ
    @PAINTBLACK0
    0;JMP

(CLEAR)
    @color
    D=M
    @PAINTWHITE0
    D;JGT
    @LISTEN
    0;JGT

(PAINTWHITE0)
    @SCREEN
    D=A
    @addr
    M=D
    @PAINTWHITE
    0;JMP

(PAINTWHITE)
    @SCROPENCLOSE
    D=M
    @addr
    D=D-M
    @WHITEDONE
    D;JEQ

    @addr
    A=M
    M=0
    @addr
    M=M+1

    @PAINTWHITE
    0;JMP

(WHITEDONE)
    @color
    M=0
    @LISTEN
    0;JMP

(PAINTBLACK0)
    @SCREEN
    D=A
    @addr
    M=D
    @PAINTBLACK
    0;JMP

(PAINTBLACK)
    @SCROPENCLOSE
    D=M
    @addr
    D=D-M
    @BLACKDONE
    D;JEQ

    @addr
    A=M
    M=-1
    @addr
    M=M+1

    @PAINTBLACK
    0;JMP

(BLACKDONE)
    @isblack
    M=1
    @LISTEN
    0;JMP
