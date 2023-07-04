#[cfg(test)]
mod tests {
    #[test]
    fn read_example() {
        let input = dxf_table::input::read_input("example.toml").unwrap();
        dxf_table::output::write(input, "table.dxf".to_string()).unwrap();
    }
}
