pub enum FunAction<T> {
    DoRun(T),
    DoWalk(T),
    DoJump(T),
}

pub struct Action {
    name :String,
}

impl Action {
    pub fn new(name: String) -> Action {
        Action{name:name}
    }

    pub fn do_something(self, do) {

        match do_action{
            FunAction::DoRun(T) => println!{"{} is running", T.to_string()},
            FunAction::DoWalk(T) => println!{"{} is walking", T.to_string()},
            FunAction::DoJump(T) => println!{"{} is jumping", T.to_string()},
            //_ => println!{" do nothing"},
        }
    }
}