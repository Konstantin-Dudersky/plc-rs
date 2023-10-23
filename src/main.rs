mod fb1_example;
mod fb2_example;
mod function_block;
mod library;
mod template;
mod types;

fn main() {
    let mut fb1 = fb1_example::FunctionBlockBase::default();

    let mut counter = 0;

    loop {
        let fb1_q = fb1.call(fb1_example::VarInput { counter: counter });
        println!("{}", fb1_q.out_counter);

        counter += 1;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
