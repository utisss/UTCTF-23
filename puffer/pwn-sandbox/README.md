# Source for this challenge has deliberately not been included in the initial backup

# UTCTF Sandbox
This is an easy pwn problem involving a custom sandboxing solution. The
sandboxed binary (hello) is run within a unicorn emulator and syscalls are 
hooked and performed on its behalf. 

The key to solving this challenge is noticing that there is an added syscall
at rax==1024. This syscall accepts a pointer through %rdi and writes 4 bytes
to the corresponding address, corresponding to how many syscalls the user has
run so far. 

The intended solution is to use this syscall to write to the exit_syscalls
global array within the sandbox loader. The value of %rdi is assumed to be 
within the stack, so there is an integer overflow possible to write to an
arbitrary address. The attacker is able to execute a bunch of dummy syscalls
using ROP to increase the syscall count to be equal to the syscall number of 
execve(59). 

Now, the attacker can overwrite an element of the exit_syscalls array and
execute the execve syscall using ROP. The values of rdi, and rsi, and rdx
are patched right through. 
