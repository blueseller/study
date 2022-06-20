pub fn test_ref() {
    let x = 5;

    // 编译失败， 不可变值  不能被可变得借用
    //let y = &mut x; 

    // 这个是允许得
    let z = &x; 

    // x 是不可变 变量， 所以有 z 也不能改
    //z = 6;

    println!("{}", z);
}
