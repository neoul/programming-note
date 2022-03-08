#!/usr/bin/env python3

# https://programmers.co.kr/learn/courses/30/lessons/42627

import heapq

def solution(jobs):    
    # 가정: 가장 짧근 소요시간의 작업을 가장 먼저 처리해야 평균작업처리 시간이 짧다.
    # priority queue에 삽입하여 작업소요시간으로 정렬하고,
    # 짧은 소요시간의 작업을 꺼내 작업을 처리한다.
    # 평균을 구한다.
    jobs = sorted(jobs, key = lambda j: j[0])
    prique = []
    index = 0
    jobt = 0
    t = 0
    total = 0
    while True:
        while index < len(jobs):
            if jobs[index][0] == t:
                heapq.heappush(prique, [jobs[index][1], jobs[index][0]])
            else:
                break
            index += 1
        if jobt <= 0:
            while prique:
                job = heapq.heappop(prique)
                total = total + job[0] + (t - job[1])
                if job[0] >= 0:
                    jobt = job[0] - 1
                    break
        else:
            jobt -= 1
        t += 1
        if index >= len(jobs)  and jobt <= 0 and len(prique) <= 0:
            break
    return total//len(jobs)

print(solution([[0, 3], [1, 9], [2, 6]])) # 9
print(solution([[24, 10], [28, 39], [43, 20], [37, 5], [47, 22], [20, 47], [15, 34], [15, 2], [35, 43], [26, 1]])) # 72
print(solution([[0, 5], [2, 10], [10000, 2]])) # 6
