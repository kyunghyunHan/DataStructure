# Algorithm

##

- 쾨니히스베르스 다리문제:홀수점이 0개 혹은 2개이어야한다,홀수점이 2개일경우 홀수점에서 시작

## 자료구조

- 컴퓨터에서 데이터 사이의 논리적 관계를 표현하고 조직화 하는 방법
- 자료구조에 대한 고려없는 효율적인 알고리즘의 선택,또는 알고리즘에 대한 고려없는 효율적인 자료구조의 선택은 무의미
- 선형자료구조:배열,연결리스트,스택,큐
- 비선형 자료구조:트리,그래프

## 자료구조)Array

- 배열,연결리스트:정의 /논리적/물리적표현/구조/삽입,삭제,접근방법,종류ㅂ
- 데이터의 논리적 순서와 물리적 순서가 동일
- 원칙적으로 데이터에 대한 접근 시간은 동일
- 데이터의 삽입과 삭제시 추가적인 자료의 이동이 발생

## 자료구조)Linked List

- 단순연결리스트,이중연결리스트,원형연결리스트,이중원형 연결리스트
- 각 데이터 시퀀스가 순서를 가지고 연결된 순차적 구조
- 동적인 데이터 추가/삭제에 유리하다.
- 각 요소는 Node
- 각 Node에는 key와 다음 노드를 가리키는 포인터인 next가 포함
- 첫 번째 요소는 Head
- 마지막 요소는 Tail
- 배열보다 삽입 삭제가 상대적으로 효율적이다.

## 자료구조)Stack

- push,pop
- 순서가 보존되는 선형 데이터 구조
- 가장 마지막 요소부터 처리하는 LIFO
- 선형리스트의 한쪽 끝에서만 데이터와 삽입과 삭제가 발생하는 자료구조

## 자료구조)Que

- insert,delete
- Front/Head,Rear/Tail
- 가장 먼저 입력된 요소를 처리하는 FIFO
- 멀티 스레딩에서 스레드를 관리

## 자료구조)Tree

- T의 원소 가운데 단 하나의 루트노드가 존재한다.
- 로트 노드를 제외한 나머지 노드는 n개의서로 분리된 부분집합으로 나누어지며 각 Ti는 트리가된다
- 차수:degree노드의차수,트리의차수
- 리프노드(단말노드),비단말 노드
- 부모노드,자식노드,형제노드
- 조상,후손
- level,height,depth
- forest
- 그래프가 계층적 구조를 가진 형태
- 최상위 노드를 가지고 있다.
- 상위 노드를 parent node ,하위 노드를 child node라 한다.
- Binary Trees
- Binary Search Tree
- Heap

## 이진트리

- 각노드의 차수가 2이하인 순서트리
- 레벨 I에서 최대 노드의 개수 2i
- 높이 h인 이진트리의 최대 노드의개수 2h-1
- 포화이진트리,완전이진트리,전이진트리,균형이진트리,경사 이진트리
- 구현
- N개의 노드를 갖는 어떠한 이진트리의 높이도 완전이진트리의 높이보다는 크거나 같다
- 전 이진트리는 모든 노드의 차수가 0이거나 2인 이진트리이다.
- n개의 노드를 가진 완전이진트리의 높이는 정확히 log2n+1이다.

## 자료구조)Heap

- Binary Tree
- 최소힙:부모의 키 값이 자식의 키 값보다 작거나 같다
- - 루트 노드의 키 값이 트리의 최솟값
- 최대힙:부모의 키 값이 자식의 키 값보다 크거나 같다.
- - 루트 노드의 키 값이 트리의 최솟값

## 자료구조)Hash table

- 해시함수를 사용하여 변환한 값을 색인으로 삼아 키와 데이터를 저장하는 자료구조

## 자료구조)Graph

- G= (V,E)
- V:정점의 집합
- E;:간선의집합
- 무방향그래프,방향그래프
- 인접,부수
- 부분그래프
- 경로,경로의길이:경로에 존재하는 간선의 개수라고 한다.
- 차수:해당 정점에 부수된 간선의 개수이며, 방향 그래프에서는 정점으로 들어오는 간선의 수를 진입 차수라고 하고 해당 정점에서 나가는 간선의 수를 진출 차수라고 한다.
- 깊이 또느 높이:트리에 속하는 노드 중에서 가장 큰 레벨(레벨≥0)에 1을 더한 것을 의미한다.
- 단순경로,사이클,루프
- 연결
- 구현
- nodes/vertices사이에 edge가 있는 collection
- directed그래프는 일방통행
- undirected그래프는 양방향
- 소셜미디어 네트워크를 나타내는데사용

### 알고리즘

- 주어진 문제를 풀기위한 명령어들의 단계적 나열
- 입출력,명확성,유한성,유효성(이론적으로 문제해결이라는 관점에서 반드시 만족하지 않아도 되는 효율성)
- 실용적 관점에서는 효율적이어야함
- 모든 명령은 컴퓨터에 수행,효율적,단순명료,일정한 단계후 반드시 종료
- 순서:설계(상,하향식)-표현기술-정확성검증(수학적 증명)-효율성분석(공가복잡도,시간복잡도)
- 순서도

## 알고리즘 설계기법

- 분할정복
- 동적프로그래밍
- 욕심쟁이,탐욕

메모리양 -> 공간복잡도
수행시간 -> 시간복잡도(평균,최선,최악)

시간복잡도 알고리즘 단계의 단위 연산의 수행 횟수의 합

- 입력크기,입력 데이터의 상태가 증가하면 수행시간도 증가
- Bigo
- 빅오메가
- 빅세타
- 재귀함수

## 점근선능

- 수행시간의 다항식 함수에서 최고차항만을 계수없이 취해서 표현
- 입력크기가 무한대로 커짐에 따라 결정되는 성능
- 표기법(BigO,BigOmega,BigSeta)
- 최악의경우,최선의 수행시간
- 빅오함수의 크기관계
- O(1)< O(log n) < O(N) < O(n log n) < O(n^2) < O(n^3) < O (2^n)
- 효율적 -> 비효율적

## 알고리즘의 시간복잡도 구하기

- 알고리즘의 수행시간 f(n)을구 한후 f(n)= O(g(n))을 만족하는 최소 차수의 함수 g(n)을 찾음

## 기본 점화식과 폐쇄형

## 순환알고리즘

- 점화식으로 표현
- 재귀함수
- 알고리즘의 수행과정에서 자기 자신의 알고리즘을 다시 수행하는 형태
- 이진탐색,퀵정렬(최악,최선),합병정렬

## 1)분할정복

- 순한적으로 문제를 푸는 하향식 접근방법
- 원래 문제와 동일 ->입력크기만 작아짐
- 서로 독립적
- 처리단계:분할-정복-결합
- 주어진 문제의 입력을 나눌수 없을떄까지 두개이상의 작은 문제들로 순한분할하고
  분할된 작은 문제들을 각각 해결 후 이들의 해를 결합하여 문제의 해를 구하는 방법
- 분할,정복,결합
- 이진탐색
- 합병정렬
- 퀵정렬
- 선택문제

## 분할정복)이진탐색

- 정렬된 상태의 입력데이터를 절반씩 줄여가면서 원하는 값을 찾는 방법
- 정렬된 데이터에 대해서만 적용가능
- 데이터의 이동이발생
- 분할:배열의 가운데 원소를 기준으로 왼쪽과 오른쪽 부분 배열로 분할,탐색키와 가운데 원소 같으면 가운데 원소의 배열 인덱스를 반환/종료
- 정복:탐색키가 가운데 원소보다 작으면 왼쪽 부분 배열을 대상으로 이진탐색을 순할 호출,크면 오른쪽 부분배열을 대상으로 이진탐색을 순한 호출
- 결과:부분배열에 대하여 탐색 결과가 직접 반환
- 알고리즘
- T(n)= O(log n)
- 입력 배열의 데이터가 정렬된 경우에 대해서만 적용가능
- 삭제 /삽입이 빈번한 응용에서는 부적합
-

## 분할정복)합병정렬

- 전형적인 분할정복 방법이 적용된 알고리즘
- 입력크기가 n인 문제를 크기가 n/2인 두개의 작은 문제를 분할하고 분할된 두 작은 문제에 대해서 순한적으로 호출하여 주어진 문제를 해걀하는 것으
- 합병정렬에서 크기가 각각 n인 정렬된 두 부분 배열을 하나의 정렬배열로 합병할떄 걸리는 시간
- 합병함수 Merge- 정렬된 두 부분배열을 하나의 정렬배열로 만드는 함수
- O(n)
- tn= 2t(n/2)+0n
- 같은크기의 두 부분배열을 분할하고 순한 호출하여 정렬
- 정렬된 두 부분배열을 합쳐서 하나의 정렬배열을만듬
- 성능 t(n)= 2T(n/2)+0(n),T(1)= 0(1)->o(nlogn)
- 입력크기 n만큼의 추가적인 저장 장소필요
- 분할:배열을 동일한 크기의 두개의 부분 배열로 분할하고
- 정복:각각이, 부분배열을 순한적으로 정렬한후
- 정렬된 두 부분 배열을 합병하여 하나의 정렬된 배열을 만듬

## 분할정복)선택문제

- n개의 원소가 순서로 저장된 배열 a[0..n-1]에서 i번쨰로 작은 원소를 찾는 문제
- /5 나머지 버리고 그룹의 개수 (중간값들의 중간값 이용)
- 최소값찾기 적어도 (n-1)번 비교 o(n)
- 최솟값과 최대값 모두찾기 3n/2 -2번비교 O(n)
- 퀵정렬의 partition이용 최악 O(n2),평균 O(n)
- 중간값들의ㄴ 중간값 이용 최악 O(n) 평균 O(n)

- 방법1:최솟값 찾은후 최대값 찾기
- 방법2:모든 원소를 두개씩 짝을 이루어 동시에 최솟값/최갯값비교
- 최솟값 최대값 모두 찾기
- i번쨰로 작은 원소 찾기:퀵정렬의 분할함수 파티션 을 순한적으로 적용(최악O(n^2),평균(O(n))
- 최악의 경우:퀵정렬의 최악의 경우다
- 중간값들의 중간값이 피벗이다.
- 항상 일정한 비율의 두부분배열로 분할되도록 특정 성질을 만족하는 값을 피벗으로 선택한다₩

## 분할정복)퀵정렬

- 특정 원소를 기준으로 주어진 배열을 두 부분 배열로 분할하고 각 부분 배열에 대해서 퀵정렬을 순한적으로 적용하는 방식
- 피벗:주어진 배열을 두 부분배열로 분할할때 기준이 되는 특정 원소
- 피벗이 제자리를 잡도록 하여 정렬하는 방식
- 분할함수 파티션
- 분할:피벗을 기준으로 주어진 배열을 두 부분배열로 분할한다.
- 정복:두 부분배열에 대해서 퀵정렬을 순한적으로 적용하여 각 부분배열을 정렬한다
- 결합 :필요없음
- 알고리즘
- 최악의경우:피벗만 제자리를 잡고 나머지 원소가 하나의 부분배열이 되는 경우
- 피벗이 항상 부분배열의 최솟값또는 최대값이 되는경우
- T(n)= o(n^2)
- T(n)= o(nlogn)
- 평균:o(nlogn)
- 피벗 선택의 임의성만 보장되면 평균성능을 보일 가능성이 매우 높음

## 동적프로그램

- 최적성의 원리->점화식도출->소문제의 해를 테이블에 저장->저장된 해를 이용해서 점차적으로 상위 문제의 해를 구함
- 상향식 접근방법
- 최적화 문제해결에주로사용
- "해를 구축하는 테이블을 이용한다."
- 문제의 크기가 작은 소문제에 대한 해를 저장해 놓고 이를 이용 크기가 보다 큰 문제의 해를 점진적으로 만들어가는 상향식 접근방법
- 최적성의 원리->점화식도출->소문제의 해를 테이블에 저장->저장된 해를 이용해서 점차적으로 상위문제의 해를 구함
- 원래의 문제와 동일하지만 입력의 크기만 줄어들음
- 최적화문제(최솟값,최댓값)에 주로사용
- 소문제들은 서로 독립적이지 않고 중복되는 부분이존재
- 최적성의 원리를 반드시 만족해야함
- 피보나치 수열문제
- 연쇄 행렬곱셈문제
- 스트링 편집 거리문제
- 모든 정점간의 최단경로
- 저울문제
- 문제의 특성을 분석하여 최적성의 원리가 성립되는지 확인(주어진 문제에 대해서 최적해를 제고하는 점화식을 도출)
- !!최적성의 원리

```
1.주어진 문제에 대해서 최적해를 제공하는 점화식을 도출
2.가장 작은 소문제부터 점화식의 해를 구해서 테이블에 저장
3.테이블에 저장된 소문제의 해를 이용하여 점차적으로 큰 상위 문제의 해를 구함

```

## 동적프로그래밍)피보나치수열

- 알고리즘
- 분할적복:순한형태릐 알고리즘:소문제가 독립이 아니므로 중복된 계산이 필요=>매우 아라같음

## 동적프로그래밍)연쇄행렬곱셈

- n개의 행렬을 연쇄적으로 곱하는 경우
- n개의 행렬을 연쇄적으로 곱할떄 최적의 곱셈 순서를 구하는 문제
- n개의 행렬을 곱하는 최적의 순서는 n개의 행렬의 어떤 부분집합을 곱하는 최적의 순서를 포함
- 점화식 도출
- 알고리즘:MintMatMut
- 최적의 곱셈순서= i부터 j까지의 행렬을 곱할때 최적의 순서로 갈라지는 지점이 k이다
- m(i)(mxn) x Mj(nxr)->Mi-Mj(mxr)(기본 곱셈 횟수:m x n x r)
- O(n^3)

## 동적프로그래밍)스트링 편집거리문제

- 두 문자열 x와 y사이의 편집거리
- 문자열x를 y로 변환하는데 전체 편집 연산에 대한 최소 비용
- 점화식
- 최적성의 원리가 만족하는지 확인
- 연산
- 성능= O(nm)

## 동적프로그래밍)플로이드 알고리즘

- 모든 정점간의 최단경로
- 가중방향 그래프에서 모든 조합의 두정점간의 최단경로
- 경유할수 있는 정점의 범위를 1에서|v|까지 하나씩 늘려가면서 최단 경로를 구하는 방법
- 가중차의 합이 음수인 사이클이 존재하지 않음
- 점화식

## 동적프로그래밍)저울문제

- 무게 M인 물체를 n개의 추를 이용하여 양팔 저울로 달수 있는지 확인하는 문제
- 점화식
- 정수가 아니면 동적프로그래밍 방법 적용불가

```
C(4,6)은 M456을 수행하는 데 필요한 최소 곱셈 횟수를 의미한다.
3개의 행렬을 곱하는 경우에는 2가지 순서가 존재하며, 이때 필요한 기본 곱셈의 횟수는 다음과 같이 계산된다.
(d0=5, d1=2, d2=3, d3=4, d4=6, d5=7 d6=8)

▶ (M4M5)M6의 곱셈 횟수 = d345 + d356

▶ M4(M5M6)의 곱셈 횟수 = d456 + d346


따라서 C(4,6)의 값은 min(392, 528)인 392가 된다.

```

## 욕심쟁이 (Greedy Algorithm)

- 각 단계에서 가장 최선이라고 여겨지는 해를 선택해 나감으로써 전체적인 최적해를 구하는 방법
- 각 단계마다 국부적인 최적해를 선택해서 전체적인 최적해를 구함
- 항상 전체적인 최적해를 구한다는 것을 보장하지 못함
- 해를 구하는 일련의 선택단계마다 전후 단계의 선택과는 무관하게 해당 단계에서 가장 최선이라 여겨지는 국부적인 최적해를 선택하는

## 욕심쟁이)동전거스름돈 문

- 단순히 액면가가 가장 큰 동전부터 최대한 사용해서 거스름돈을 만듬
- o(n),동전의 액면가가 일반적인 경우에는 해결불가
- 동전의 액면가가 임의로 주어지는 경우에는 욕심쟁이 불가능

## 욕심쟁이)베낭문제

- M,n물체= (wi,pi)
- 단위 무게당 이익이 가장 큰 물체부터 최대한 베낭에 넣음
- 물체를 쪼개 넣을수 있다
- 베낭에 들어 있는 물체의 이익의 합이 최대가 되도록 물체를 넣는 방법을 찾는 문제
- O(n)

## 욕심쟁이)최소신장트리문제

- 그래프의 모든 정점을 포함한 연결된 트리 중에서 가중치의 합이 가장 작은 트리
- 크루스칼
- 프림

## 욕심쟁이)크루스칼

- 가장 무방향 그래프에 대한 트리중에서 간선의 가중치의 합이 가장 작은 트리
- 무방향 그래프 ,모든정점 연결,무사이클
- 서로 다른 연결성분에 속하는 정점을 있는 최소 가중치의 간선을 선택

## 욕심쟁이 프림

- 가장 무방향 그래프에 대한 트리중에서 간선의 가중치의 합이 가장 작은 트리
- 무방향그래프,모든정점연결,무사이클
- 선택된 정점.가중치가 최소인 선택

## 욕심쟁이)최단경로문제

- 플로이드
- 데이스크라

## 욕심쟁이)플로이드문제

- 모든 정점간의 최단경로
- 동적프로그래밍 방법적용

## 욕심쟁이)데이스크라문제

- 특정한 하나의 정점에서 다른 모든 정점으로의 최단경로
- 욕심쟁이
- 음의 가중치를 갖는 간선이 없다고 가정

## 욕심쟁이)작업스케줄링문제

- 가장 적은 개수의 기계를 사용하여 작업간의 충돌이 발행하지 않도록 모든 작업을 기계에 할당하는 문제
- 작업스케줄링 문제
- 최소기계를 선택해서 충돌없이 모든 작업을 기계에 할당하는 문제
- 시작이 빠른작업먼저
  작업선택문제,
  완료시간이 빠른거부터

## 욕심쟁이)작업선택문제

- 작업이 완료가 빠른거부터
- x[1]부터

## 욕심쟁이)허프만코딩

- 문자의 빈도또는 확률 정보를 이용한 통계적 압축방법
- 텍스트에서 각 문자가 출현하는 빈도수에 따라 다른 길이의 경로를 부여
- 출현 빈도수가 낮은 문자 -긴코드
- 인코딩과정
- 주어진 텍스트에서 각 문자의 출현 빈도수를 계산
- 각 문자의 빈도수를 이용하여 허프만 트리를 생성
- 문자의 출현 빈도수에 따라 다른 길이의 부호를 부여
- 인코딩->각문자의 출현 빈도수 계산
- 허프만 트리를 통해 각 문자의 이진코드 부여
- 성능
- 각 문자의 빈더수를 모르면 텍스트를 두번 읽어야 하므로 속도 저하

## 욕심쟁이)허프만 트리

- 욕심쟁이,전 이진트리
- 빈도수가 가장 작은 두트리를 합치는 과정을 반복
- 허프만 트리는 유일하지 안음
- 메세지는 길이는 동일
- 메세지는다름

## 정렬

- 주어진 데이터를 값의 크기 순서에 따라 재배치 하는것
- 내부정렬:모든 데이터를 주 기억장치에 저장한 후 정렬하는 방식
- 외부정렬:
  비교기반 알고리즘:버블정렬,선택정렬,셸정렬,합병정렬,퀵정렬,힙정렬->키값의 비교횟수 n2,n(longN),
  데이터 분포 기반 알고리즘:계수정렬,기수정렬

## 정렬)버블정렬

- 안정적인 정렬
- 서로 인접한 두 원소를검사하여 정렬하는 알고리즘
- 인접한 2개의 레코드를 비교하여 크기가 순서되로 되어 있지 않으면 서로 교환
- 제자리 정렬 알고리즘

## 정렬 )선택정렬

- 데이터가 가장 작은 값부터 차례대로 선택해서 나열하는 방식
- 언제나 동일한 시간 복잡도 n(o2)
- 제자리 정렬 알고리즘
- 안정적이지 않은 정렬 알고리즘

## 정렬)삽입정렬

- 주어진 데이터를 하나씩 뽑은 후 나열된 데이터들이 항상 정렬된 형태를 갖도록 뽑은 데이터를 바은 위치에 삽입해서 나열하는 방식
- 첫번쨰 데이터는 무조건 정렬됫다 가정을하고
- 입력이 거의 정렬되 ㄴ 경우 O(n)
- 안정적인 정렬
- 제자리

## 정렬)셸정렬

- 멀리떨어진 데이터와 비교 .교환해서 처리속도 향상
- 데이터가 삽입될 위치에서 멀리 떨어져 있어도 바로 왼쪽이 이웃한 데이터와의 비교를 통해 한번에 한자리씩만 이동 ->제자리 찾는데 느리다
- N2,nlongn

## 정렬)합병정렬

- 최선/최악/평균 수행시간이 모두 O(logn)
- 안정적인
- 평규적인 경우의 시간복잡도가 nlongn인 알고리즘 중에서 안정적인 정렬알고리즘

## 정렬)퀵정렬

- 피벗
- 제자리 정렬 알고리즘
- 안정적이지 않은 정렬 알고리즘
- 분할정복

## 정렬)힙정렬

- 힙자려구저의 장점을 활용한 정렬
- 최대힙
- 완전 이진트리
- 각 노드의 값은 자신의 자식 노드의 크거나 같다
- 힙의장점:임의의 값 삭제와 삽입이 용이
- 1

## 정렬)비교기반 정렬의하한

## 정렬)계수정렬

## 정렬)기수정렬

## 탐색알고리즘

## 탐색알고리즘)순차탐색

- 데이터큰거에는 적합하지안음
- 리스트 형태로 주어진 원소들에 대해서 처음부터 하나씩 차례대로 비교하면서 원하는 값을 가진원소를 찾는 방법
  순차 탐색은 키값의 순서에 관계없이 무순서로 연속해서 저장된 경우에 적합한 탐색 방법이다.
  입력 데이터에 대한 어떠한 처리도 없다는 것은 키값과는 무관하게 단지 데이터가 입력되는 순서로 저장되어 있다는 것을 의미한다.
  한편 이진 탐색은 데이터가 정렬된 형태를 유지하는 경우에만 적용 가능하며, 탐색 트리를 이용하는 경우에는 삽입을 통해 트리를 생성하는 별도의 초기화 과정을 거쳐야 한다.

## 탐색알고리즘)이진탐색

- 초기화 알고리즘
- 정렬된 리스트 형태로 주어진 원소들을 절반씩 줄여가면서 원하는 값을 가진 원소를 찾는 방법
  이진탐색 방법으로 가장 빨리 찾아낼수 있는 데이터 위치는
  이진 탐색 트리의 평균 탐색 시간은 O(logn)이지만, 리프 노드를 제외한 모든 노드의 차수가 1이 되어 경사 트리가 형성되는 최악의 경우에는 O(n)의 시간 복잡도를 갖는다.
- 삭제 알고리즘
- 연결리스트에서는 이진탐색 불간으
- 탐색 loan
- 초기화 long
- On
- 정렬된 리스트에 대해서만 적용가능
- 삽입과 삭제가 빈번한 경우네는 부적합

## 탐색알고리즘)이진탐색트리

- 이진트리
- 각 노드의 서브트리에 있는 모든 키값은 그 노드의 키값보다 작다
- 각 노드의 오른쪽 서브트리에 모든 키갑ㄱ은 그 노드의 키값보다 크다
- 연결리스트 사용
- 탐색 알고리즘
- 루트 노드로 부터 시작해서 크기 관계에 따라 트리의 경로를 다라 내려가면서 탐색을 수행
- 평균 logn 최악 on
- 삽입 삭제 연산시 기존 노드의 이동이 거의 발생하지 않음
- 원소의 삽입/삭제에 따라 경사 트리 형태가 될수 있음
- 최악의 수행시간

## 탐색알고리즘)흑적트리

- 모든 노드는 흑색이거나 적색이다
- 루트 노드와 리프노드는 흑색이다
- 적색 노드의 부모 노드는 흑색이다
- 임의의 노드로부터 리프 노드까지의 경로상에는 동일한 개수의 흑색 노드가 존재한다

## 탐색알고리즘)B-트리

## 탐색알고리즘)해싱

## 근사알고리즘

- 최적화 문제에 대해서 최적해에 가까운 해를 구하는 다항 시간 알고리즘

## 튜링기계

- 컴퓨터의 이론적 몯레
- 구성요소 ->테이프,기호,헤드,상태,규칙
- 결정론적 튜링기계
- 비결정로적 튜링기계
- 다항 시간 알고리즘
- 나닝도에 따른 문제분류
- - 쉬운문제
- - 어려운문제
    클레스 p와 클래스 NP

## 근사알고리즘)NP-완전문제

안전문제
NP-하드문제
NP\_완전문제와 NP-하드문제

## 근사알고리즘)근사알고리즘

## 허프만 트리

# 공부순서

### 0.문자열

### 1.liked list

- single linked list
- 이중연결

### 2.hash table

### 3.stack

### 4.queue

### 5.재귀

### 6.이진탐색

### 7.tree

### 8.그래프
