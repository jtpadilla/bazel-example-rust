extern crate basic_main_lib;
extern crate common_proto_rust;

pub fn main() {
    basic_main_lib::do_something(&common_proto_rust::Config::new());
}
