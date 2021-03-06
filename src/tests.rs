use super::*;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "Rust:\nsafe, fast, productive.'\nPick three.\nTrust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
