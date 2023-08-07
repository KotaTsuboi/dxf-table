#[cfg(test)]
mod tests {
    #[test]
    fn read_example() {
        let input = stdr_table::input::read_input("example.toml").unwrap();
        stdr_table::output::write(input, "table.dxf".to_string()).unwrap();
    }
}
