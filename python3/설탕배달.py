#!/usr/bin/env python3

# https://www.acmicpc.net/problem/2839
# 설탕배달

N=int(input())

def solution(N):
    
    # 5 * x + 3 * y
    m=N%10
    n=N//10
    a=0
    b=0
    if N in [1,2,4,7]:
        return -1, 0
    if m == 0 or m == 5:
        a=N//5
    elif m == 1 or m == 6:
        b=2
        a=(N - 3*b)//5
    elif m == 2 or m == 7:
        b=4
        a=(N - 3*b)//5
    elif m == 3 or m == 8:
        b=1
        a=(N - 3*b)//5
    elif m == 4 or m == 9:
        b=3
        a=(N - 3*b)//5
    else:
        return -1, 0
    return a,b
print(sum (solution(N)))
print(sum (solution(362)), 74)
print(sum (solution(18)), 4)
print(sum (solution(4)), -1)
print(sum (solution(6)), 2)
print(sum (solution(11)), 3)
