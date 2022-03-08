#!/usr/bin/env python3

import heapq

def solution(scoville, K):
    mixedcnt = 0
    heapq.heapify(scoville)
    while scoville:
        if scoville[0] < K:
            if len(scoville) < 2:
                return -1
            food = heapq.heappop(scoville)
            food2 = heapq.heappop(scoville)
            mixed = food + food2 * 2
            heapq.heappush(scoville, mixed)
            mixedcnt += 1
        else:
            break
    if mixedcnt == 0 and scoville[0] < K:
        return -1
    return mixedcnt

print(solution([1, 2, 3, 9, 10, 12], 7)) # 2
