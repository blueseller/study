mod s_option;
mod s_vet;
use s_option::opt_fun;
fn main() {

    // 学习 option
    //option_study();

    // 学习数组
    vet_study();


}

fn vet_study(){
    s_vet::one::first();
    s_vet::one::do_slice();
    s_vet::one::loop_vec();
    s_vet::diff_type::use_diff_types();
    s_vet::diff_type::show_ip_addrs();

}

fn option_study() {
    let typ = s_option::s_option::Director::North;
    s_option::s_option::match_director(typ);


    let typ = s_option::s_option::Director::West;
    s_option::s_option::Director::run(typ);

    let people_do=  s_option::opt_fun::Action::new("lk".to_string());

        let do_run = opt_fun::FunAction::DoRun("action is run".to_string());
        let do_walk = opt_fun::FunAction::DoWalk("action is walk".to_string());
        let do_jump= opt_fun::FunAction::DoJump("action is jump".to_string());

        
    people_do.do_something(do_run);

    people_do.do_something(do_walk);

    people_do.do_something(do_jump);

    let age = Some(30);

    println!("my age is {:?}", age);

    s_option::funs::compare();
}
