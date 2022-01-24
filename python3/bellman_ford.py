#!/usr/bin/env python3
import sys
import heapq
from typing import Any
INF = sys.maxsize

# function BellmanFord(list vertices, list edges, vertex source) is
#     // This implementation takes in a graph, represented as
#     // lists of vertices (represented as integers [0..n-1]) and edges,
#     // and fills two arrays (distance and predecessor) holding
#     // the shortest path from the source to each vertex
#     distance := list of size n
#     predecessor := list of size n

#     // Step 1: initialize graph
#     for each vertex v in vertices do
#         distance[v] := inf             // Initialize the distance to all vertices to infinity
#         predecessor[v] := null         // And having a null predecessor
#     distance[source] := 0              // The distance from the source to itself is, of course, zero

#     // Step 2: relax edges repeatedly
#     repeat |V|−1 times:
#          for each edge (u, v) with weight w in edges do
#              if distance[u] + w < distance[v] then
#                  distance[v] := distance[u] + w
#                  predecessor[v] := u

#     // Step 3: check for negative-weight cycles
#     for each edge (u, v) with weight w in edges do
#         if distance[u] + w < distance[v] then
#             error "Graph contains a negative-weight cycle"

#     return distance, predecessor

def bellman_ford(graph: list[Any], start: int):
    distance = [INF] * len(graph)
    distance[start] = 0 # set starting vertex
    predecessor = [None] * len(graph)
    for _ in range(len(graph) -1):
        for u in range(len(graph)):
            for v, w in enumerate(graph[u]):
                if w >= INF: # skip not connected
                    continue
                # print((v,u), w)
                if distance[u] + w < distance[v]:
                    distance[v] = distance[u] + w
                    predecessor[v] = u
                    
    for u in range(len(graph)):
        for v, w in enumerate(graph[u]):
            if distance[u] + w < distance[v]:
                raise Exception(f"Graph contains a negative-weight cycle in %d %d" %(v, u))
    return distance, predecessor

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
    print(bellman_ford(graph, 0))

    graph = [ # 음 가중치가 크지 않을 경우
        # E(4) -> B(1)로 -5 가중치 부여
        [0, 10, 30, 15, INF, INF],
        [INF, 0, INF, INF, 20, INF],
        [INF, INF, 0, INF, INF, 5],
        [INF, INF, 5, 0, INF, 20],
        [INF, -5, INF, INF, INF, 20],
        [INF, INF, INF, 20, INF, 0]
    ]
    print(bellman_ford(graph, 0))
    
    graph = [ # 음 가중치가 커 발산할 경우
        # E(4) -> B(1)로 -30 가중치 부여
        [0, 10, 30, 15, INF, INF],
        [INF, 0, INF, INF, 20, INF],
        [INF, INF, 0, INF, INF, 5],
        [INF, INF, 5, 0, INF, 20],
        [INF, -30, INF, INF, INF, 20],
        [INF, INF, INF, 20, INF, 0]
    ]
    print(bellman_ford(graph, 0))

if __name__ == '__main__':
    main()
