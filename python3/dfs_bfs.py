#!/usr/bin/env python3

# REF: https://codereview.stackexchange.com/questions/247368/depth-first-search-using-stack-in-python

from typing import Any, Iterator
from collections import deque

Stack = list[Any]


def depth_first_search(graph, start) -> Iterator:
    stack: Stack = [start]
    visited = set()
    while stack:
        vertex = stack.pop()
        if vertex in visited:
            continue
        yield vertex
        visited.add(vertex)
        for neighbor in graph[vertex]:
            stack.append(neighbor)

def breadth_first_search(graph, start) -> Iterator:
    queue = deque()
    queue.append(start)
    visited = set()
    while queue:
        vertex = queue.popleft()
        if vertex in visited:
            continue
        yield vertex
        visited.add(vertex)
        for neighbor in graph[vertex]:
            queue.append(neighbor)

def main():
    matrix = {
        1: [2, 3],
        2: [4, 5],
        3: [5],
        4: [6],
        5: [6],
        6: [7],
        7: []
    }
    print("dfs")
    dfs_path = depth_first_search(matrix, 1)
    for e in dfs_path:
        print(e)
    print("bfs")
    bfs_path = breadth_first_search(matrix, 1)
    for e in bfs_path:
        print(e)


if __name__ == '__main__':
    main()
