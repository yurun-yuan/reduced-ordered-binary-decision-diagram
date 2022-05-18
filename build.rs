extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .set_in_dir("formla_parser")
        .generate_in_source_tree()
        .process()
        .unwrap();
}
