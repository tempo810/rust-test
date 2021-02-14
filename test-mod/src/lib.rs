mod my_test {
    pub fn it_works() {
        println!("it works")
    }

    pub mod inner_test {
        pub fn it_works_too() {
            super::it_works();
            println!("it works too!")
        }
    }
}

use crate::my_test::inner_test;

pub fn try_test() {
    my_test::it_works();
    inner_test::it_works_too();
}
