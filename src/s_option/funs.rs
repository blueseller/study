pub fn compare(){
    let num = Some(5);

    if num.unwrap() == 5 {
        println!("compare is true")
    }

    if num.unwrap() == 6 {
        println!("compare is false")
    }

    if None.unwrap_or("bike") == "bike" {
        println!("unwrap_or can set a default value")
    }
}