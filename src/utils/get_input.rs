pub fn get_input(msg: Option<&str>) -> String {
    if let Some(msg) = msg {
        println!("{}: ", msg);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
