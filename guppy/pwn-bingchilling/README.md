# Bing Chilling
IMPORTANT: A small part of the challenge is finding out that the binary is 
loongarch. Please don't rob contestants of this discovery.

This is a simple stack overflow challenge on the loongarch64 ISA. The intended
solution is available in shellcode.txt and solution.py. 

If you're the officer on-call, don't give any hints for this challenge.

Basically:
| shellcode | padding to reach 68 bytes | return address (buf) |
