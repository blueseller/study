
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

trait Show {
   fn show_ip(&self);
}

struct V4(String);
impl Show for V4 {
   fn show_ip(&self) {
      println!("ip v4 addr is {}", self.0) 
   } 
}

struct V6(String);
impl Show for V6 {
   fn show_ip(&self) {
      println!("ip v6 addr is {}", self.0) 
   } 
}

pub fn show_ip_addrs() {

    let v: Vec<Box<dyn Show>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.show_ip();
    }
}


pub fn use_diff_types() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in &v {
        println!("{:?}",ip)
    }
}