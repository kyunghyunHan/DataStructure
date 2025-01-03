# Algorithm
## 알고리즘의 분석
- 그 알고리즘을 실행하는데 필요한 자원을 예측하는 것을 의미
- 
## 점근적 표기
- 알고리즘에서는 최악의 수행시간이 모든 입력의 수행시간에 대한 상한이됨
- O:점근적으로 함수의상한을 나타내기 위해 사용,즉 함수가 최고차 항을 기준으로 특정 속도보다 빠르게 증가하지 않음을 나타냄
- Ω:점근적으로 하한을 나타냄,함수가 적어도 특정 비율만큼 빠르게 증가함을 나타냄
- 
## 표준 표기법과 흔히 사용되는 함수
#### 단조성
- m<=n일댸 f(m)<=f(n)이면 함수 f(n)은 단조증가
- m<=n일댸 f(m)>=f(n)이면 함수 f(n)은 단조감소
- m< n일댸 f(m)< f(n)이면 함수 f(n)은 순증가
- m< n일댸 f(m)> f(n)이면 함수 f(n)은 순감소

#### 내림과 올림
- ⌊n⌋ == n == ⌈n⌉
#### 모듈러연산
- 임의의 정수 a와 임의의 양의 정수n에 대한 am mod n 값은 a/n의 나머지다.
- a mod n  ==a-n⌊a/n⌋
#### 다향식
- 주어진 음이 아닌 정수 d에 대해 n의 d차 다항식은 다음과 같다
#### 지수함수
- n! : factorial
- 
#### 함수의 반복
## 자료구조
- 링크드리스트
- 배열
- 스택
- 큐
- 힙
- 해시테이블
- 이진검색트리
- 레드블랙트리
- B-트리

## Sort
- Key:정렬할 숫자
- 부속데이터:숫자가 다른 데이터와 연관된 키인 경우 이를 부속데이터(satellite data)
- 키와 부속데이터가 함꼐 하나의 record생성

#### 1.Bubble Sort
- O(n^2)
  
#### List 
- [삽입정렬](https://github.com/kyunghyunHan/algorithm_/blob/main/src/sort/insertion.rs)
- [병합정렬](https://github.com/kyunghyunHan/algorithm_/blob/main/src/sort/merge.rs)
- [선형시간정렬]()
## 분할정복
   #### 단계
   - divide:현재의 문제를 같은 문제를 다루는 다수의 부분 문제로 분할
   - conquer:부분 문제를 재귀적으로 풀어서 정복
   - combine:부분 문제의 해를 결합하여 원래 문제의 해가 되도록 만든다
   - 부분문제가 충분히 작아져 재귀호출없이 풀수 있을떄 bottoms out이라고 한다
## 동적프로그래밍
## 그리드 알고리즘
## 분할지불 분석
## 그래프 알고리즘
- [너비 우선 탐색]()
## 병렬알고리즘
## 온라인알고리즘
## 머신러닝
## NP-완비
## 근사알고리즘