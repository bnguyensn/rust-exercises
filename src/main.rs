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
    mod ex_404_derive;
    mod ex_405_trait_bound;
    mod ex_406_str_slice;
    mod ex_407_deref;
    mod ex_409_from;
    mod ex_410_assoc;
    mod ex_411_clone;
    mod ex_412_copy;
    mod ex_413_drop;
    mod outro {
        mod ex_414_outro;
        mod ex_414_outro_test;
    }
}

mod mod_05 {
    mod ex_501_enum;
    mod ex_502_match;
    mod ex_503_variants;
    mod ex_504_if_let;
    mod ex_505_nullability;
    mod ex_506_fallibility;
    mod ex_507_unwrap;
    mod ex_508_error_enum;
}

fn main() {
    println!("Hello, world!");
}
