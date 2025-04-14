
#[test]
fn iterator_demonstration(){

    // java의 stream api와 매우 유사한 동작을 한다
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2, vec![2,3,4]);
}