
pub mod east_asian_width;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn east_asian_width() {
        assert_eq!(east_asian_width::get_east_asian_width('a'), east_asian_width::EastAsianWidth::Na);
        assert_eq!(east_asian_width::get_east_asian_width('あ'), east_asian_width::EastAsianWidth::W);
    }
}
