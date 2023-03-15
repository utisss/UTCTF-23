#!/usr/bin/env python3
import secrets
import random
import math

def mat_mul(a, b, mod):
    c = [[0] * len(b[0]) for _ in range(len(a))]
    for i in range(len(a)):
        for j in range(len(b)):
            for k in range(len(b[0])):
                c[i][k] = (c[i][k] + a[i][j] * b[j][k]) % mod
    return c

def mat_sum(a, b, mod):
    c = [[0] * len(b[0]) for _ in range(len(a))]
    for i in range(len(a)):
        for j in range(len(b[0])):
            c[i][j] = (a[i][j] + b[i][j]) % mod
    return c


def rand_matrix(mod, size, sample_func=secrets.randbelow):
    data = [[sample_func(mod) for _ in range(size[1])] for _ in range(size[0])]
    return data

def gen_errors(num, width, mod, size):
    values = [i for i in range(-8*width, 8*width)]
    weights = [math.e ** (-math.pi * (i / width)**2)for i in values]
    def dg(mod):
        return random.choices(values, weights)[0] % mod
    return [rand_matrix(mod,size,dg) for _ in range(num)]

def check(array, mod, width):
    for x in array[0]:
        if not (x < 4 * width or mod-x < 4 * width):
            return False, False
    all_zero = True
    for x in array[0]:
        all_zero = x % mod == 0 and all_zero
    return True, all_zero

def keygen_many(num, width, mod, size):
    e_T = gen_errors(num, width, mod, (1,size[1]))

    keys = []
    for i in range(num):
        A_bar = rand_matrix(mod, size)
        s_bar = rand_matrix(mod, (1, size[0]))
        index = secrets.randbelow(num)
        A = A_bar + mat_sum(mat_mul(s_bar, A_bar, mod), e_T[index], mod)
        keys.append(A)
    return keys

any_all_zero = False

for r in range(1, 11):
    print('round %d / 10' % r)
    print('how many keys would you like? (1-10)')
    num = int(input())
    mod = 10**9+7
    width = 6
    size = (10*min(r, 5), 30*min(r,5))
    keys = keygen_many(num, width, mod, size)
    for i, key in enumerate(keys):
        print('Key %d' % (i+1))
        print(key)

    print('which key would you like to crack? (1-%d)' % num)
    index = int(input()) - 1
    print('enter the secret key (%d space separated integers)' % (size[0] + 1))

    values = input().split()

    secret_key = [[int(x) for x in values]]

    worked, all_zero = check(mat_mul(secret_key, keys[index], mod), mod, width)

    if worked:
        print('ok')
    else:
        print('looks wrong tom e :/')
        exit()

    any_all_zero = any_all_zero or all_zero

if any_all_zero:
        print('utflag{mY_l34Rn1Ng_h4s_3rr0rs_2f11a84e}')
else:
        print('utflag{learning_with_errors_part_2_writing_a_challenge_without_test_solving_:clown:}')
