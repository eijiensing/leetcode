mod problems;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run two-sum");
        return;
    }

    let problem_name = args.get(1).unwrap().replace("-", "_");

    match problem_name.as_str() {
        // <PROBLEMS>
"valid_anagram" => problems::valid_anagram::run(),
"contains_duplicate" => problems::contains_duplicate::run(),
"two_sum" => problems::two_sum::run(),
        _ => {
            create_new_problem(&problem_name);
            println!("Created new problem {problem_name}");
        }
    }
}

fn create_new_problem(name: &String) {
    let main_contents = std::fs::read_to_string("src/main.rs").unwrap();
    let mut new_main_contents: Vec<String> =
        main_contents.clone().lines().map(String::from).collect();
    let mut inserted = false;
    for (index, line) in main_contents.lines().enumerate() {
        if line.contains("<PROBLEMS>") && !inserted {
            new_main_contents.insert(index + 1, format!("\"{name}\" => problems::{name}::run(),"));
            inserted = true;
        }
    }
    std::fs::write("src/main.rs", new_main_contents.join("\n")).unwrap();

    let mod_contents = std::fs::read_to_string("src/problems/mod.rs").unwrap();
    let mut new_mod_contents: Vec<String> =
        mod_contents.clone().lines().map(String::from).collect();
    new_mod_contents.insert(0, format!("pub mod {name};"));
    std::fs::write("src/problems/mod.rs", new_mod_contents.join("\n")).unwrap();
    std::fs::write(
        format!("src/problems/{name}.rs"),
        "pub struct Solution;\n\npub fn run() {}",
    )
    .unwrap();
}