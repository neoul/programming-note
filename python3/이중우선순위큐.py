#!/usr/bin/env python3

# https://programmers.co.kr/learn/courses/30/lessons/42628

import heapq

def solution(operations):
    # need to search n/2 elements to delete maximum value.
    # min heap에서 max value를 찾아지우기 위해선
    # 하위 n/2 요소를 모두 검사해야 함.
    pque = []
    for op in operations:
        # print(op)
        if op.startswith("I"):
            o = op.split(" ")
            heapq.heappush(pque, -int(o[1]))
        elif op == "D 1":
            if pque:
                heapq.heappop(pque)
        elif op == "D -1":
            if pque:
                m = 0
                for i in range(len(pque)//2, len(pque)):
                    if pque[i] > pque[m]:
                        m = i
                del pque[m]
        # print(pque)
    # print(pque)
    if len(pque) == 0:
        return 0,0
    m = 0
    for i in range(len(pque)//2, len(pque)):
        if pque[i] > pque[m]:
            m = i
    return -(pque[0]), -(pque[m])

print(solution(["I 16","D 1"])) # [0,0]
print(solution(["I 7","I 5","I -5","D -1"])) # [7,5]
print(solution(["I 16", "I -5643", "D -1", "D 1", "D 1", "I 123", "D -1"])) # [0,0]
print(solution(["I -45", "I 653", "D 1", "I -642", "I 45", "I 97", "D 1", "D -1", "I 333"])) # [333, -45]
print(solution(["I 2","I 4","D -1", "I 1", "D 1"]))