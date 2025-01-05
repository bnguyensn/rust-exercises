mod common {
    pub mod lib;
}

mod mod_03 {
    pub mod ex_01_struct;
    pub mod ex_02_validation;
    pub mod ex_03_modules;
    pub mod ex_04_visibility;
    pub mod ex_05_encapsulation;
    pub mod ex_06_ownership;
    pub mod ex_07_setters;
    pub mod ex_08_stack;
    pub mod ex_09_heap;
    pub mod ex_10_references;

    mod outro {
        pub mod ex_12_outro;
        pub mod ex_12_outro_test;
    }
}

fn main() {
    println!("Hello, world!");
}
