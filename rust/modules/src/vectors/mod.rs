pub fn vectors() {
    // declare
    let mut v1: Vec<i32> =  Vec::new();
    v1.push(5);
    v1.push(6);
    println!("{:?}", v1);

    // declare
    let v2: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", v2);

    // get
    println!("{:?}", v1.get(0));
    println!("{}", v1[1]);
    
    // set
    v1.push(7);

    // update each elements
    for i in &mut v1 {
        *i += 50;
    }

    // loop
    for i in &v1 {
        println!("{}", i);
    }

    // enumerations - 열거형으로 다른 종류의 type 삽입 가능
    
}
