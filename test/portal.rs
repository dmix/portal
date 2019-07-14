#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "cd ";
        let contents = "\
cd ~/dev/_rust/blog
rustup hi
rvm bno
cd ~
";
        let result = vec!["cd ~/dev/_rust/blog", "cd ~"];
        assert_eq!(result, portal::run(query, contents));
    }
}
