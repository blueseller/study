use std::collections::HashMap;

use crate::s_vet::hash;
pub fn show_hash(){
    let mut hash_data = HashMap::new();

    hash_data.insert("x",6);
    hash_data.insert("y",2);
    hash_data.insert("z",3);
    println!("{:?}", hash_data);
    hash_data.insert("z",5);
    println!("{:?}", hash_data);

    // 获取 get
    let x = hash_data.get("x");
    println!("x = {}", x.unwrap());
    let y = Some(&6);

    println!("{:?}",y);
   // assert_eq!(y, x);

     // 查询Yellow对应的值，若不存在则插入新值
    let v = hash_data.entry("Yellow").or_insert(5);
    assert_eq!(*v , 5);
    *v = 7;
    assert_eq!(*v , 7);
    let v = hash_data.entry("Yellow").or_insert(50);
    assert_eq!(*v , 7);



}