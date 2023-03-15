from pwn import *
from sage.all import *

MOD = 10**9 + 7
Z_N = Integers(MOD)
threshold = 24
def small(vals):
    for x in vals.list():
        if x > threshold and MOD-x >threshold:
            return False
    return True

def break_key(keys):
    rows = len(keys[0])
    cols = len(keys[0][0])

    for i_a in range(len(keys)):
        for i_b in range(len(keys)):
            if i_a == i_b:
                pass
            try:
                key1 = keys[i_a]
                key2 = keys[i_b]
                A = key1[:-1] + key2[:-1]
                s_TA = [key1[-1][i] - key2[-1][i] for i in range(cols)]
                A = matrix(Z_N, A)
                s_TA = matrix(Z_N, s_TA)
                sk = A.solve_left(s_TA)
                sk = [i for i in sk.list()]
                sk = sk[:rows-1] + [-1]
                sk = -matrix(Z_N, sk)
                A1 = matrix(Z_N, key1)
                if small(sk * A1):
                    return i_a+1, [i for i in sk.list()]
            except:
                pass
    print('could not solve :(')

def solve(r):

    for round in range(10):
        print(r.recvline())
        r.recvline()
        r.sendline(b'10')

        keys = []
        for i in range(10):
            r.recvline()
            keys.append(eval(r.recvline()))

        # index, sk = break_key(keys)
        index = 0
        sk = [0] * len(keys[0])

        print(r.recvline())
        r.sendline(b'%d' % index)

        print(r.recvline())
        r.sendline(' '.join([str(x) for x in sk]))

        print(r.recvline())
    r.interactive()

r = remote('puffer.utctf.live', 8484)
solve(r)
