use reduced_ordered_binary_decision_diagram::*;

fn main() {
    let mut formula = String::new();
    std::io::stdin()
        .read_line(&mut formula)
        .expect("Error reading from the standard input");
    match construct_robdd(&formula) {
        Ok((_, root)) => {
            println!("{}", root);
            println!("");
            println!("To visualize the diagram, paste the output to http://viz-js.com/");
        }
        Err(e) => {
            println!("Error {}", e);
            panic!()
        }
    }
}
