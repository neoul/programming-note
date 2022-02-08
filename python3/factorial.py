#!/usr/bin/env python3

from functools import lru_cache

# caching factorial results against to the input, n
@lru_cache(maxsize=None)
def factorial(n : int) -> int:
    if n > 0:
        x = 1
        for i in range(2, n+1):
            x = x * i
        return x

import math

factorial(10) # 3628800
math.factorial(10) # 3628800
