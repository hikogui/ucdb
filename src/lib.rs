
pub mod east_asian_width;

pub use east_asian_width::EastAsianWidth;
pub use east_asian_width::get_east_asian_width;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn east_asian_width() {
        assert_eq!(get_east_asian_width('a'), EastAsianWidth::Na);
        assert_eq!(get_east_asian_width('あ'), EastAsianWidth::W);
    }
}
