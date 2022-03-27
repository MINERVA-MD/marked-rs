use lazy_static::lazy_static;
use regex::{ Regex, RegexSet};

lazy_static! {
    static ref NEWLINE: Regex = Regex::new(r#"\n"#).unwrap();
    static ref CODE_NON_SPACE_CHARS: Regex = Regex::new(r#"[^ ]"#).unwrap();
    static ref CODE_CHAR_AT_START: Regex = Regex::new(r#"^ "#).unwrap();
    static ref CODE_CHAR_AT_END: Regex = Regex::new(r#" $"#).unwrap();
    static ref LINK_START: Regex = Regex::new(r#"^<"#).unwrap();
    static ref LINK_END: Regex = Regex::new(r#">$"#).unwrap();
    static ref LINE_DOWN: Regex = Regex::new("\n*$").unwrap();
    static ref CODE_TEXT: Regex = Regex::new("(?m)^ {1,4}").unwrap();
    static ref END_WITH_HASH_TAG: Regex = Regex::new("#$").unwrap();
    static ref END_WITH_SPACE: Regex = Regex::new(" $").unwrap();
    static ref BLOCK_QUOTE_TEXT: Regex = Regex::new("(?m)^ *> ?").unwrap();
    static ref INDENT: Regex = Regex::new(r#"[^ ]"#).unwrap();
    static ref ONE_LINE: Regex = Regex::new(r#"^ *$"#).unwrap();
    static ref LINE_SEARCH: Regex = Regex::new(r#"[^ ]"#).unwrap();
    static ref END_WITH_BLANK_LINE: Regex = Regex::new(r#"\n *\n *$"#).unwrap();
    static ref TASK_ITEM: Regex = Regex::new(r#"^\[[ xX]\] "#).unwrap();
    static ref LIST_ITEM_CONTENTS: Regex = Regex::new(r#"^\[[ xX]\] +"#).unwrap();
    static ref TAG: Regex = Regex::new(r#"\s+"#).unwrap();
    static ref ALIGN_REPLACED: Regex = Regex::new(r#"^ *|\| *$"#).unwrap();
    static ref HEADER_ALIGNMENT: Regex = Regex::new(r#" *\| *"#).unwrap();
    static ref TABLE_ROW: Regex = Regex::new(r#"\n[ \t]*$"#).unwrap();
    static ref CENTER_ALIGN: Regex = Regex::new(r#"^ *:-+: *$"#).unwrap();
    static ref RIGHT_ALIGN: Regex = Regex::new(r#"^ *-+: *$"#).unwrap();
    static ref LEFT_ALIGN: Regex = Regex::new(r#"^ *:-+ *$"#).unwrap();
    static ref ANCHOR_TAG_START: Regex = Regex::new(r#"(?i)^<a "#).unwrap();
    static ref ANCHOR_TAG_END: Regex = Regex::new(r#"(?i)^<\\/a>"#).unwrap();

    static ref SPACES: Regex = Regex::new(r#"\s+"#).unwrap();
    static ref LETTER_NUMBER: Regex = Regex::new(r#"[\p{L}\p{N}]"#).unwrap();
    static ref OUTPUT_LINK_TEXT: Regex = Regex::new(r#"\\([\[\]])"#).unwrap();
    static ref INDENT_TO_CODE: Regex = Regex::new(r#"^(\s+)(?:```)"#).unwrap();
    static ref INDENT_TO_NODE: Regex = Regex::new(r#"^\s+"#).unwrap();
    static ref NON_WORD_COLON: Regex = Regex::new(r#"[^\w:]"#).unwrap();
    static ref ORIGIN_INDEPENDENT: Regex = Regex::new(r#"(?i)^$|^[a-z][a-z0-9+.-]*:|^[?#]"#).unwrap();
    static ref ENCODED_PERCENT: Regex = Regex::new("%25").unwrap();
    static ref TABLE_CELL: Regex = Regex::new(r#" \|"#).unwrap();
    static ref TRAILING_WHITESPACE: Regex = Regex::new(r#"\\\|"#).unwrap();
    static ref ROW: Regex = Regex::new(r#"\|"#).unwrap();

    static ref PROTOCOL: Regex = Regex::new("^([^:]+:)[\\s\\S]*$").unwrap();
    static ref JUST_DOMAIN: Regex = Regex::new("^[^:]+:/*[^/]*$").unwrap();
    static ref DOMAIN: Regex = Regex::new("^([^:]+:/*[^/]*)[\\s\\S]*$").unwrap();

    static ref PEDANTIC_SPACING: Regex = Regex::new(r#"(?m)^ +$"#).unwrap();


    static ref LEXER_PRE_SPACES: Regex = Regex::new(r#"\r\n|\r"#).unwrap();
    static ref LEXER_PRE_TABS: Regex = Regex::new(r#"\t"#).unwrap();

    static ref OPEN_SINGLES: Regex = Regex::new(r#"(^|[-\u2014/(\[{"\s])'"#).unwrap();
    static ref OPEN_DOUBLES: Regex = Regex::new(r#"(^|[-\u2014/(\[{\u2018\s])""#).unwrap();
    static ref ELLIPSES: Regex = Regex::new(r#"\.{3}"#).unwrap();

    static ref CODE_SPACES: Regex = Regex::new("\\S*").unwrap();
    static ref END_WITH_NEWLINE: Regex = Regex::new("\n$").unwrap();

    static ref SERIALIZE_HTML: Regex = Regex::new("(?i)<[!\\\\/a-z].*?>").unwrap();
    static ref SERIALIZE_CHARS: Regex = Regex::new(r#"[\u2000-\u206F\u2E00-\u2E7F\\'!\\"\\#$%&()*+,./:;<=>?@\[\]^`\{|\}~]"#).unwrap();
    static ref SERIALIZE_SPACES: Regex = Regex::new(r#"\s"#).unwrap();


    // Regex Sets
    static ref CODE_CHARS_ON_BOTH_ENDS: RegexSet = RegexSet::new(&[ r#"^ "#, r#" $"# ]).unwrap();

    // Fancy
    static ref LINK_CAPTURES: fancy_regex::Regex = fancy_regex::Regex::new(r#"^([^'"]*[^\s])\s+(['"])(.*)\2"#).unwrap();
    static ref RE_ALIGN: fancy_regex::Regex = fancy_regex::Regex::new(r#"^ {1,4}(?=( {4})*[^ ])"#).unwrap();
    static ref RAW_BLOCK_START: fancy_regex::Regex = fancy_regex::Regex::new(r#"(?i)^<(pre|code|kbd|script)(\s|>)"#).unwrap();
    static ref RAW_BLOCK_END: fancy_regex::Regex = fancy_regex::Regex::new(r#"(?i)^<\/(pre|code|kbd|script)(\s|>)"#).unwrap();
}

pub enum RegexHelper {
    LineDown,
    Newline,
    CodeText,
    CodeNonSpaceChars,
    CodeCharAtStart,
    CodeCharAtEnd,
    LinkStart,
    LinkEnd,
    EndWithHashTag,
    EndWIthSpace,
    BlockQuoteText,
    Indent,
    OneLine,
    LineSearch,
    EndWithBlankLine,
    TaskItem,
    ListItemContents,
    Tag,
    AlignReplaced,
    HeaderAlignment,
    TableRow,
    CenterAlign,
    LeftAlign,
    RightAlign,
    AnchorTagStart,
    AnchorTagEnd,
    Spaces,
    LetterNumber,
    OutputLinkText,
    IndentToCode,
    IndentToNode,
    NonWordColon,
    OriginIndependent,
    EncodedPercent,
    TableCell,
    TrailingWhitespace,
    Row,
    Protocol,
    JustDomain,
    Domain,
    PedanticSpacing,
    LexerPreSpaces,
    LexerPreTabs,
    OpenSingles,
    OpenDoubles,
    Ellipses,
    CodeSpaces,
    EndWithNewline,
    SerializeHtml,
    SerializeChars,
    SerializeSpaces
}

pub enum RegexHelperFc {
    LinkCaptures,
    ReAlign,
    RawBlockStart,
    RawBlockEnd,
}

pub enum RegexHelperSet {
    CharsOnBothEnds
}

pub fn regx_helper<'a>(re: RegexHelper) -> &'a Regex {
    match re {
        RegexHelper::Newline                => { &NEWLINE }
        RegexHelper::CodeNonSpaceChars      => { &CODE_NON_SPACE_CHARS }
        RegexHelper::CodeCharAtStart        => { &CODE_CHAR_AT_START }
        RegexHelper::CodeCharAtEnd          => { &CODE_CHAR_AT_END }
        RegexHelper::LinkStart              => { &LINK_START }
        RegexHelper::LinkEnd                => { &LINK_END }
        RegexHelper::CodeText               => { &CODE_TEXT }
        RegexHelper::LineDown               => { &LINE_DOWN }
        RegexHelper::EndWithHashTag         => { &END_WITH_HASH_TAG }
        RegexHelper::EndWIthSpace           => { &END_WITH_SPACE }
        RegexHelper::BlockQuoteText         => { &BLOCK_QUOTE_TEXT }
        RegexHelper::Indent                 => { &INDENT }
        RegexHelper::OneLine                => { &ONE_LINE }
        RegexHelper::LineSearch             => { &LINE_SEARCH }
        RegexHelper::EndWithBlankLine       => { &END_WITH_BLANK_LINE }
        RegexHelper::TaskItem               => { &TASK_ITEM }
        RegexHelper::ListItemContents       => { &LIST_ITEM_CONTENTS }
        RegexHelper::Tag                    => { &TAG }
        RegexHelper::AlignReplaced          => { &ALIGN_REPLACED }
        RegexHelper::HeaderAlignment        => { &HEADER_ALIGNMENT }
        RegexHelper::TableRow               => { &TABLE_ROW }
        RegexHelper::CenterAlign            => { &CENTER_ALIGN }
        RegexHelper::LeftAlign              => { &LEFT_ALIGN }
        RegexHelper::RightAlign             => { &RIGHT_ALIGN }
        RegexHelper::AnchorTagStart         => { &ANCHOR_TAG_START }
        RegexHelper::AnchorTagEnd           => { &ANCHOR_TAG_END}
        RegexHelper::Spaces                 => { &SPACES }
        RegexHelper::LetterNumber           => { &LETTER_NUMBER }
        RegexHelper::OutputLinkText         => { &OUTPUT_LINK_TEXT }
        RegexHelper::IndentToCode           => { &INDENT_TO_CODE}
        RegexHelper::IndentToNode           => { &INDENT_TO_NODE}
        RegexHelper::NonWordColon           => { &NON_WORD_COLON}
        RegexHelper::OriginIndependent      => { &ORIGIN_INDEPENDENT }
        RegexHelper::EncodedPercent         => { &ENCODED_PERCENT }
        RegexHelper::TableCell              => { &TABLE_CELL }
        RegexHelper::TrailingWhitespace     => { &TRAILING_WHITESPACE }
        RegexHelper::Row                    => { &ROW }
        RegexHelper::Protocol               => { &PROTOCOL }
        RegexHelper::JustDomain             => { &JUST_DOMAIN }
        RegexHelper::Domain                 => { &DOMAIN }
        RegexHelper::PedanticSpacing        => { &PEDANTIC_SPACING }
        RegexHelper::LexerPreSpaces         => { &LEXER_PRE_SPACES }
        RegexHelper::LexerPreTabs           => { &LEXER_PRE_TABS }
        RegexHelper::OpenSingles            => { &OPEN_SINGLES }
        RegexHelper::OpenDoubles            => { &OPEN_DOUBLES}
        RegexHelper::Ellipses               => { &ELLIPSES }
        RegexHelper::CodeSpaces             => { &CODE_SPACES }
        RegexHelper::EndWithNewline         => { &END_WITH_NEWLINE }
        RegexHelper::SerializeHtml          => { &SERIALIZE_HTML }
        RegexHelper::SerializeChars         => { &SERIALIZE_CHARS }
        RegexHelper::SerializeSpaces        => { &SERIALIZE_SPACES }
    }
}

pub fn regx_helper_fc<'a>(re: RegexHelperFc) -> &'a fancy_regex::Regex {
    match re {
        RegexHelperFc::LinkCaptures     => { &LINK_CAPTURES }
        RegexHelperFc::ReAlign          => { &RE_ALIGN }
        RegexHelperFc::RawBlockStart    => { &RAW_BLOCK_START }
        RegexHelperFc::RawBlockEnd      => { &RAW_BLOCK_END}
    }
}

pub fn regx_helper_set<'a>(re: RegexHelperSet) -> &'a regex::RegexSet {
    match re {
        RegexHelperSet::CharsOnBothEnds => { &CODE_CHARS_ON_BOTH_ENDS }
    }
}