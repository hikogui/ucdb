
pub mod east_asian_width;
pub mod line_break;
pub mod word_break;
pub mod sentence_break;
pub mod grapheme_break;
pub mod script;
pub mod composition_exclusion;
pub mod white_space;
pub mod bidi_control;
pub mod join_control;
pub mod dash;
pub mod hyphen;
pub mod quotation_mark;
pub mod terminal_punctuation;
pub mod other_math;
pub mod hex_digit;
pub mod ascii_hex_digit;
pub mod other_alphabetic;
pub mod ideographic;
pub mod diacritic;
pub mod extender;
pub mod other_lowercase;
pub mod other_uppercase;
pub mod noncharacter_code_point;
pub mod other_grapheme_extend;
pub mod ids_unary_operator;
pub mod ids_binary_operator;
pub mod ids_trinary_operator;
pub mod radical;
pub mod unified_ideograph;
pub mod other_default_ignorable_code_point;
pub mod deprecated;
pub mod soft_dotted;
pub mod logical_order_exception;
pub mod other_id_start;
pub mod other_id_continue;
pub mod id_compat_math_continue;
pub mod id_compat_math_start;
pub mod sentence_terminal;
pub mod variation_selector;
pub mod pattern_white_space;
pub mod pattern_syntax;
pub mod prepended_concatenation_mark;
pub mod regional_indicator;
pub mod modifier_combining_mark;
pub mod lower_case_mapping;
pub mod upper_case_mapping;
pub mod title_case_mapping;

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
pub use composition_exclusion::get_composition_exclusion;
pub use white_space::get_white_space;
pub use bidi_control::get_bidi_control;
pub use join_control::get_join_control;
pub use dash::get_dash;
pub use hyphen::get_hyphen;
pub use quotation_mark::get_quotation_mark;
pub use terminal_punctuation::get_terminal_punctuation;
pub use other_math::get_other_math;
pub use hex_digit::get_hex_digit;
pub use ascii_hex_digit::get_ascii_hex_digit;
pub use other_alphabetic::get_other_alphabetic;
pub use ideographic::get_ideographic;
pub use diacritic::get_diacritic;
pub use extender::get_extender;
pub use other_lowercase::get_other_lowercase;
pub use other_uppercase::get_other_uppercase;
pub use noncharacter_code_point::get_noncharacter_code_point;
pub use other_grapheme_extend::get_other_grapheme_extend;
pub use ids_unary_operator::get_ids_unary_operator;
pub use ids_binary_operator::get_ids_binary_operator;
pub use ids_trinary_operator::get_ids_trinary_operator;
pub use radical::get_radical;
pub use unified_ideograph::get_unified_ideograph;
pub use other_default_ignorable_code_point::get_other_default_ignorable_code_point;
pub use deprecated::get_deprecated;
pub use soft_dotted::get_soft_dotted;
pub use logical_order_exception::get_logical_order_exception;
pub use other_id_start::get_other_id_start;
pub use other_id_continue::get_other_id_continue;
pub use id_compat_math_continue::get_id_compat_math_continue;
pub use id_compat_math_start::get_id_compat_math_start;
pub use sentence_terminal::get_sentence_terminal;
pub use variation_selector::get_variation_selector;
pub use pattern_white_space::get_pattern_white_space;
pub use pattern_syntax::get_pattern_syntax;
pub use prepended_concatenation_mark::get_prepended_concatenation_mark;
pub use regional_indicator::get_regional_indicator;
pub use modifier_combining_mark::get_modifier_combining_mark;
pub use lower_case_mapping::get_lower_case_mapping;
pub use upper_case_mapping::get_upper_case_mapping;
pub use title_case_mapping::get_title_case_mapping;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn east_asian_width()
    {
        assert_eq!(get_east_asian_width('a'), EastAsianWidth::Na);
        assert_eq!(get_east_asian_width('あ'), EastAsianWidth::W);
    }

    #[test]
    fn lower_case_mapping()
    {
        assert_eq!(get_lower_case_mapping('B'), Some('b'));
    }
}
