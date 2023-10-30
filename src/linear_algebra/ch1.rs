use ndarray;
use ndarray::{array,arr1,arr2 ,arr3,Array1, Array2,Array};
use ndarray_linalg::Norm;
pub fn main(){
    

    /*
    선형대수학에서 벡터는 수를 순서대로 나열한 것이다.
    벡터는 몇가지 중요한 특징을 가집니다.
    벡터의 차원은 벡터가 가진 원소의 수이며,벡터의 방향은 열방향 행방향인지를 나타냅니다.
    차원은 R^N으로 나타내며 R은 실수,N은 차원을 나타냅니다.
    

    x는 4차원 열벡터,y는 2차원 열벡터 ,z는 4차원 행벡터입니다.
    또는 x ∈ R^4로 표현할수 잇습니다.
    
    벡터 v를 일반적으로 소문자 v로 나타내며 선형대수학에서 벡터에 아무런 표시가 없다면 열방향 이라고 가정합니다.
    행벡터는 w^로 씁니다.
    T는 전치연산을 나타내며 전치연산은 열벡터를 행벡터를 변환합니다.

    벡터에 대해 잘 이해하기 위해 파이썬의 Numpy나 Rust의 ndarray를 사용하겟습니다.
    asArray는 방향이 없는 배열입니다.
    행이나 열 벡터가 아니라 1차원 리스트입니다.

    방향은 대괄호로 지정하며 가장 바깥족 대괄호는 모든 숫자를
    하나의 객체로 묶습니다.
    행벡터(rowVec)는 하나의 행이 모든 숫자를 가지지만  열벡터는 하나의 숫자를 가진 행이 여러개가 있습니다.
     */
    let as_list = arr1(&[1, 2, 3]);
    let as_array = arr1(&[1, 2, 3]);
    let row_vec = arr2(&[[1, 3, 3]]);
    let col_vec = arr2(&[[1], [2], [3]]);

    println!("asList:  {:?}", as_list.shape());
    println!("asArray: {:?}", as_array.shape());
    println!("rowVec:  {:?}", row_vec.shape());
    println!("colVec:  {:?}", col_vec.shape());


    /*
    순서대로 나열된 수 목록은 벡터의 대수학적 해석입니다.
    기하학적으로 해석하면 벡터는 특정 길이와 방향(또는 각도)를 가진 직선입니다.
    벡터의 두 점은 꼬리와 머리라고 부릅니다.
    일반적으로 머리는 꼬리와 명확하게 구분하기 위해 화살표가 달려있습니다.
    벡터를 기하학적 좌표가 인코딩된 형태로 볼수도 있지만 벡터와 좌표는 실제로 다릅니다.
    하지만 벡터가 원점에서 시작될때는 일치합니다.이를 기준위치라 부릅니다
    
    
    
     */
    

    /*
    
    벡터의 연산은 단순하고 직관적이며 에상한 방식대로 정확하게 동작할수 있습니다.
    두 벡터의 덧셈과 뺼셈은 서로 대응되는 원소끼리 더하고 뺼수 있습니다.
    벡터는 동일한 차원을 갖는 벡터끼리만 더하고 뺼수 있습니다.
    R^3의 벡터와 R5의 벡터를 더하고 빼는것은 불가능합니다.

     */

     let v = arr1(&[4, 5, 6]);
     let w = arr1(&[10, 20, 30]);
     let u = arr1(&[0, 3, 6, 9]);
 
     let v_plus_w = &v + &w;
     // Uncomment the following line to see the dimension mismatch error
     // let u_plus_w = &u + &w;
     
     println!("vPlusW: {:?}", v_plus_w);
     // Uncomment the following line to see the dimension mismatch error
     // println!("uPlusW: {:?}", u_plus_w);
     

     let v = arr2(&[[4, 5, 6]]);
     let binding = arr2(&[[10, 20, 30]]);
     let w = binding.t();

     // Perform the addition of v and w
     let result = &v + &w;
     println!("{:?}", result);
 
     /*    
     두 벡터를 기하학적으로 더할떄 한 벡터의 꼬리와 다른 벡터의 머리를 연결합니다
     더한 결과 벡터는 첫번쨰 벡터의 꼬리와 두벤쨰 벡터의 머리를 이은 선 입니다.
     이 방식으로 원하는 만큼 벡터를 더할수 있습니다.
     모든 벡터의 꼬리와 머리를 계속 이으면 최종합은 첫번쨰 꼬리에서 마지막 머리까지 이어지는 선이 됩니다.

     뺴는것은 덧셈과 조금다릅니다.
     두 벡터의 꼬리들을 같은 좌표에 둡니다.뺸결과의 벡터는 두번째 벡터의 머리에서 첫번쨰 벡터의 머리로 가는 선입니다.
     벡터 뺼셈은 기하학적으로 매우 중요한 개념입니다.
     직교벡터 분해의 기초이며 이는 곧 선형 최소제곱법의 기초가되고 선형 최고제곱법은  선형대수학의 가장 중요한 응용입니다.
     
      */


      /*
      스칼라-벡터 곱셈 
      선형대수학에서 스칼라는 벡터나 행렬에 포함된 숫자가 아닌 수 자체입니다.
      스칼라는 일반적으로 a 또는 그리스어 소문자로 나타냅니다.
      스칼라 벡터 곱은 간단한 편에 속합니다.
      각 벡터 원소에 스칼라를 곱합니다. 밑에와 같습니다.
      벡터를 저장하는 변수의 데이터 타입이 스칼라-벡터 곱셈에서는 데이터타입이 중요합니다.    
      */
      let s= 2;
      let a= [3,4,5];
      let b: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 1]>>= arr1(&[3,4,5]);
      println!("{:?}",b*s);

      let s= 2;
      let v= arr1(&[3,6]);
      println!("{:?}",s+v);
      
      
     /*위 코드는 선형대수학의 스칼라 -벡터 곱셈이아닙니다  하지만 실제 코딩에서는 자주 사용됩니다. */
  
     
    /*스칼라 벡터 덧셈
     벡터에 스칼라를 더하는 것은 선형대수학에서 불가능합니다.
     벡터와 스칼라는 별도의 수학적 객체이기떄문입니다.
     하지만 프로그래밍에서는 벡터에 스칼라를 더할수 잇으며  스칼라 벡터 곱셈 연산과 유사합니다.
    각 벡터 원소에 스칼라를 더합니다
    
     */
    
    let s= 2;
    let v: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 1]>>= arr1(&[3,6]);
    println!("{:?}",&v +s);

  
    /*스칼라 -벡터 곱셈의 기하학적 해석 
    스칼라는 벡터의 방향을 바꾸지않고 크기만 조정합니다.
    스칼라-벡터 곱셉의 결과는 스칼라가 1보다 큰지 0과 1사이인지 정확히 0인지 음수인지에 따라 다릅니다.

    스칼라는 벡터의 방향을 바꾸지 않습니다.하지만 음수일때 벡터 방향이 바뀌엇습니다.
    모순같지만 벡터는 원점을 통과해서 양방향의 무한히 긴 선을 가리킨다는 해석도있습니다.이것을 1차원 부분공간이라 정의합니다.
    그런 의미에서 회전된 벡터는 여전히 동일한 무한한 선을 가리키므로 음의 스칼라가 방향을 바꾼것이 아닙니다.
    이해석은 행렬공간,고유벡터 ,특이벡터에서 중요합니다.
    벡터 덧셈과 스칼라-벡터곱셈을 이용해 벡터의 평균을 구할수 있습니다.벡터의 평균을 구하는 것은 숫자의 평균과 동일합니다.
    벡터의 평군을 구하려면 먼저 두벡터를 더하고 스칼라 1/2를 곱하면 댑니다 일반적으로 N개벡터의 평균을 구하려면 모두 더하고 스칼라1/N을 곱헙니다; 
    
    
    */

    /*전치 
    전치 연산은 열벡터를 행벡터또는 반대로 변환합니다.
    행렬에는 행과 열이 있으므로 각 행렬 원소는 (행,열)인덱스를 가리킵니다.전치연산은 단순히 이러한 인덱스를 맞바꾸는 것입니다.

    벡터는 방향에 따라 하나의 행 또는 하나의 열입니다.예를 들어 6차원 행벡터에서 i인덱스는 1로 고정이고 j는 1에서 6까지 존재합니다.
    6차원 열벡터는 i인덱스가 1에서 6까지 존재하고 j는 1로 고정입니다.따라서 i와 j인덱스를 맞바꾸면 행과 열이 뒤바뀝니다.

    중효한 규칙은 벡터를 두번 전치하면 벡터는 원래 방향이 됩니다.다시말해 vᵀᵀ = v입니다 
    이는  데이터 과학과 머신러닝등 여러 중요한 증명에서 핵심근거가 됩니다.
    예를 들어 데이터 행렬에 자신의 전치를 곱하면 대칭공분산 행렬이 만들어집니다.

    */

    /*벡터 브로드캐스팅 
    브로드 캐스팅 연산은 현대 컴퓨터 기반 선형대수학에서만 존재합니다.
    브로드캐스팅은 본질적으로 한 벡터를 다른 벡터의 각 원소로 연산을 여러번반복하는 작업입니다.

    벡터[1 2 3]의 전치와 [10 20]의 패턴을 모두 모은 다음 덧셈을 브로드캐스팅하면 이식들을 하나의 식으로 간결하게 구현이 가능합니다
    
     */

     
     let binding = arr2(&[[1,2,3]]);
     let v = binding.t();
     let w = arr2(&[[10,20]]);
     println!("{}",&v+&w);



    /*벡터 크기와 단위벡터 
    벡터의 크기(기하하적 길이 또는 노름)는 벡터의 꼬리부터 머리까지의거리이며  표준 유클리드 거리 공식으로 구합니다
    벡터 크기는 벡터양 옆에 이중 수직 막대로 표시합니다 (|| v ||).


    일부 응용에서는 제곱 코드를 사용하는데 이떄는 오른쪽에 있는 제곱근 항을 제거합니다
    (||v||²)

    수학에서 벡터의 차원은 벡터의 원소 수이고 길이는 기하학적 거리입니다 
    프로그램에서는 len은 배열의 차원을 반환하고 np.norm()은 기하학적 길이를 반환합니다

    여러 응용 분야에서 기하학적인 길이가 1인 벡터를 사용하는데 이를 단위벡터라고합니다
    응용의 예로는 직교행렬과 회전행렬 .고유벡터 특이벡터가 있습니다.
    단위벡터는 ||v|| = 1로 정의합니다

    단위벡터가 아닌 벡터가 많지만 비단위 벡터로 이루어진 무한 집합이 단위벡터의 무한집합보다 아무리크다고 해도 무한 단위벡터의 수와 비단위벡터의 수는 모두 똑같이 무한합니다
    모든 비단위벡터는 하나의 단위벡터와 연관되어 있습니다.
    즉 비단위벡터와 같은 방향의 같은 방향의 단위벡터를 만들수 있습니다. 연관된 단위벡터를 만드는건 간단합니다.
    벡터 노름의 역수를 스칼라 곱셈하면 됩니다.

    부모벡터(v)와 같은 방향의 단위벡터를 표시하는 일반적인 규약

 
    */

    let v = arr1(&[1.0, 2.0, 3.0, 7.0, 8.0, 9.0]);
    let v_dim = v.len();  // Dimension in mathematics

    // Calculate the mathematical magnitude, length, or norm (L2 norm) manually
    // let v_mag = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    let v_mag= v.norm();
    println!("{}", v_dim);
    println!("{}", v_mag);








    /*벡터 내적
     내적은 선형대수학에서 가장 중요한 연산입니다.내적은 합성곱,상관관계,푸리에변환,행렬곱셈,선형특징추출,신호필터링등의 많은 연산과 알고리즘의 기본이 됩니다.

     aᵀa 를 사용하여 나타내며 내적은 하나의 숫자로 두 벡터 사이의관계를 나타냅니다.
     내적을 계산하려면 두벡터에서 대응되는 원소끼리 곱한 다음 모든 결과를더합니다.다시말해 원소별로 곱하고 합하는 것입니다.
     a와 b는 벡터이며 aℹ는 a의 i번쨰 원소입니다.
     
     공식을 보면 내적이 동일한 차원의 두 벡터 사이에만 성립함을 알수 있습니다.


     
          */


          let v= vec![1,2,3,4];

          let a: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 1]>> = arr1(&[1,2,3,4]);
          let b = arr1(&[5,6,7,8]);
          //내적을 구하는 함수
          //np.dot()함수는 실제로 벡터-내적을 구현하지는 않습니다.여러 내적으로 이루어진 행렬곱셈을 구현했습니다.

          let final_mat: i32 = a.dot(&b);


          println!("Dot product of v and w: {:?}", final_mat);

          //내적은 흥미로운 특성이 있습니다.
          //벡터에 스칼라를 곱하면 내적도 그만큼 커집니다.
          let s= 10;
          let final_mat: i32 = (s*a).dot(&b);
          println!("Dot product of v and w: {:?}", final_mat);
          //v와 w의 내적은 70이고 s*w(ơvᵀw)와 w의 내적은 700입니다.
          //음수 스칼라 s= -1은 내적크기는 그대로지만 기호는 반대가 됩니다.
          let a: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 1]>> = arr1(&[1,2,3,4]);

          let final_mat = (-1*a).dot(&b);
          println!("Dot product of v and w: {:?}", final_mat);
          //내적은 두 벡터 사이의 유사성 또는 매핑의 척도로 해석할수 있습니다.
          //20명의 키와 몸무게 데이터를 수집하고 그 데이터를 두개의 벡터에 저장했다고 가정해 봅니다.
          //당연히 이 두 변수들은 서로 관련이 있다고 예상할수 있습니다.
          //따라서 두벡터의 내적은 클것입니다.
          //또한 내적의 크기는 데이터의 단위에 따라 달라집니다.
          //예를 들어 그램과 센티미터로 측정된 데이터의 내적이 미터로 측정된 내적보다 더  큽니다.
          
          //그러나 이러한 단위의 차이는 정규화 계수로 제거할수 있습니다.
          //실제로 두변수 사이의 정규화된 내적을 피어슨 상관계수라고 하며 데이터 과학에서 가장 중요한 분석입니다.



         

    /*내적의 분배법칙 
    
    수학의 분배 법칙은 a(b+c)= ab+ac입니다   이를 벡터 내적에 적용하면 다음과 같습니다.
    즉 벡터것셈의 내적은 벡터-내적의 덧셈과 같습니다.
    두결과인 res1과 res2는 동일합니다.
    이는 내적의 분배 법칙이 성립함을 보여줍니다.
    */
   let a: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 1]>>= arr1(&[0,1,2]);
   let b: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 1]>>= arr1(&[3,5,8]);
   let c: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 1]>>= arr1(&[13,21,34]);
   //내적 분배 법칙
    let add= (&b+&c);
   let res1: i32 = (&a).dot(&add);
   let res2 = (&a).dot(&b)+(&a).dot(&c);

   println!("{} {}",res1,res2);







    /*내적의 기하학적 해석 
    내적은 두벡터의 크기(노름)를 곱하고 두벡터 사이의 각도에서 코사인만큼 크기를 늘리는 것입니다.
    수학적으로 동일하지만 다르게 표현된 형태입니다.
    
    벡터크기는 엄격하게 양수(0벡터는 ||0||=0ㅇ,러 제외)지만 각의 코사인 값은 -1과 +1사이입니다.
    이는 내적의 부호가 전적으로 두 벡터 사이의 기하학적 관계로 결정된다는 것을 뜻합니다.
    두벡터 사이의 각에 따른 내적부호 다섯가지 사례입니다.
    참고로 직교벡터의 내적은 0입니다.

    
    */

     /*아다마르곱 
     원소별로 곱하는 것을 뜻합니다
     아다마르곱을 구현하려면 두벡터의 대응되는 각 원소를 곱하면됩니다.곱의 결과는 두 벡터와 같은 차원의 벡터입니다.
      */

      let a = arr1(&[5.0, 4.9, 8.0, 2.0]);
      let b = arr1(&[1.0, 0.0, 0.5, 0.0]); // 크기를 a 배열과 동일하게 수정
  
      let result = &a * &b; // 아다마르 곱 수행

    println!("{:?}", result);
    /*외적
    외적은 열벡터와 행벡터를 이용해 행렬을 만듭니다.외적 행렬은 각 행은 행벡터 스칼라에 대응되는 열벡터원소를 곱한 것
    외적행렬의 각 열은 열벡터 스칼라에 대응되는 행백터 원소를 곱한 것
    외적은 스칼라 대신 행렬을 생성
    외적은 두벡터는 차원이 달라되 되지만 내적의 두벡터는 차원이 같아야 합니다.
    외적은 vwᵀ로 나타냅니다(벡터가 열방향)
     */

    /*직교벡터 분해
    벡터 또는 행렬을 분해하면 여러 간단한 조각으로 나뉘어 집니다.
    분해를 통해 행렬의 숨겨진 정보를 밝혀내거나 행렬을 사용하기 쉬운 형태로 만들기도 하고 데이터를 압축하기도 합니다.
    선형대수학의 거의 대부부닝 행렬분해와 연관되어 있다고 해도 과언이 아니므로 행렬분해는 대단히 중요합니다.
    - 숫자 42.01을 42와 0.01 두조각으로 나눌수 있습니다.아마도 0.01이 오차라서 제거하거나 아니면 데이터를 압축하려는 목적입니다.
    - (정수42는 부동소수점42.01보다 메모리 사용량이 적습니다.)
    - 어떤의도일지라도 수학적 대상을 더 단순한 대상의 합(42= 42+0.01)로 표현하는 것입니다.
    - 42를 소수 3,2,7의 곱으로 분해할수 잇습니다.이러한 분해를 소인수 분해 라고 하며 수치 처리 및 암호학에서 많이 사용됩니다.

    하나의 벡터를 두개의 벡터를 분해하는데,하나는 기준벡터와 직교하고 다른 하나는 기준벡터와평행합니다.직교벡터분해는 통계에서 그람-슈미드 과정과 QR분해에 직접적인 연관이 있습니다.
    분해의 목적을 시각화 해보면 표준위치에서 두개의 벡터 a와 b가 있습니다.우리는 a에서 b의 머리와 최대한 점을 찾아야 합니다/
    이것을 최적화 문제로도 나타낼수 있습니다.
    즉 투영거리가 최소가 되도록 벡터b를 벡터a에 투영합니다.그점은 a의 크기를 줄인가됩니다.
    그러면 이제 스칼라 벡터를 찾으면 댑니다

    중요한 것은 벡터뺼셈으로 b에서 까지의 선을 정의할수 있다는 점입니다.이선에 문자(벡터c)를 부여할수도 잇지만 먼저 답을 차기위해 뺄셈으로 표현하겟습니다.
    핵심은 b의 머리와 a가 직각으로 만나도록 그리면 b와 가장 가까운 a위의 점을 찾을수 있다는 점입니다.
    원점 b의 머리 a로 이루어진 삼각형을 직관적으로 상상해보면 b에서 a까지의 선의 길이는 각각 90보다 작거나 90보다 클수록 길어집니다.
    종합해보면 b -a가 a와 직교한다는 것을 추론해볼수 있습니다.
    즉 이벡터들은 수직이며 둘사이의 내적이 0이 되어야 합니다.

    이제b를 구하기 위해 약간의 대수학을 적용해보면 


    가단한 기하학적 그림과 공식 약간의 대수학을 적용하여 최소거리로 선에 투영하는 공식을 발견했씁니다.
    이를 직교투영법이라고 하며 선형모델을 푸는데 잘알려진 최소제곱식,통계학,머신러닝등의 많은 응용분야에서 기초가됩니다.


    최소거리 투영은 벡터 분해를 배우기 위해꼭 필요한 기초 내용입니다.
    목표벡터와 기준벡터라는 두개의 벡터를  설정하여 목적은 목표벡터를 두개의 다른벡터로 분해는것입니다.
    그 두개의 벡터의 합은 목표 벡터가 되고 하나의 벡터는 기준벡터와 직교하지만 다른벡터는 기준벡터와 평행합니다.

    수학계산을 시작하기 전에 용어를 잡으면
    목표벡터를 t,기준벡터를 r라고 부르며 목표벡터로부터 만들어진 두 벡터는 수직성분 t 과 평생성분 t로 각각 표시합니다.
    평행성분을 정의하면 r과 평행한 벡터는 r의 크기를 조정한 벡터는 분명히 r과 평행합니다.
    직교 투영공식을 적용하기만 하면 t를찾을수 잇습니다.

    이것이 평행성분이며 수직성분은 두벡터성분의 합이 목표벡터가 된다는 것을 이미 알고 있기 떄문에 간단하게 찾을수 잇습니다.
    목표벡터에서 평생성분을 빼고 남은것이 수직성분입니다.
   
    수직 성분이 기준벡터와 직교하는지 증명하려면 수직성분과 기준벡터 사이의 내적이 0인지 계산해보면 알수 있습니다.

     */
}