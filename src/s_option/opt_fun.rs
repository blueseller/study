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

    pub fn do_something(&self, do_action: FunAction<String>) {

        match do_action{
            FunAction::DoRun(t) => println!{"{} is  {} running",self.name, t.to_string()},
            FunAction::DoWalk(t) => println!{"{} is walking", t.to_string()},
            FunAction::DoJump(t) => println!{"{} is jumping", t.to_string()},
            //_ => println!{" do nothing"},
        }
    }
}