
#[derive(Clone,Default)]
pub struct CodePointDescription {
    // UnicodeData.txt
    //pub bidi_class : String,
    //pub canonical_combining_class : String,
    //pub decomposition_type : String,
    //pub decomposition_mapping : String,
    //pub general_category : String,
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
    //pub composition_exclusion : bool,

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
    //pub white_space : bool,
    //pub bidi_control : bool,
    //pub join_control : bool,
    //pub dash : bool,
    //pub hyphen : bool,
    //pub quotation_mark : bool,
    //pub terminal_punctuation : bool,
    //pub other_math : bool,
    //pub hex_digit : bool,
    //pub ascii_hex_digit : bool,
    //pub other_alphabetic : bool,
    //pub ideographic : bool,
    //pub diacritic : bool,
    //pub extender : bool,
    //pub other_lowercase : bool,
    //pub other_uppercase : bool,
    //pub non_character : bool,
    //pub other_grapheme_extend : bool,
    //pub ids_binary_operator : bool,
    //pub ids_trinary_operator : bool,
    //pub radical : bool,
    //pub unified_ideograph : bool,
    //pub other_default_ignorable : bool,
    //pub deprecated : bool,
    //pub soft_dotted : bool,
    //pub logical_order_exception : bool,
    //pub other_id_start : bool,
    //pub other_id_continue : bool,
    //pub sentence_terminal : bool,
    //pub variation_selector : bool,
    //pub pattern_white_space : bool,
    //pub pattern_syntax : bool,
    //pub prepended_concatenation_mark : bool,
    //pub regional_indicator : bool,
}

impl CodePointDescription {
    pub fn new() -> Self {
        return Self::default();
    }
}


