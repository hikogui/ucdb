
pub mod east_asian_width;
pub mod line_break;
pub mod word_break;
pub mod sentence_break;
pub mod grapheme_break;
pub mod script;

pub use east_asian_width::EastAsianWidth;
pub use east_asian_width::get_east_asian_width;
pub use line_break::LineBreak;
pub use line_break::get_line_break;
pub use word_break::WordBreak;
pub use word_break::get_word_break;
pub use sentence_break::SentenceBreak;
pub use sentence_break::get_sentence_break;
pub use grapheme_break::GraphemeBreak;
pub use grapheme_break::get_grapheme_break;
pub use script::Script;
pub use script::get_script;

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

    #[test]
    fn line_break_full_coverage()
    {
        for c in '\u{0000}'..='\u{d7ff}' {
            let _ = get_line_break(c);
        }
        for c in '\u{e000}'..='\u{10ffff}' {
            let _ = get_line_break(c);
        }
    }

    #[test]
    fn word_break_full_coverage()
    {
        for c in '\u{0000}'..='\u{d7ff}' {
            let _ = get_word_break(c);
        }
        for c in '\u{e000}'..='\u{10ffff}' {
            let _ = get_word_break(c);
        }
    }

    #[test]
    fn sentence_break_full_coverage()
    {
        for c in '\u{0000}'..='\u{d7ff}' {
            let _ = get_sentence_break(c);
        }
        for c in '\u{e000}'..='\u{10ffff}' {
            let _ = get_sentence_break(c);
        }
    }

    #[test]
    fn grapheme_break_full_coverage()
    {
        for c in '\u{0000}'..='\u{d7ff}' {
            let _ = get_grapheme_break(c);
        }
        for c in '\u{e000}'..='\u{10ffff}' {
            let _ = get_grapheme_break(c);
        }
    }

    #[test]
    fn script_coverage()
    {
        for c in '\u{0000}'..='\u{d7ff}' {
            let _ = get_script(c);
        }
        for c in '\u{e000}'..='\u{10ffff}' {
            let _ = get_script(c);
        }
    }
}
