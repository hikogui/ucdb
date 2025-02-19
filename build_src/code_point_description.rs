
#[derive(Clone,Default)]
pub struct CodePointDescription {
    // UnicodeData.txt
    pub bidi_class : String,
    pub canonical_combining_class : u8,
    pub decomposition_type : String,
    pub decomposition_mapping : String,
    pub general_category : String,
    //pub bidi_mirrored : bool,
    //pub upper_cast_mapping : String,
    //pub lower_cast_mapping : String,
    //pub title_cast_mapping : String,

    // BidiBrackets.txt
    //pub bidi_paired_bracket_type : String,
    //pub bidi_paired_bracket : char,

    // BidiMirroring.txt
    //pub bidi_mirroring_glyph : char,

    // CompositionExclusions.txt
    pub composition_exclusion : bool,

    // EastAsianWidth.txt
    pub east_asian_width : String,

    // GraphemeBreakProperty.txt
    pub grapheme_break : String,

    // SentenceBreakProperty.txt
    pub sentence_break : String,

    // WordBreakProperty.txt
    pub word_break : String,

    // LineBreak.txt
    pub line_break : String,

    // Scripts.txt
    pub script : String,

    // emoji-data.tt
    //pub emoji : bool,
    //pub emoji_presentation : bool,
    //pub emoji_modifier : bool,
    //pub emoji_modifier_base : bool,
    //pub emoji_component : bool,
    //pub extended_pictorgraphic : bool,

    // PropList.txt
    pub white_space : bool,
    pub bidi_control : bool,
    pub join_control : bool,
    pub dash : bool,
    pub hyphen : bool,
    pub quotation_mark : bool,
    pub terminal_punctuation : bool,
    pub other_math : bool,
    pub hex_digit : bool,
    pub ascii_hex_digit : bool,
    pub other_alphabetic : bool,
    pub ideographic : bool,
    pub diacritic : bool,
    pub extender : bool,
    pub other_lowercase : bool,
    pub other_uppercase : bool,
    pub noncharacter_code_point : bool,
    pub other_grapheme_extend : bool,
    pub ids_unary_operator : bool,
    pub ids_binary_operator : bool,
    pub ids_trinary_operator : bool,
    pub radical : bool,
    pub unified_ideograph : bool,
    pub other_default_ignorable_code_point : bool,
    pub deprecated : bool,
    pub soft_dotted : bool,
    pub logical_order_exception : bool,
    pub other_id_start : bool,
    pub other_id_continue : bool,
    pub id_compat_math_continue : bool,
    pub id_compat_math_start : bool,
    pub sentence_terminal : bool,
    pub variation_selector : bool,
    pub pattern_white_space : bool,
    pub pattern_syntax : bool,
    pub prepended_concatenation_mark : bool,
    pub regional_indicator : bool,
    pub modifier_combining_mark : bool,
}

impl CodePointDescription {
    pub fn default() -> Self {
        return Self{
            bidi_class : "ON".to_string(),
            canonical_combining_class : 0,
            decomposition_type : "canonical".to_string(),
            decomposition_mapping : String::new(),
            general_category : "Cn".to_string(),
            composition_exclusion : false,
            east_asian_width : String::new(),
            grapheme_break : String::new(),
            sentence_break : String::new(),
            word_break : String::new(),
            line_break : String::new(),
            script : String::new(),
            white_space : false,
            bidi_control : false,
            join_control : false,
            dash : false,
            hyphen : false,
            quotation_mark : false,
            terminal_punctuation : false,
            other_math : false,
            hex_digit : false,
            ascii_hex_digit : false,
            other_alphabetic : false,
            ideographic : false,
            diacritic : false,
            extender : false,
            other_lowercase : false,
            other_uppercase : false,
            noncharacter_code_point : false,
            other_grapheme_extend : false,
            ids_unary_operator : false,
            ids_binary_operator : false,
            ids_trinary_operator : false,
            radical : false,
            unified_ideograph : false,
            other_default_ignorable_code_point : false,
            deprecated : false,
            soft_dotted : false,
            logical_order_exception : false,
            other_id_start : false,
            other_id_continue : false,
            id_compat_math_continue : false,
            id_compat_math_start : false,
            sentence_terminal : false,
            variation_selector : false,
            pattern_white_space : false,
            pattern_syntax : false,
            prepended_concatenation_mark : false,
            regional_indicator : false,
            modifier_combining_mark : false,
        };
    }

    pub fn new() -> Self {
        return Self::default();
    }
}


