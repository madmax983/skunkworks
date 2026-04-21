; Noise Generator
Loop:
PUSH 128
RND ; x
PUSH 128
RND ; y
PUSH 16
RND ; color
PLOT
JMP Loop
