# Python3 handbook

- [Python3 handbook](#python3-handbook)
  - [Reference](#reference)
  - [Variables](#variables)
  - [String](#string)
  - [String formatting](#string-formatting)
    - [f-string (Literal String Interpolation)](#f-string-literal-string-interpolation)
    - [%-string](#-string)
    - [str.format()](#strformat)
  - [Byte](#byte)
  - [Condition (if/elif/else)](#condition-ifelifelse)
  - [while / for](#while--for)
  - [function and lambda express](#function-and-lambda-express)
  - [built-in functions](#built-in-functions)
  - [Input](#input)
  - [Math](#math)
  - [list](#list)
  - [tuple](#tuple)
  - [set (집합)](#set-집합)
  - [dict (dictionary) = map](#dict-dictionary--map)
  - [Iterable Generators (yield)](#iterable-generators-yield)
  - [Decorator](#decorator)
  - [Comprehesion](#comprehesion)
  - [Useful libraries or simple implementation](#useful-libraries-or-simple-implementation)
    - [Prefix Tree (Trie)](#prefix-tree-trie)
    - [itertools](#itertools)
    - [functools](#functools)
      - [LRU (least recently used) caching for memoization (dynamic programming)](#lru-least-recently-used-caching-for-memoization-dynamic-programming)
      - [reduce](#reduce)
    - [heapq (Min heap)](#heapq-min-heap)
    - [collections](#collections)
      - [deque](#deque)
      - [Counter](#counter)
  - [Type Hint (Type Annotation)](#type-hint-type-annotation)
    - [Variable Type Annotation](#variable-type-annotation)
    - [Function Type Annotation](#function-type-annotation)
    - [Type Module](#type-module)
      - [Final (Constraint), Union (Multiple Types)](#final-constraint-union-multiple-types)
      - [Optional](#optional)
      - [Callable (Function Type Annotation)](#callable-function-type-annotation)
    - [Abstraction Type Annotation](#abstraction-type-annotation)
    - [User Type Annotation](#user-type-annotation)
  - [Type Check](#type-check)
  - [Environment Variables](#environment-variables)
  - [File](#file)
    - [Create, Write](#create-write)
    - [Exist](#exist)
    - [Copy](#copy)
    - [Get files](#get-files)
  - [Exception](#exception)

## Reference

- https://note.nkmk.me/en/python-list-clear-pop-remove-del/
- https://freedeveloper.tistory.com/271 # summary
- https://seongbindb.tistory.com/54 # summary
- https://wikidocs.net/book/1553

## Variables

```python
a = 5. # 5.0
a = 1e9 # 100000000.0
a = 0.3 + 0.6

import sys
sys.maxsize # max int
```

## String

```python
a = "hello"
b = 'world'
c = """hello\nworld"""
d = a+b # helloworld
c = ["goodbye", "world", "!"]
" ".join(c) # goodbye world !

a.startswith("Hello")
txt = "welcome to the jungle"
x = txt.split() # ['welcome', 'to', 'the', 'jungle']


```

## String formatting

### f-string (Literal String Interpolation)

다른 formatting 방법보다 직관적임

```python
name = 'Song'
sex = 'male'
f'Hi, I am {name}. I am {sex}.' # 'Hi, I am song. I am male.'
F'Hi, I am {name}. I am {sex}.' # 'Hi, I am song. I am male.'

x = 10
y = 3
f'x + y = {x+y} | x * y = {x*y}' # 'x + y = 13 | x * y = 30'

tuple = ('Hi, I am', 'song', 123)
f'tuple: {tuple}' # "tuple: ('Hi, I am', 'song', 123)"
```

### %-string

```python
'x + y = %d | x * y = %d' %(x+y, x*y) # 'x + y = 13 | x * y = 30'
# Positional argument
'Hey %s, there is a 0x%x error!' % (name, errno)
# 'Hey Bob, there is a 0xbadc0ffee error!'

# keyward argument
'Hey %(name)s, there is a 0x%(errno)x error!' % {"name": name, "errno": errno }
# 'Hey Bob, there is a 0xbadc0ffee error!'
```

### str.format()

```python
'x + y = {} | x * y = {}'.format(x+y, x*y) # 'x + y = 13 | x * y = 30'

'Hey {name}, there is a 0x{errno:x} error!'.format(name=name, errno=errno)
# 'Hey Bob, there is a 0xbadc0ffee error!'
```


> REF> 
> - https://docs.python.org/3/library/string.html#string-formatting
> - https://realpython.com/python-string-formatting

```python
"First, thou shalt count to {0}"  # References first positional argument
"Bring me a {}"                   # Implicitly references the first positional argument
"From {} to {}"                   # Same as "From {0} to {1}"
"My quest is {name}"              # References keyword argument 'name'
"Weight in tons {0.weight}"       # 'weight' attribute of first positional arg
"Units destroyed: {players[0]}"   # First element of keyword argument 'players'.
"Harold's a clever {0!s}"        # Calls str() on the argument first
"Bring out the holy {name!r}"    # Calls repr() on the argument first
"More {!a}"                      # Calls ascii() on the argument first
```

```python
format_spec     ::=  [[fill]align][sign][#][0][width][grouping_option][.precision][type]
fill            ::=  <any character>
align           ::=  "<" | ">" | "=" | "^"
sign            ::=  "+" | "-" | " "
width           ::=  digit+
grouping_option ::=  "_" | ","
precision       ::=  digit+
type            ::=  "b" | "c" | "d" | "e" | "E" | "f" | "F" | "g" | "G" | "n" | "o" | "s" | "x" | "X" | "%"
```

## Byte

```python
b'\xde\xad\xbe\xef'.hex() # 'deadbeef'
bytes.fromhex('deadbeef') # b'\xde\xad\xbe\xef'
```

## Condition (if/elif/else)

```python
# 비교 연산
x == y 
x != y 
x > y 
x < y 
x >= y 
x <= y

# 논리 연산
x and y
x or y
not x

x in [1, 2, 3]
x not in [1, 2, 3]

# if/elif/else
x = 100
if x >= 100:
    pass
elif x >= 200:
    print(">=200")
else:
    print("<100")

if x >= 100: pass
else: print("x >= 100")

r = ">=100" if x >= 100 else "<100"

# list 생성시 조건문
a = [1,2,3,4,5,5,5]
remove_set = {3,5}
result = [i for i in a if i not in remove_set]
```

## while / for

```python
i = 0
while i < 10:
    i++

# for i in ITERATABLE_OBJECT: e.g. list, tuple, set, dict
for i in range(5): # 0,1,2,3,4
    pass

for i in range(1, 10, 2): # 1,3,5,7,9
    pass

for i in range(1, 10, 2): # 1
    break # break the loop
```

## function and lambda express

```python
# function declaration
def add(a,b):
    return a + b

# call a function
print(add(3,7))
print(add(b=3, a=7))

print((lambda a, b: a+b)(3, 7))
```

## built-in functions

```python
a = [6,2,3,4,5]
sum(a) # 합
min(a) # 작은 수 선택
max(a) # 큰 수 선택
eval("(3+3) * 10") # 문자 수식 계산
sorted(a) # 오름차순 정렬
sorted(a, reverse=True) # 내림차순 정렬
a.sort() # 정렬
b = [('A', 1),('D', 0),('B', 2)]
sorted(jobs, key = lambda x: x[0]) # 첫번째값으로 정렬
sorted(b, key = lambda x: x[1], reverse=True) # 두번째값으로 정렬
b.sort(key = lambda x: x[1], reverse=True)

# sort 참고: https://docs.python.org/ko/3/howto/sorting.html#sortinghowto

```

```python
# ITERABLE 순회하며 FUNC호출
# map(FUNC, ITERABLE)
a = [1.2, 2.5, 3.7, 4.6]
for i in range(len(a)):
    a[i] = int(a[i])
a = [1.2, 2.5, 3.7, 4.6]
a = list(map(int, a))
a # [1, 2, 3, 4]


# ITERABLE 순회하며 FUNC호출하고, 결과를 병합
# reduce(FUNC, ITERABLE)
from functools import reduce 
data = [1, 2, 3, 4, 5]
def sum(a, b):
    return a + b
reduce(sum, data) # 15

```

## Input

```python
# Example1
>>> a = input().split()
10 20 (입력) # a=['10', '20']

>>> a = map(int, input().split())
10 20 (입력)
>>> a
<map object at 0x03DFB0D0>
>>> list(a)
[10, 20]

# Example2
import sys 
sys.stdin.readline().rstrip()
```


## Math

```python
a = 7 b = 3
a / b # 나누기
a % b # 나머지
a // b # 몫
a ** b # 거듭제곱; a의 b승
```

```python
import math

math.log10(5) # log10
math.log2(5) # log2
math.log(2) # log e 자연로그 밑
math.ceil(3.1) # 올림함수
math.floor(2.5) # 내림함수
math.factorial(a) # !a
math.sqrt(a) # 제곱근
math.pow(a) # a의 제곱
math.trunc(f) # 소수점을 자르는 함수
math.gcd(a, b) # a, b의 최대공약수
math.radians(t) # 각도 -> 라디안
math.degrees(t) # 라디안 -> 각도
math.fabs(x) # x의 절대값; 내장함수 abs와 달리 실수 가능
math.exp(x) # e의 x승
round(a,4) # 반올림
```

## list

```python
a = [1,2,3,4,5,6]
a[-1] # 6
a[-3] # 4
a[1:4] # [2,3,4]

# 초기화
array = [i for i in range(20) if i % 2 == 1] # [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
m = 3
n = 4
array = [[0] * m for _ in range(n) ] # [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]]
array[2][0] = 1 # [[0, 0, 0], [0, 0, 0], [1, 0, 0], [0, 0, 0]]s

a = [1,4,3]
a.append(2) 
a.sort() # 오름차순 정렬
a.sort(reverse = True) # 내림차순 정렬
a.reverse() # 리스트 원소 뒤집기
a.insert(2,3) # 특정 인덱스에 데이터 추가
a.count(3) # 데이터 개수 
a.remove(1) # 데이터 삭제 # 시간복잡도를 고려해서 remove는 사용하지 않는 것을 추천 (?)
a.pop(0) # 데이터 삭제 (index가 없으면, 마지막 데이터 삭제)
a.clear() # 데이터 모두 삭제
del a[1] # 데이터 삭제
del a[1:4] # 범위 삭제
del a[:] # 모두 삭제
del a[::step] # step 간격에 있는 데이터 삭제
```

## tuple

- list와 동일
- immutable value: 수정 x
- 소괄호 사용

```python
a = (10, 20)
a[0] = 20 # error
```

## set (집합)

```python
# 초기화
a = set([1,1,2,3,4,4,5]) 
b = {1,2,5,6}

a | b # {1, 2, 3, 4, 5, 6}
a & b # {1, 2, 5}
a - b # {3, 4}

a.add(7)
a.update([8, 9])
a.remove(7) # {1, 2, 3, 4, 5, 8, 9}
```

## dict (dictionary) = map

- mapping immutable key to mutable value
- not duplicatable key
- value access through keys

```python
>>> a = {1: 5, 2: 3}   # int 사용
>>> a = {(1,5): 5, (3,3): 3} # tuple사용
>>> a = { 3.6: 5, "abc": 3} # float 사용
>>> a = { True: 5, "abc": 3} # bool 사용
```

```python
d = dict()
d = dict( alice = 5, bob = 20, tony= 15, suzy = 30)

d = {}
d = {'abc' : 1, 'def' : 2}
d['abc'] = 5

# nested list (tuple) to a dict
name_and_ages = [['alice', 5], ['Bob', 13]]
name_and_ages = [('alice', 5), ('Bob', 13)]
name_and_ages = (('alice', 5), ('Bob', 13))
name_and_ages = (['alice', 5], ['Bob', 13])
dict(name_and_ages) # 위 동일한 결과

# deepcopy
import copy
a = {'alice': [1, 2, 3], 'bob': 20, 'tony': 15, 'suzy': 30}
b = copy.deepcopy(a)
b['alice'].append(5)
# b {'alice': [1, 2, 3, 5], 'bob': 20, 'tony': 15, 'suzy': 30}
# a {'alice': [1, 2, 3], 'bob': 20, 'tony': 15, 'suzy': 30}

# update
a = {'alice': [1, 2, 3], 'bob': 20, 'tony': 15, 'suzy': 30}
a.update({'bob':99, 'tony':99, 'kim': 30})
# a {'alice': [1, 2, 3], 'bob': 99, 'tony': 99, 'suzy': 30, 'kim': 30}

# loop
for key in a: # key loop
    print(key, a[key])

for val in a.values(): # value loop
    print(val)

for key, val in a.items(): # key, value loop
    print("key = {key}, value={value}".format(key=key,value=val))

# check a key available
'alice' in {'alice': [1, 2, 3], 'bob': 20, 'tony': 15, 'suzy': 30} # True
'teacher' in {'alice': [1, 2, 3], 'bob': 20, 'tony': 15, 'suzy': 30} # False

# delete
del a['alice']
```

## Iterable Generators (yield)

Generators are iterators, a kind of iterable you can only iterate over once. Generators do not store all the values in memory, they generate the values on the fly:

```python
def generater():
   mylist = range(3)
   for i in mylist:
       yield i * i

iterable = generater() # Generater 생성
print(iterable) # <generator object generater at 0xb7555c34>
for i in iterable:
    print(i)
```

## Decorator

## Comprehesion

> REF: https://wikidocs.net/16064

```python

words = "나는 파이썬을 공부하고 있습니다. 파이썬은 무척 심플하고 명료합니다.".split()

# List representation
[len(word) for word in words if len(word)> 3]

# Set representation
{len(word) for word in words if len(word)> 3}

# dict (hash)
# {key표현식 : value표현식 for item in iterable}
country_capital = {'대한민국':'서울', '영국' :'런던', '미국' :'워싱턴', '일본' :'도쿄'}
capital_country = {capital: country for country, capital in country_capital.items()}
```

## Useful libraries or simple implementation

### Prefix Tree (Trie)

```python
class Trie(object):
    def __init__(self):
        self.child = {}
    def insert(self, word):
        current = self.child
        for l in word:
            if l not in current:
            current[l] = {}
            current = current[l]
        current['#']=1
    def search(self, word):
        current = self.child
        for l in word:
            if l not in current:
            return False
            current = current[l]
        return '#' in current
    def startsWith(self, prefix):
        current = self.child
        for l in prefix:
            if l not in current:
            return False
            current = current[l]
        return True
ob1 = Trie()
ob1.insert("apple")
print(ob1.search("apple"))
print(ob1.search("app"))
print(ob1.startsWith("app"))
ob1.insert("app")
print(ob1.search("app"))
```

### itertools

> https://docs.python.org/ko/3/library/itertools.html

- permutations (순열)
- combinations (조합)
- product (중복을 허용하는 순열)
- combinations_with_replacement (중복을 허용하는 모든 조합)

```python
from itertools import permutations, combinations, product, combinations_with_replacement

# 데카르트 곱(cartesian product) - 중복허용 순열
product('ABCD', repeat=2) # AA AB AC AD BA BB BC BD CA CB CC CD DA DB DC DD

# 순열
permutations('ABCD', 2) # AB AC AD BA BC BD CA CB CD DA DB DC

# 조합 (중복X)
combinations('ABCD', 2) # AB AC AD BC BD CD

# 조합 (중복허용)
combinations_with_replacement('ABCD', 2) # AA AB AC AD BB BC BD CC CD DD
```

### functools

> https://docs.python.org/ko/3/library/functools.html

#### LRU (least recently used) caching for memoization (dynamic programming)

functools의 lru_cache decorator를 사용해 memoization 구현

```python
from functools import lru_cache
# 동적 프로그래밍(dynamic programming) 기법을 구현하기 위해 
# 캐시를 사용하여 피보나치 수를 효율적으로 계산하는 예:

@lru_cache(maxsize=None)
def fib(n):
    if n < 2:
        return n
    return fib(n-1) + fib(n-2)

>>> [fib(n) for n in range(16)]
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610]

>>> fib.cache_info()
CacheInfo(hits=28, misses=16, maxsize=None, currsize=16)
#버전 3.2에 추가.
```

#### reduce

위에서 언급함.

### heapq (Min heap)

- 최소 힙
- PriorityQueue
- O(NlogN)

```python
import heapq
heaplist = []
heapq.heappush(heaplist, 4)
heapq.heappush(heaplist, 1)
heapq.heappush(heaplist, 7)
heapq.heappush(heaplist, 3) # [1, 3, 7, 4]
a = heapq.heappop(heaplist) # [3, 4, 7]
print(a) # 1
print(heaplist) # [3, 4, 7]

# list -> priority queue
a = [10, 4, 111, 40]
heapq.heapify(a) # min heap order로 정렬
# [4, 10, 111, 40]

# max heap
nums = [4, 1, 7, 3, 8, 5]
heap = []

for num in nums:
  heapq.heappush(heap, (-num, num))  # (우선 순위, 값)

while heap:
  print(heapq.heappop(heap)[1])  # index 1
```


### collections

#### deque

```python
from collections import deque

q = deque()
q.append('a')
q.append('b')
q.append('c')
print("Initial queue")
print(q)
print("\nElements dequeued from the queue")
print(q.popleft())
print(q.popleft())
print(q.popleft())
print("\nQueue after removing elements")
print(q)
 
# Uncommenting q.popleft()
# will raise an IndexError
# as queue is now empty
```

#### Counter

A Counter is a dict subclass for counting hashable objects (e.g. dict).

```python

# A Python program to show different ways to create
# Counter
from collections import Counter
  
# With sequence of items 
print(Counter(['B','B','A','B','C','A','B','B','A','C']))
# with dictionary
print(Counter({'A':3, 'B':5, 'C':2}))
# with keyword arguments
print(Counter(A=3, B=5, C=2))

# output
# Counter({'B': 5, 'A': 3, 'C': 2})
# Counter({'B': 5, 'A': 3, 'C': 2})
# Counter({'B': 5, 'A': 3, 'C': 2})

coun = Counter()
coun.update([1, 2, 3, 1, 2, 1, 1, 2]) # Counter({1: 4, 2: 3, 3: 1})
coun.update([1, 2, 4]) # Counter({1: 5, 2: 4, 3: 1, 4: 1})

# Python program to demonstrate that counts in 
# Counter can be 0 and negative
c1 = Counter(A=4,  B=3, C=10)
c2 = Counter(A=10, B=3, C=4)
c1.subtract(c2) #  Counter({'c': 6, 'B': 0, 'A': -6})
c1 - c2 ##  Counter({'c': 6, 'B': 0, 'A': -6})
```

## Type Hint (Type Annotation)

> REF: https://www.daleseo.com/python-type-annotations/

```python
def greeting(name: str) -> str:
    return 'Hello ' + name
```

### Variable Type Annotation

```python
name: str = "John Doe"
age: int = 25
emails: list: ["john1@doe.com", "john2@doe.com"]
address: dict = {
  "state": "NM",
  "zip": "80556"
}
```

### Function Type Annotation

```python
def stringify(num: int) -> str:
    return str(num)

def plus(num1: int, num2: float = 3.5) -> float:
    return num1 + num2

def greet(name: str) -> None:
    return "Hi! " + str

def repeat(message: str, times: int = 2) -> list:
    return [message] * times
```

### Type Module

```python
from typing import List, Set, Dict, Tuple

nums: List[int] = [1]
unique_nums: Set[int] = {6, 7}
vision: Dict[str, float] = {'left': 1.0, 'right': 0.9}
john: Tuple[int, str, List[float]] = (25, "John Doe", [1.0, 0.9])
```

#### Final (Constraint), Union (Multiple Types)

```python
from typing import Final, Union

TIME_OUT: Final[int] = 10

def toString(num: Union[int, float]) -> str:
    return str(num)

toString(1)
toString(1.5)
```

#### Optional 

Optional은 None이 허용되는 함수의 매개 변수에 대한 타입을 명시할 때 유용

```python
from typing import Optional

def repeat(message: str, times: Optional[int] = None) -> list:
    if times:
        return [message] * times
    else:
        return [message]
```

#### Callable (Function Type Annotation)

```python
from typing import Callable

def repeat(greet: Callable[[str], str], name: str, times: int = 2) -> None:
    for _ in range(times):
        print(greet(name))

# Lamda
greet: Callable[[str], str] = lambda name: f"Hi, {name}!"
```

### Abstraction Type Annotation

```python
from typing import Iterable, List

def toStrings(nums: Iterable[int]) -> List[str]:
    return [str(x) for x in nums]
```

> Other types: `Sequence`, `Mapping`, `MutableMapping`

### User Type Annotation

```python
class User:
    ...

def find_user(id: str) -> User:
    ...

def create_user(user: User) -> User:
    ...
```

## Type Check

```python
pip install mypy
mypy our_file.py our_directory
```

## Environment Variables

```python
>>> import os
>>> user = os.environ['USER']
>>> user
'willing'
```

## File

### Create, Write

```python
try:
    #  w: write, +: newly
    with open('docs/readme.txt', 'w+') as f:
        f.write('Create a new text file!')
except FileNotFoundError:
    print("The 'docs' directory does not exist")
```

```python
f = open('docs/readme.txt', 'w+')
f.write("write data")
f.close()
```

### Exist

```python
import os.path
from os import path
path.exists("guru99.txt")
```

### Copy

```python
shutil.copyfile(original, target) # copy a file to a target file.
shutil.copy(src, dest) # copy a file to a directory.
```

### Get files

```python
import os
import glob
from pprint import pprint as pp
file_info = {os.path.realpath(p) : os.stat(p).st_size for p in glob.glob('*.*')}
pp(file_info)
{'/Users/Blidkaga/Documents/CodeLab/Python_Basic/a.log': 68,
 '/Users/Blidkaga/Documents/CodeLab/Python_Basic/a.txt': 0,
 '/Users/Blidkaga/Documents/CodeLab/Python_Basic/b.log': 0,
 '/Users/Blidkaga/Documents/CodeLab/Python_Basic/b.txt': 0,
 '/Users/Blidkaga/Documents/CodeLab/Python_Basic/c.log': 68,
 '/Users/Blidkaga/Documents/CodeLab/Python_Basic/err.log': 13,
 '/Users/Blidkaga/Documents/CodeLab/Python_Basic/exceptional.py': 566,
 '/Users/Blidkaga/Documents/CodeLab/Python_Basic/words.py': 910}
```

## Exception

> https://docs.python.org/3/library/exceptions.html

