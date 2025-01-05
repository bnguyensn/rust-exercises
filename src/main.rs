mod common {
    pub mod lib;
}

mod mod_03 {
    mod ex_01_struct;
    mod ex_02_validation;
    mod ex_03_modules;
    mod ex_04_visibility;
    mod ex_05_encapsulation;
    mod ex_06_ownership;
    mod ex_07_setters;
    mod ex_08_stack;
    mod ex_09_heap;
    mod ex_10_references;
    mod outro {
        mod ex_12_outro;
        mod ex_12_outro_test;
    }
}

mod mod_04 {
    mod ex_401_trait;
    mod ex_403_operator_overloading;
}

fn main() {
    println!("Hello, world!");
}
