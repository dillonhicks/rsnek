//
//extern crate fringe;
//
//use fringe::{OsStack, Generator};
//use fringe::generator::Yielder;
//
//
//type RuntimeYielder = Yielder<(), i32>;
//
//
//fn otherthing(yielder: &RuntimeYielder) {
//    let dataframe: Vec<i32> = vec![0, 1, 3, 123, 432];
//
//
//    for datum in &dataframe {
//        yielder.suspend(*datum);
//    }
//}

extern crate rattlesnake;

fn main() {
    println!("Hello RSNEK!");

    //    let stack = OsStack::new(1 << 16).unwrap();
    //    let mut gen = Generator::new(stack, move |yielder, ()| {
    //
    //        otherthing(yielder);
    //    });
    //
    //    loop {
    //        let out = gen.resume(());
    //        println!("{:?}", out); // Some(1)
    //
    //        match out {
    //            None => break,
    //            _ => (),
    //        };
    //    }

}
