from math import lcm

flag = 'utflag{L0c4l1z3d_Ch1ck3n_M0d1f1c4t10N_g8h91b3h89}'
flag = [ord(x) for x in flag]
n = len(flag)
mat = [[0 for _ in range(n)]for _ in range(n)]

for i in range(n):
    for j in range(n):
        l = lcm(flag[i], flag[j]) ^ flag[i] ^ flag[j]
        # print(i, j, l ^ flag[i] ^ flag[j])
        if(i == j):
            l = 0
        mat[i][j] = l

m = {}

for i in range(128):
    for j in range(i, 128):
        l = lcm(i, j) ^ i ^ j
        if l not in m:
            m[l] = []
        m[l] += [(i,j)]

print(str(mat).replace('[','{').replace(']','}'))
print(n)

a = ord('s')
b = ord('t')
print(lcm(a,b))
