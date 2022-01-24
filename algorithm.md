# Algorithm note

## Basic Data Structure

- Stack
- Queue
- PQ (Priority Queue), Max Heap, Min Heap
- DFS (Depth First Search)
- BFS (Breadth First Search)

### 최단 경로 알고리즘

- Dijkstra Algorithm:
  - 그래프의 단일 노드에서 모든 노드로의 경로 탐색; O((V+E)lgV)
  - priority queue (heap)로 성능향상 가능
- Bellman-Ford-Moore Algorithm:
  - 가중 유향 그래프(Weighted-Directed Graph)에서 노드 사이의 최단 경로 탐색; O(VE)
  - 음수 가중치 사용가능, negative-weight cycles 유무 판단 (무한 발산하므로 ...)
  - 그래프의 단일 노드에서 모든 노드로의 경로 탐색
  - https://victorydntmd.tistory.com/104
- Floyd Warshall Algorithm:
  - 그래프의 모든 노드에서 모든 노드로의 경로 탐색 (O(V * V * V))
  - 임의의 노드 s에서 e까지 가는 데 걸리는 최단거리를 구하기 위해, s와 e 사이의 노드인 m에 대해 s에서 m까지 가는 데 걸리는 최단거리와 m에서 e까지 가는 데 걸리는 최단거리를 이용, 가능한 모든 m을 계산


## Dynamic programming (동적계획법)

문제 유형이 다음과 같을 때 사용한다.

- 부분 반복 문제(Overlapping Subproblem)
- 최적 부분 구조(Optimal Substructure)

![e.g. Fibonacci numbers](https://media.vlpt.us/images/gillog/post/eb96e602-b7bf-47eb-9c49-2eda8465e158/1231313133.png)

> Memoization: 계산된 값을 memo (caching)

- **Top-down 방식**: 재귀함수 + memoization으로 caching된 부분 결과를 읽어와 해결
- **Bottom-up 방식**: 작은 문제부터 해결하여 큰 문제로 반복진행

```c
// Top-down
int memo[101];
memo[1] = 1;
memo[2] = 1;

int fib(int n)
{
  if (memo[n] != 0) 
    return memo[n];
  memo[n] = fib(n-1) + fib(n-2);
  return memo[n];
}
```

```c
// Bottom-up
int memo[101];
memo[1] = 1;
memo[2] = 1;

int fib(int n)
{
  for(int i = 3; i <=n; i++){
    memo[i] = memo[i-1] + memo[i-2];
  }
  return memo[n];
}
```
