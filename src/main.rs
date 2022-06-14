mod s_option;
use s_option::opt_fun;
fn main() {
    println!("Hello, world!");
    let typ = s_option::s_option::Director::North;
    s_option::s_option::match_director(typ);


    let typ = s_option::s_option::Director::West;
    s_option::s_option::Director::run(typ);

    let people_do=  s_option::opt_fun::Action::new("lk".to_string());

        let do_run = opt_fun::FunAction::DoRun("action is run");
        let do_walk = opt_fun::FunAction::DoRun("action is walk");
        let do_jump= opt_fun::FunAction::DoRun("action is jump");
        
        people_do.do_something(do_run);

}
