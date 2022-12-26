pub fn read_file(year: String, day: String) -> String {
    let template = format!("input/{}/input_day{}", year, day);
    std::fs::read_to_string(template).expect("Couldn't read file")
}
