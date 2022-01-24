#!/usr/bin/env python3
import sys
import heapq
from typing import Any
INF = sys.maxsize

def dijkstra(graph: list[Any], start: int):
    distance = [INF] * len(graph)
    distance[start] = 0 # set starting vertex
    S = [] # visited node
    Q = [i for i in range(len(graph))]
    while Q:
        curv = 0 # current vertex
        curw = INF # current weight
        for q in Q:
            if distance[q] < curw:
                curw = distance[q]
                curv = q
        Q.remove(curv)
        S.append(curv)
        for v, weight in enumerate(graph[curv]):
            if weight >= INF: # skip not connected one
                continue
            alt = distance[curv] + weight
            if alt < distance[v]:
                distance[v] = alt
    # S로 특정 end vertex까지의 path 계산 가능
    return distance, S

def dijkstra_prioque(graph: list[Any], start: int):
    distance = [INF] * len(graph)
    distance[start] = 0 # set starting vertex
    S = []
    Q = []
    heapq.heappush(Q, start)
    while Q:
        curv = heapq.heappop(Q)
        S.append(curv)
        for nextv, nextw in enumerate(graph[curv]):
            if nextw >= INF: # skip not connected one
                continue
            alt = distance[curv] + nextw # next weight
            if alt < distance[nextv]:
                distance[nextv] = alt
                heapq.heappush(Q, nextv)
    # S로 특정 end vertex까지의 path 계산 가능
    return distance, S

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
    print(dijkstra(graph, 0))
    print(dijkstra(graph, 1))
    print(dijkstra_prioque(graph, 0))
    print(dijkstra_prioque(graph, 1))
    

if __name__ == '__main__':
    main()

    # print(solution([1,2,3,1]), 4)
    # print(solution([1,1,4,1,4]), 8)
    # print(solution([1000,0,0,1000,0,0,1000,0,0,1000]), 3000)
    # print(solution([1000,1,0,1,2,1000,0]), 2001)
    # print(solution([1000,0,0,0,0,1000,0,0,0,0,0,1000]), 2000)
    # print(solution([1,2,3,4,5,6,7,8,9,10]), 30)
    # print(solution([0,0,0,0,100,0,0,100,0,0,1,1]), 201)
    # print(solution([11,0,2,5,100,100,85,1]), 198)
    # print(solution([1,2,3]), 3)
    # print(solution([91, 90, 5, 7, 5, 7]), 104)
    # print(solution([90,0,0,95,1,1]), 185)