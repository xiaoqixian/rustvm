# !/usr/bin/python3
# -*- coding: utf-8 -*-
# > Author          : lunar
# > Email           : lunar_ubuntu@qq.com
# > Created Time    : Fri 05 Nov 2021 03:41:58 PM CST
# > Location        : Shanghai
# > Copyright@ https://github.com/xiaoqixian

# a = [1,2,3,4,5,7,8,9]
# b = [1,3,5]
# print a[:]
# print a[1:]
# print a[:2]
# print a[1:4]
# print a[b]

a = {1: 'a', 2: 'b'}
print a
print a[1]

class Own:
    def __init__(self, a, b):
        self.a = a
        self.b = b

o1 = Own(1, 2)
o2 = Own(3, 4)

a[o1] = o2
print a[o1].b

# """
# Slice0: 30
# Slice1: 31
# Slice2: 32
# Slice3: 33
# """

