#!/usr/bin/env python3
import sys
import heapq
from typing import Any, Dict
INF = sys.maxsize


def dijkstra(graph: Dict[str, Any], start: str):
    distance = {v: INF for v in graph}
    distance[start] = 0 # set starting vertex
    S = []  # visited node
    Q = [i for i in graph]
    print(Q)
    while Q:
        curv = ''  # current vertex
        curw = INF  # current weight
        for q in Q: # get a vertex that has the minimum distance(weight)
            if distance[q] < curw:
                curw = distance[q]
                curv = q
        Q.remove(curv)
        S.append(curv)
        for v, weight in graph[curv].items():
            if weight >= INF:  # skip not connected one
                continue
            alt = distance[curv] + weight
            if alt < distance[v]:
                distance[v] = alt
    # S로 특정 end vertex까지의 path 계산 가능
    return distance, S


def dijkstra_prioque(graph: Dict[str, Any], start: str):
    distance = {v: INF for v in graph}
    distance[start] = 0 # set starting vertex
    S = []
    Q = []
    heapq.heappush(Q, start)
    while Q:
        curv = heapq.heappop(Q)
        S.append(curv)
        for nextv, nextw in graph[curv].items():
            # print(nextv, nextw)
            if nextw >= INF:  # skip not connected one
                continue
            alt = distance[curv] + nextw  # next weight
            if alt < distance[nextv]:
                distance[nextv] = alt
                heapq.heappush(Q, nextv)
    # S로 특정 end vertex까지의 path 계산 가능
    return distance, S


def main():
    # https://justkode.kr/algorithm/python-dijkstra
    graph = {
        'A': {'B': 8, 'C': 1, 'D': 2},
        'B': {},
        'C': {'B': 5, 'D': 2},
        'D': {'E': 3, 'F': 5},
        'E': {'F': 1},
        'F': {'A': 5}
    }
    print(dijkstra(graph, 'A'))
    print(dijkstra_prioque(graph, 'A'))


if __name__ == '__main__':
    main()
