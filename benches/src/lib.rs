use stm::{atomically, TVar};

pub fn write(tvar: &TVar<i32>, n: i32) {
    atomically(|transaction| transaction.write(tvar, n));
}
