pub enum Director {
    East,
    West,
    North,
    South,
}

trait Path {
   fn run() ->String;
}

// 为什么不能这么做呢？
/* 
impl Path for Director::North {
    fn run() -> String{
        "run to east".to_string()
    }
}
*/

impl Director {
    pub fn run(self) -> String {
        match self {
            Director::East => {
                println!("run to east");
                "run to east".to_string()
            },
            Director::West=> {
                println!("run to west");
                "run to west".to_string()
            },
            Director::North=> {
                println!("run to north");
                "run to north".to_string()
            },
            Director::South=> {
                println!("run to south");
                "run to south".to_string()
            },
            _ => {
                "not run correct path".to_string()
            }
        }
    }
}

pub fn match_director(typ: Director) {
    match typ {
        Director::East => println!("match this is east"),
        Director::North | Director::South => {
            println!("match is north or south")
        },
        _ => println!("no math")
    }
}