#!/usr/bin/env python3

# def insertAndSort(l, e):
#     for i in range(1, len(l)):
#         if e < l[i]:

import heapq

def solution(prices):
    r = [0 for i in range(len(prices))]
    queue = []
    for i in range(0, len(prices)-1):
        heapq.heappush(queue, [-prices[i], i, 0])
        # print(queue)
        if len(queue) > 0:
            while -queue[0][0] > prices[i]:
                p = heapq.heappop(queue)
                # print("pop", p)
                r[p[1]] = p[2]
            for j in range(len(queue)):
                queue[j][2] += 1
            # print(queue, prices[i])
    for j in range(len(queue)):
        p = queue[j]
        r[p[1]] = p[2]
    # print(r)
    return r


solution([1, 2, 3, 2, 3]) # [4, 3, 1, 1, 0]
solution([1, 3, 5, 7, 9, 4, 5, 2, 1, 0]) # [9, 6, 3, 2, 1, 2, 1, 1, 1, 0] 
