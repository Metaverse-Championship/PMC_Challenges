#!/usr/bin/env python3

from math import lcm
from gensafeprime import *
from Crypto.Util.number import *
from flag import flag

def keygen(nbit):
	p, q = [generate(nbit) for _ in '01']
	n, e, phi = p * q, 65537, lcm(p - 1, q - 1)
	d = inverse(e, phi)
	return n, e, d

def encrypt(n, e, d, m):
	c1 = pow(n + 1, d >> 3, n ** 2)
	c2 = pow(m, e, n)
	return (c1, c2)

flag = bytes_to_long(flag)
print (flag)
n, e, d = keygen(512)
c1, c2 = encrypt(n, e, d, flag)

print(f'n = {n}')
print(f'c1 = {c1}')
print(f'c2 = {c2}')

#validity = '[OK]' if (c1 - 1) % n == 0 else '[Not OK!]'
#d3 = (c1 - 1) // n 
#....
