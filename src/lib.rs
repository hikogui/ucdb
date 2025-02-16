
pub mod east_asian_width;

pub use east_asian_width::EastAsianWidth;
pub use east_asian_width::get_east_asian_width;

#[cfg(test)]
mod hikoru_ucdb {
    use super::*;

    #[test]
    fn east_asian_width()
    {
        assert_eq!(get_east_asian_width('a'), EastAsianWidth::Na);
        assert_eq!(get_east_asian_width('あ'), EastAsianWidth::W);
    }

    #[test]
    fn east_asian_width_full_coverage()
    {
        for c in '\u{0000}'..='\u{d7ff}' {
            let _ = get_east_asian_width(c);
        }
        for c in '\u{e000}'..='\u{10ffff}' {
            let _ = get_east_asian_width(c);
        }
    }
}
