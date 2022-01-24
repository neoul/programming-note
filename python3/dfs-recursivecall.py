#!/usr/bin/env python3

def trace(mm, nn, target, cache):
    p = (mm, nn)
    # print(p)
    if p == target:
        return 1
    memo = cache.get(p, 0)
    if memo > 0:
        # print(p, "found!", memo)
        return memo
    elif memo == 0:
        s = 0
        if nn+1 <= target[1]:
            cnt = trace(mm, nn+1, target, cache)
            if cnt < 0:
                cache[p] = -1
            elif cnt > 0:
                s = s + cnt
                cache[p] = cnt + cache.get(p, 0)
        if mm+1 <= target[0]:
            cnt = trace(mm+1, nn, target, cache)
            if cnt < 0:
                cache[p] = -1
            elif cnt > 0:
                s = s + cnt
                cache[p] = cnt + cache.get(p, 0)
        return s
    return 0

def solution(m, n, puddles):
    target = (m, n)
    # -1: no way, 0: not cache, 1: the num of reached
    cache={tuple(s):-1 for s in puddles}
    return trace(1, 1, target, cache) % 1000000007

print(solution(4,3, [[2,2]]))
