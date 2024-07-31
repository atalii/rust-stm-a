use std::thread;
use stm::{atomically, TVar};

fn increment(tvar: TVar<i32>, n: u32) {
    for _ in 0..n {
        atomically(|transaction| {
            let x = transaction.read(&tvar)?;
            transaction.write(&tvar, x + 1)
        });
    }
}

#[test]
fn concurrent_increments() {
    let tvar = TVar::new(0);

    let tvara = tvar.clone();
    let tvarb = tvar.clone();

    let a = thread::spawn(move || increment(tvara, 1024));
    let b = thread::spawn(move || increment(tvarb, 1024));

    a.join().expect("threading to work");
    b.join().expect("threading to work");

    let val = atomically(|transaction| transaction.read(&tvar));
    assert_eq!(val, 2048);
}
