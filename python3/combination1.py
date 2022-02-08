#!/usr/bin/env python3

# 서로 다른 가짓수를 가진 원소로 만들 수 있는 모든 조합 수
# https://programmers.co.kr/learn/courses/30/lessons/42578

def solution(clothes):
    # clothes 종류로 정렬
    pool = dict()
    for item in clothes:
        pool[item[1]] = pool.get(item[1], 0) + 1

    # 조합문제
    # [착용X, Gear#1, Gear#2, ...]의 가짓수를 가진 모든 조합수를 구하는 문제
    numOfCases = 1
    for _, gearcount in pool.items():
        numOfCases = numOfCases * (gearcount + 1)
    # 아무것도 찾용하지 않은 경우는 제외
    numOfCases = numOfCases -1
    return numOfCases


if __name__ == '__main__':
    print(solution([["yellowhat", "headgear"], ["bluesunglasses", "eyewear"], ["green_turban", "headgear"]]))
    print(solution([["crowmask", "face"], ["bluesunglasses", "face"], ["smoky_makeup", "face"]]))
    print(solution([["crowmask", "face"], ["bluesunglasses", "face"], ["smoky_makeup", "face"], ["B1", "B"], ["B2", "B"], ["C1", "C"]]))
