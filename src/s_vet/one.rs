use std::{ops::Index, ptr::read};



pub fn first() {
    let one = [1, 2, 3];

    let mut two = [1,2,3];

    if one == two {
        println!("one == mut two")
    }

    two[2] = 1;
    if one == two {
        println!("one == mut two")
    }

    println!("index 1 value is {}", two.index(1));
   
    // 不能够通过index 添加新的值
    //two[3] = 4;
    //two[4] = 4;
    //two[5] = 4;


    let xxx = vec![1,2,3];

    let yyy = vec!["a","b","c"];

    let mut xnew: Vec<i32> = Vec::with_capacity(10);
    xnew.push(1);
    xnew.push(2);
    xnew.push(3);
    xnew.push(4);
    xnew.push(5);
    xnew.resize(8, 6);

    while let Some(top) = xnew.pop(){
        println!("{}",top)
    }

}

pub fn do_slice(){
    println!("start do slice");
   fn read_slice(slice: &[usize])  {
    println!("{}", slice[1]);    
    // slice is read only
    // slice[0] = 2;
   }

   let v = vec![0,1];
   read_slice(&v);

   let u: &[usize] = &v;

   println!("use v point is {:?}", u);

   let u: &[_] = &v;

   println!("use v point is {:?}", u);



   let mut vv = vec![1,2,3,4,5];

   let first = &vv[0];

   // 如果push了， 后续还在使用first 则会报错，主要是指针可能会随着push 发生变更。
   //vv.push(6);

   println!("first is {}", first);

   println!("end do slice");

}

pub fn loop_vec() {
    println!("loop vec start");
    let v = vec![0; 6];

    for x in v {
        println!("loop value is {}", x)
    }
}