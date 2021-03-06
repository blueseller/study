mod s_option;
mod s_vet;
mod typ_change;
mod s_thread;
use s_option::opt_fun;
fn main() {

    // 学习 option
    //option_study();

    // 学习数组
    //vet_study();

    //vet_hash();

    //change_type();

    run_thread();

}

fn run_thread() {
    s_thread::ths::run_thread();

    s_thread::ths::run_thread_wait();

    s_thread::ths::run_thread_move();

    s_thread::s_mutex::count_num_arc();
}

fn change_type() {
    typ_change::change::change();
}

fn vet_hash(){
    s_vet::hash::show_hash();
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
