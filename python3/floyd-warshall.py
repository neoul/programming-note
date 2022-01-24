#!/usr/bin/env python3
import sys
import heapq
from typing import Any
import copy
# INF = sys.maxsize
INF = 10000

def floyd_warshall(graph: list[Any]):
    distance = copy.deepcopy(graph)
    for m in range(len(distance)):
        for s in range(len(distance)):
            for e in range(len(distance)):
                print(m, s, e, distance[s][e], distance[s][m], distance[m][e])
                if distance[s][e] > distance[s][m] + distance[m][e]:
                    distance[s][e] = distance[s][m] + distance[m][e]
                    print("updated", m, s, e)
    return distance

def main():
    # 다음 예제 2차 배열
    # https://namu.wiki/w/%EB%8B%A4%EC%9D%B5%EC%8A%A4%ED%8A%B8%EB%9D%BC%20%EC%95%8C%EA%B3%A0%EB%A6%AC%EC%A6%98
    # 0:A, 1:B, 2:C, 3:D, 4:E, 5:F
    graph = [
        [0, 10, 30, 15, INF, INF],
        [INF, 0, INF, INF, 20, INF],
        [INF, INF, 0, INF, INF, 5],
        [INF, INF, 5, 0, INF, 20],
        [INF, INF, INF, INF, INF, 20],
        [INF, INF, INF, 20, INF, 0]
    ]
    print(floyd_warshall(graph))    

if __name__ == '__main__':
    main()
