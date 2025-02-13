

pub struct CodePointDescription {
    // UnicodeData.txt
    bidi_class : String,
    canonical_combining_class : String,
    decomposition_type : String,
    decomposition_mapping : String,
    general_category : String,
    bidi_mirrored : bool,
    upper_cast_mapping : String,
    lower_cast_mapping : String,
    title_cast_mapping : String,

    // BidiBrackets.txt
    bidi_paired_bracket_type : String,
    bidi_paired_bracket : char,

    // BidiMirroring.txt
    bidi_mirroring_glyph : char,

    // CompositionExclusions.txt
    composition_exclusion : bool,

    // EastAsianWidth.txt
    east_asian_width : char,

    // GraphemeBreakProperty.txt
    grapheme_cluster_break : String,

    // SentenceBreakProperty.txt
    sentence_break : String,

    // WordBreakProperty.txt
    word_break : String,

    // LineBreak.txt
    line_break : String,

    // Scripts.txt
    script : String,

    // emoji-data.tt
    emoji : bool,
    emoji_presentation : bool,
    emoji_modifier : bool,
    emoji_modifier_base : bool,
    emoji_component : bool,
    extended_pictorgraphic : bool,

    // PropList.txt
    white_space : bool,
    bidi_control : bool,
    join_control : bool,
    dash : bool,
    hyphen : bool,
    quotation_mark : bool,
    terminal_punctuation : bool,
    other_math : bool,
    hex_digit : bool,
    ascii_hex_digit : bool,
    other_alphabetic : bool,
    ideographic : bool,
    diacritic : bool,
    extender : bool,
    other_lowercase : bool,
    other_uppercase : bool,
    non_character : bool,
    other_grapheme_extend : bool,
    ids_binary_operator : bool,
    ids_trinary_operator : bool,
    radical : bool,
    unified_ideograph : bool,
    other_default_ignorable : bool,
    deprecated : bool,
    soft_dotted : bool,
    logical_order_exception : bool,
    other_id_start : bool,
    other_id_continue : bool,
    sentence_terminal : bool,
    variation_selector : bool,
    pattern_white_space : bool,
    pattern_syntax : bool,
    prepended_concatenation_mark : bool,
    regional_indicator : bool,
}

