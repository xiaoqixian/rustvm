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

# """
# Slice0: 30
# Slice1: 31
# Slice2: 32
# Slice3: 33
# """

class Fuck:
    def __init__(self, a, b):
        self.a = a
        self.b = b

    def calc(self):
        return self.a + self.b

a = Fuck(1,2)
print a.calc()

print Fuck
