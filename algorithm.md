# Algorithm note

## Dynamic programming (동적계획법)

문제 유형이 다음과 같을 때 사용한다.

- 부분 반복 문제(Overlapping Subproblem)
- 최적 부분 구조(Optimal Substructure)

![e.g. Fibonacci numbers](https://media.vlpt.us/images/gillog/post/eb96e602-b7bf-47eb-9c49-2eda8465e158/1231313133.png)

> Memoization: 계산된 값을 memo (caching to an array)

> Top-down 방식: 재귀함수 + memoization으로 caching된 부분 결과를 읽어와 해결
> Bottom-up 방식: 작은 문제부터 해결하여 큰 문제로 반복진행

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
