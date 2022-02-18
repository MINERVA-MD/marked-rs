use std::cmp::min;
use derivative::Derivative;

use crate::parser::md4rs_html::{MD_BLOCKTYPE, MD_SPANTYPE, MD_TEXTTYPE, MDBlock, MDHTMLTag, MDParser, MDSpan};
use crate::parser::md4rs_html::MD_TEXTTYPE::MD_TEXT_NULLCHAR;


/* Flags specifying extensions/deviations from CommonMark specification.
 *
 * By default (when MD_PARSER::flags == 0), we follow CommonMark specification.
 * The following flags may allow some extensions or deviations from it.
 */
static MD_FLAG_COLLAPSEWHITESPACE: u32           = 0x0001;  /* In MD_TEXT_NORMAL, collapse non-trivial whitespace into single ' ' */
static MD_FLAG_PERMISSIVEATXHEADERS: u32         = 0x0002; /* Do not require space in ATX headers ( ###header ) */
static MD_FLAG_PERMISSIVEURLAUTOLINKS: u32       = 0x0004;  /* Recognize URLs as autolinks even without '<', '>' */
static MD_FLAG_PERMISSIVEEMAILAUTOLINKS: u32     = 0x0008;  /* Recognize e-mails as autolinks even without '<', '>' and 'mailto:' */
static MD_FLAG_NOINDENTEDCODEBLOCKS: u32         = 0x0010;  /* Disable indented code blocks. (Only fenced code works.) */
static MD_FLAG_NOHTMLBLOCKS: u32                 = 0x0020;  /* Disable raw HTML blocks. */
static MD_FLAG_NOHTMLSPANS: u32                  = 0x0040;  /* Disable raw HTML (inline). */
static MD_FLAG_TABLES: u32                       = 0x0100;  /* Enable tables extension. */
static MD_FLAG_STRIKETHROUGH: u32                = 0x0200;  /* Enable strikethrough extension. */
static MD_FLAG_PERMISSIVEWWWAUTOLINKS: u32       = 0x0400;  /* Enable WWW autolinks (even without any scheme prefix, if they begin with 'www.') */
static MD_FLAG_TASKLISTS: u32                    = 0x0800;  /* Enable task list extension. */
static MD_FLAG_LATEXMATHSPANS: u32               = 0x1000;  /* Enable $ and $$ containing LaTeX equations. */
static MD_FLAG_WIKILINKS: u32                    = 0x2000;  /* Enable wiki links extension. */
static MD_FLAG_UNDERLINE: u32                    = 0x4000;  /* Enable underline extension (and disables '_' for normal emphasis). */

static MD_FLAG_PERMISSIVEAUTOLINKS: u32          = (MD_FLAG_PERMISSIVEEMAILAUTOLINKS | MD_FLAG_PERMISSIVEURLAUTOLINKS | MD_FLAG_PERMISSIVEWWWAUTOLINKS);
static MD_FLAG_NOHTML: u32                       = (MD_FLAG_NOHTMLBLOCKS | MD_FLAG_NOHTMLSPANS);


/* Convenient sets of flags corresponding to well-known Markdown dialects.
 *
 * Note we may only support subset of features of the referred dialect.
 * The constant just enables those extensions which bring us as close as
 * possible given what features we implement.
 *
 * ABI compatibility note: Meaning of these can change in time as new
 * extensions, bringing the dialect closer to the original, are implemented.
 */
static MD_DIALECT_COMMONMARK: u32               = 0;
static MD_DIALECT_GITHUB: u32                   = (MD_FLAG_PERMISSIVEAUTOLINKS | MD_FLAG_TABLES | MD_FLAG_STRIKETHROUGH | MD_FLAG_TASKLISTS);


/************************
 ***  Internal Types  ***
 ************************/

pub enum MD_LINETYPE {
    MD_LINE_BLANK,
    MD_LINE_HR,
    MD_LINE_ATXHEADER,
    MD_LINE_SETEXTHEADER,
    MD_LINE_SETEXTUNDERLINE,
    MD_LINE_INDENTEDCODE,
    MD_LINE_FENCEDCODE,
    MD_LINE_HTML,
    MD_LINE_TEXT,
    MD_LINE_TABLE,
    MD_LINE_TABLEUNDERLINE
}

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct MDLineAnalysis {
    #[derivative(Default(value = "16"))]
    _type: u32,
    #[derivative(Default(value = "16"))]
    data: u32,
    beg: u32,
    end: u32,
    indent: u32  /* Indentation level. */
}

pub struct MDLine {
    beg: u32,
    end: u32
}

pub struct MDVerbatimLine {
    data: u32,
    beg: u32,
    indent: u32
}

pub struct MDMarkChain {
    head: i32, /* Index of first mark in the chain, or -1 if empty. */
    tail: i32, /* Index of last mark in the chain, or -1 if empty. */
}

pub struct MDCtx {
    pub text: &'static str,
    pub parser: MDParser,
    pub userdata: MDHTMLTag,
    pub  doc_ends_with_newline: bool,
    pub mark_char_map: [char; 256],
    pub mark_chains: [MDMarkChain; 13],
    pub n_table_cell_boundaries: u32,

    /* For resolving links. */
    pub unresolved_link_head: u32,
    pub unresolved_link_tail: u32,

    /* For resolving raw HTML. */
    pub html_comment_horizon: u32,
    pub html_proc_instr_horizon: u32,
    pub html_decl_horizon: u32,
    pub html_cdata_horizon: u32,

    /* Minimal indentation to call the block "indented code block". */
    pub code_indent_offset: u32,

    /* Contextual info for line analysis. */
    pub code_fence_length: u32,   /* For checking closing fence length. */
    pub html_block_type: u32,    /* For checking closing raw HTML condition. */
    pub last_line_has_list_loosening_effect: u32,
    pub last_list_item_starts_with_two_blank_lines: u32
}

impl MDCtx {
    fn ptr_chain(&self) -> &MDMarkChain {
        return &self.mark_chains[0];
    }
    fn tablecell_boundaries(&self) -> &MDMarkChain {
        return &self.mark_chains[1];
    }
    fn asterisk_openers_extraword_mod3_0(&self) -> &MDMarkChain {
        return &self.mark_chains[2];
    }
    fn asterisk_openers_extraword_mod3_1(&self) -> &MDMarkChain {
        return &self.mark_chains[3];
    }
    fn asterisk_openers_extraword_mod3_2(&self) -> &MDMarkChain {
        return &self.mark_chains[4];
    }
    fn asterisk_openers_intraword_mod3_0(&self) -> &MDMarkChain {
        return &self.mark_chains[5];
    }
    fn asterisk_openers_intraword_mod3_1(&self) -> &MDMarkChain {
        return &self.mark_chains[6];
    }
    fn asterisk_openers_intraword_mod3_2(&self) -> &MDMarkChain{
        return &self.mark_chains[7];
    }
    fn underscore_openers(&self) -> &MDMarkChain {
        return &self.mark_chains[8];
    }
    fn tilde_openers_1(&self) -> &MDMarkChain {
        return &self.mark_chains[9];
    }
    fn tilde_openers_2(&self) -> &MDMarkChain {
        return &self.mark_chains[10];
    }
    fn bracket_openers(&self) -> &MDMarkChain {
        return &self.mark_chains[11];
    }
    fn dollar_openers(&self) -> &MDMarkChain {
        return &self.mark_chains[12];
    }
    fn openers_chain_first(&self) -> u32 {
        return 1;
    }
    fn openers_chain_last(&self) -> u32 {
        return 12;
    }
}

pub struct MDUnicodeFoldInfo {
    codepoints: [u32; 3],
    n_codepoints: u32
}

pub struct MDFoldMap {
    map: Vec<u32>,
    data: Vec<u32>,
    n_codepoints: u32
}


/*****************
 ***  Helpers  ***
 *****************/

/* Case insensitive check of string equality. */
pub fn md_ascii_case_eq(s1: &str, s2: &str) -> bool {
    return s1.to_ascii_lowercase() == s2.to_ascii_lowercase();
}


pub fn md_ascii_eq(s1: &str, s2: &str) -> bool {
    return s1 == s2;
}

pub fn md_text_with_null_replacement(ctx: &mut MDCtx, _type: &MD_TEXTTYPE, text: &str) -> u32 {
    let mut off: u32 = 0;
    let mut ret: u32 = 0;
    let mut _str: &str = text.clone();
    let mut size: u32 = text.chars().count() as u32;

    loop {
        while off < size && off != size - 1 {
            off += 1;
        }
        if off > 0 {
            ret = (ctx.parser.text)(_type, text, &mut ctx.userdata);
            if ret != 0 {
                return ret;
            }

            _str = &_str[(off as usize)..];
            size -= 1;
            off = 0;
        }

        if off >= size {
            return 0;
        }

        ret = (ctx.parser.text)(&MD_TEXTTYPE::MD_TEXT_NULLCHAR, "", &mut ctx.userdata);

        if ret != 0 {
            return ret;
        }

        off += 1;
    }
}

// pub fn md_enter_block(_type: MD_BLOCKTYPE, detail: MDBlock, r: &MDHTMLTag) {
//
// }


/* If the offset falls into a gap between line, we return the following
 * line. */
pub fn md_lookup_line(off: u32, lines: Vec<&MDLine>, n_lines: i32) -> Option<&MDLine> {
    let (mut lo, mut hi) = (0 , n_lines - 1);
    let mut pivot: i32 = 0;
    let mut line: Option<&MDLine>;

    while lo <= hi {
        pivot = (lo + hi) / 2;
        line = Some(lines[pivot as usize]);

        if off < line.unwrap().beg {
            hi = pivot - 1;
            if hi < 0 || lines[hi as usize].end <= off {
                return line;
            } else if off > line.unwrap().end {
                lo = pivot + 1;
            } else {
                return line;
            }
        }
    }
    return None;
}


/*************************
 ***  Unicode Support  ***
 *************************/

pub fn md_unicode_bsearch__(codepoint: u32, map: &Vec<u32>) -> i32 {
    let (mut beg, mut end) = (0 , map.len() as i32 - 1);
    let (mut pivot_beg, mut pivot_end) = (0, 0);

    while beg <= end {
        let mid = (beg + end) / 2;
        pivot_beg = mid;
        pivot_end = mid;

        if map[pivot_end as usize] & 0x40000000 > 0 {
            pivot_end += 1;
        }

        if map[pivot_beg as usize] & 0x80000000 > 0 {
            pivot_beg -= 1;
        }

        if codepoint < (map[pivot_beg as usize] & 0x00ffffff) {
            end = pivot_beg - 1;
        } else if codepoint > (map[pivot_end as usize] & 0x00ffffff) {
            beg = pivot_end + 1;
        } else {
            return pivot_beg;
        }
    }
    return -1;
}

pub fn is_blank_(ch: char) -> bool {
    if ch == ' ' || ch == '\t' {
        return true;
    }
    return false;
}

pub fn is_whitespace(c: u32) -> i32 {

    let ch = char::from_u32(c).unwrap();
    if is_blank_(ch) || (ch == '\x0C' || ch == '\x0B') {
        return 1;
    }
    return 0;
}

pub fn is_in(ch: u32, ch_min: u32, ch_max: u32) -> bool {
    return (ch_min <= ch) && (ch <= ch_max);
}

pub fn is_punct(c: u32) -> i32 {
    if
        is_in(c, 33, 47)||
        is_in(c, 58, 64) ||
        is_in(c, 91, 96)
    {
        return 1;
    }
    return 0;
}

pub fn is_upper(ch: u32) -> bool {
    return (ch >= 'A' as u32) && (ch <= 'Z' as u32);
}

pub fn is_utf8_lead1(byte: u32)  -> bool {
    return byte <= 0x7f;
}

pub fn is_utf8_lead2(byte: u32)  -> bool {
    return (byte & 0xe0) == 0xc0;
}

pub fn is_utf8_lead3(byte: u32)  -> bool {
    return (byte & 0xf0) == 0xe0;
}

pub fn is_utf8_lead4(byte: u32)  -> bool {
    return (byte & 0xf8) == 0xf0;
}

pub fn is_utf8_tail(byte: u32)  -> bool {
    return (byte & 0xc0) == 0x80;
}

pub fn R(cp_min: u32, cp_max: u32) -> u32 {
    let cp_1 = (cp_min) | 0x40000000;
    let cp_2 = (cp_max) | 0x80000000;

    return min(cp_1, cp_2);
}

pub fn S(cp: u32) -> u32 {
    return cp;
}

pub fn ch(off: u32, ctx: &MDCtx) -> u32 {
    return ctx.text.chars().nth(off as usize).unwrap() as u32;
}

// pub fn str_(off: u32, ctx: MDCtx) -> u32 {
//
// }


pub fn md_is_unicode_whitespace__(codepoint: u32) -> i32 {
    let mut WHITESPACE_MAP: Vec<u32> = vec![
        S(0x0020), S(0x00a0), S(0x1680),
        R(0x2000,0x200a),
        S(0x202f), S(0x205f), S(0x3000)
    ];

    if codepoint <= 0x7f {
        return is_whitespace(codepoint);
    }

    return md_unicode_bsearch__(codepoint, &WHITESPACE_MAP);
}

pub fn md_is_unicode_punct__(codepoint: u32) -> i32 {
    let mut PUNCT_MAP: Vec<u32> = vec![
        R(0x0021,0x0023), R(0x0025,0x002a), R(0x002c,0x002f), R(0x003a,0x003b), R(0x003f,0x0040),
        R(0x005b,0x005d), S(0x005f), S(0x007b), S(0x007d), S(0x00a1), S(0x00a7), S(0x00ab), R(0x00b6,0x00b7),
        S(0x00bb), S(0x00bf), S(0x037e), S(0x0387), R(0x055a,0x055f), R(0x0589,0x058a), S(0x05be), S(0x05c0),
        S(0x05c3), S(0x05c6), R(0x05f3,0x05f4), R(0x0609,0x060a), R(0x060c,0x060d), S(0x061b), R(0x061e,0x061f),
        R(0x066a,0x066d), S(0x06d4), R(0x0700,0x070d), R(0x07f7,0x07f9), R(0x0830,0x083e), S(0x085e),
        R(0x0964,0x0965), S(0x0970), S(0x09fd), S(0x0a76), S(0x0af0), S(0x0c77), S(0x0c84), S(0x0df4), S(0x0e4f),
        R(0x0e5a,0x0e5b), R(0x0f04,0x0f12), S(0x0f14), R(0x0f3a,0x0f3d), S(0x0f85), R(0x0fd0,0x0fd4),
        R(0x0fd9,0x0fda), R(0x104a,0x104f), S(0x10fb), R(0x1360,0x1368), S(0x1400), S(0x166e), R(0x169b,0x169c),
        R(0x16eb,0x16ed), R(0x1735,0x1736), R(0x17d4,0x17d6), R(0x17d8,0x17da), R(0x1800,0x180a),
        R(0x1944,0x1945), R(0x1a1e,0x1a1f), R(0x1aa0,0x1aa6), R(0x1aa8,0x1aad), R(0x1b5a,0x1b60),
        R(0x1bfc,0x1bff), R(0x1c3b,0x1c3f), R(0x1c7e,0x1c7f), R(0x1cc0,0x1cc7), S(0x1cd3), R(0x2010,0x2027),
        R(0x2030,0x2043), R(0x2045,0x2051), R(0x2053,0x205e), R(0x207d,0x207e), R(0x208d,0x208e),
        R(0x2308,0x230b), R(0x2329,0x232a), R(0x2768,0x2775), R(0x27c5,0x27c6), R(0x27e6,0x27ef),
        R(0x2983,0x2998), R(0x29d8,0x29db), R(0x29fc,0x29fd), R(0x2cf9,0x2cfc), R(0x2cfe,0x2cff), S(0x2d70),
        R(0x2e00,0x2e2e), R(0x2e30,0x2e4f), S(0x2e52), R(0x3001,0x3003), R(0x3008,0x3011), R(0x3014,0x301f),
        S(0x3030), S(0x303d), S(0x30a0), S(0x30fb), R(0xa4fe,0xa4ff), R(0xa60d,0xa60f), S(0xa673), S(0xa67e),
        R(0xa6f2,0xa6f7), R(0xa874,0xa877), R(0xa8ce,0xa8cf), R(0xa8f8,0xa8fa), S(0xa8fc), R(0xa92e,0xa92f),
        S(0xa95f), R(0xa9c1,0xa9cd), R(0xa9de,0xa9df), R(0xaa5c,0xaa5f), R(0xaade,0xaadf), R(0xaaf0,0xaaf1),
        S(0xabeb), R(0xfd3e,0xfd3f), R(0xfe10,0xfe19), R(0xfe30,0xfe52), R(0xfe54,0xfe61), S(0xfe63), S(0xfe68),
        R(0xfe6a,0xfe6b), R(0xff01,0xff03), R(0xff05,0xff0a), R(0xff0c,0xff0f), R(0xff1a,0xff1b),
        R(0xff1f,0xff20), R(0xff3b,0xff3d), S(0xff3f), S(0xff5b), S(0xff5d), R(0xff5f,0xff65), R(0x10100,0x10102),
        S(0x1039f), S(0x103d0), S(0x1056f), S(0x10857), S(0x1091f), S(0x1093f), R(0x10a50,0x10a58), S(0x10a7f),
        R(0x10af0,0x10af6), R(0x10b39,0x10b3f), R(0x10b99,0x10b9c), S(0x10ead), R(0x10f55,0x10f59),
        R(0x11047,0x1104d), R(0x110bb,0x110bc), R(0x110be,0x110c1), R(0x11140,0x11143), R(0x11174,0x11175),
        R(0x111c5,0x111c8), S(0x111cd), S(0x111db), R(0x111dd,0x111df), R(0x11238,0x1123d), S(0x112a9),
        R(0x1144b,0x1144f), R(0x1145a,0x1145b), S(0x1145d), S(0x114c6), R(0x115c1,0x115d7), R(0x11641,0x11643),
        R(0x11660,0x1166c), R(0x1173c,0x1173e), S(0x1183b), R(0x11944,0x11946), S(0x119e2), R(0x11a3f,0x11a46),
        R(0x11a9a,0x11a9c), R(0x11a9e,0x11aa2), R(0x11c41,0x11c45), R(0x11c70,0x11c71), R(0x11ef7,0x11ef8),
        S(0x11fff), R(0x12470,0x12474), R(0x16a6e,0x16a6f), S(0x16af5), R(0x16b37,0x16b3b), S(0x16b44),
        R(0x16e97,0x16e9a), S(0x16fe2), S(0x1bc9f), R(0x1da87,0x1da8b), R(0x1e95e,0x1e95f)
    ];

    if codepoint <= 0x7f {
        return is_punct(codepoint);
    }
    return md_unicode_bsearch__(codepoint, &PUNCT_MAP);
}

pub fn md_get_unicode_fold_info(codepoint: u32, mut info: MDUnicodeFoldInfo) {
    let mut FOLD_MAP_1: Vec<u32> = vec![
        R(0x0041,0x005a), S(0x00b5), R(0x00c0,0x00d6), R(0x00d8,0x00de), R(0x0100,0x012e), R(0x0132,0x0136),
        R(0x0139,0x0147), R(0x014a,0x0176), S(0x0178), R(0x0179,0x017d), S(0x017f), S(0x0181), S(0x0182),
        S(0x0184), S(0x0186), S(0x0187), S(0x0189), S(0x018a), S(0x018b), S(0x018e), S(0x018f), S(0x0190),
        S(0x0191), S(0x0193), S(0x0194), S(0x0196), S(0x0197), S(0x0198), S(0x019c), S(0x019d), S(0x019f),
        R(0x01a0,0x01a4), S(0x01a6), S(0x01a7), S(0x01a9), S(0x01ac), S(0x01ae), S(0x01af), S(0x01b1), S(0x01b2),
        S(0x01b3), S(0x01b5), S(0x01b7), S(0x01b8), S(0x01bc), S(0x01c4), S(0x01c5), S(0x01c7), S(0x01c8),
        S(0x01ca), R(0x01cb,0x01db), R(0x01de,0x01ee), S(0x01f1), S(0x01f2), S(0x01f4), S(0x01f6), S(0x01f7),
        R(0x01f8,0x021e), S(0x0220), R(0x0222,0x0232), S(0x023a), S(0x023b), S(0x023d), S(0x023e), S(0x0241),
        S(0x0243), S(0x0244), S(0x0245), R(0x0246,0x024e), S(0x0345), S(0x0370), S(0x0372), S(0x0376), S(0x037f),
        S(0x0386), R(0x0388,0x038a), S(0x038c), S(0x038e), S(0x038f), R(0x0391,0x03a1), R(0x03a3,0x03ab),
        S(0x03c2), S(0x03cf), S(0x03d0), S(0x03d1), S(0x03d5), S(0x03d6), R(0x03d8,0x03ee), S(0x03f0), S(0x03f1),
        S(0x03f4), S(0x03f5), S(0x03f7), S(0x03f9), S(0x03fa), R(0x03fd,0x03ff), R(0x0400,0x040f),
        R(0x0410,0x042f), R(0x0460,0x0480), R(0x048a,0x04be), S(0x04c0), R(0x04c1,0x04cd), R(0x04d0,0x052e),
        R(0x0531,0x0556), R(0x10a0,0x10c5), S(0x10c7), S(0x10cd), R(0x13f8,0x13fd), S(0x1c80), S(0x1c81),
        S(0x1c82), S(0x1c83), S(0x1c84), S(0x1c85), S(0x1c86), S(0x1c87), S(0x1c88), R(0x1c90,0x1cba),
        R(0x1cbd,0x1cbf), R(0x1e00,0x1e94), S(0x1e9b), R(0x1ea0,0x1efe), R(0x1f08,0x1f0f), R(0x1f18,0x1f1d),
        R(0x1f28,0x1f2f), R(0x1f38,0x1f3f), R(0x1f48,0x1f4d), S(0x1f59), S(0x1f5b), S(0x1f5d), S(0x1f5f),
        R(0x1f68,0x1f6f), S(0x1fb8), S(0x1fb9), S(0x1fba), S(0x1fbb), S(0x1fbe), R(0x1fc8,0x1fcb), S(0x1fd8),
        S(0x1fd9), S(0x1fda), S(0x1fdb), S(0x1fe8), S(0x1fe9), S(0x1fea), S(0x1feb), S(0x1fec), S(0x1ff8),
        S(0x1ff9), S(0x1ffa), S(0x1ffb), S(0x2126), S(0x212a), S(0x212b), S(0x2132), R(0x2160,0x216f), S(0x2183),
        R(0x24b6,0x24cf), R(0x2c00,0x2c2e), S(0x2c60), S(0x2c62), S(0x2c63), S(0x2c64), R(0x2c67,0x2c6b),
        S(0x2c6d), S(0x2c6e), S(0x2c6f), S(0x2c70), S(0x2c72), S(0x2c75), S(0x2c7e), S(0x2c7f), R(0x2c80,0x2ce2),
        S(0x2ceb), S(0x2ced), S(0x2cf2), R(0xa640,0xa66c), R(0xa680,0xa69a), R(0xa722,0xa72e), R(0xa732,0xa76e),
        S(0xa779), S(0xa77b), S(0xa77d), R(0xa77e,0xa786), S(0xa78b), S(0xa78d), S(0xa790), S(0xa792),
        R(0xa796,0xa7a8), S(0xa7aa), S(0xa7ab), S(0xa7ac), S(0xa7ad), S(0xa7ae), S(0xa7b0), S(0xa7b1), S(0xa7b2),
        S(0xa7b3), R(0xa7b4,0xa7be), S(0xa7c2), S(0xa7c4), S(0xa7c5), S(0xa7c6), S(0xa7c7), S(0xa7c9), S(0xa7f5),
        R(0xab70,0xabbf), R(0xff21,0xff3a), R(0x10400,0x10427), R(0x104b0,0x104d3), R(0x10c80,0x10cb2),
        R(0x118a0,0x118bf), R(0x16e40,0x16e5f), R(0x1e900,0x1e921)
    ];

    let mut FOLD_MAP_1_DATA: Vec<u32> = vec![
        0x0061, 0x007a, 0x03bc, 0x00e0, 0x00f6, 0x00f8, 0x00fe, 0x0101, 0x012f, 0x0133, 0x0137, 0x013a, 0x0148,
        0x014b, 0x0177, 0x00ff, 0x017a, 0x017e, 0x0073, 0x0253, 0x0183, 0x0185, 0x0254, 0x0188, 0x0256, 0x0257,
        0x018c, 0x01dd, 0x0259, 0x025b, 0x0192, 0x0260, 0x0263, 0x0269, 0x0268, 0x0199, 0x026f, 0x0272, 0x0275,
        0x01a1, 0x01a5, 0x0280, 0x01a8, 0x0283, 0x01ad, 0x0288, 0x01b0, 0x028a, 0x028b, 0x01b4, 0x01b6, 0x0292,
        0x01b9, 0x01bd, 0x01c6, 0x01c6, 0x01c9, 0x01c9, 0x01cc, 0x01cc, 0x01dc, 0x01df, 0x01ef, 0x01f3, 0x01f3,
        0x01f5, 0x0195, 0x01bf, 0x01f9, 0x021f, 0x019e, 0x0223, 0x0233, 0x2c65, 0x023c, 0x019a, 0x2c66, 0x0242,
        0x0180, 0x0289, 0x028c, 0x0247, 0x024f, 0x03b9, 0x0371, 0x0373, 0x0377, 0x03f3, 0x03ac, 0x03ad, 0x03af,
        0x03cc, 0x03cd, 0x03ce, 0x03b1, 0x03c1, 0x03c3, 0x03cb, 0x03c3, 0x03d7, 0x03b2, 0x03b8, 0x03c6, 0x03c0,
        0x03d9, 0x03ef, 0x03ba, 0x03c1, 0x03b8, 0x03b5, 0x03f8, 0x03f2, 0x03fb, 0x037b, 0x037d, 0x0450, 0x045f,
        0x0430, 0x044f, 0x0461, 0x0481, 0x048b, 0x04bf, 0x04cf, 0x04c2, 0x04ce, 0x04d1, 0x052f, 0x0561, 0x0586,
        0x2d00, 0x2d25, 0x2d27, 0x2d2d, 0x13f0, 0x13f5, 0x0432, 0x0434, 0x043e, 0x0441, 0x0442, 0x0442, 0x044a,
        0x0463, 0xa64b, 0x10d0, 0x10fa, 0x10fd, 0x10ff, 0x1e01, 0x1e95, 0x1e61, 0x1ea1, 0x1eff, 0x1f00, 0x1f07,
        0x1f10, 0x1f15, 0x1f20, 0x1f27, 0x1f30, 0x1f37, 0x1f40, 0x1f45, 0x1f51, 0x1f53, 0x1f55, 0x1f57, 0x1f60,
        0x1f67, 0x1fb0, 0x1fb1, 0x1f70, 0x1f71, 0x03b9, 0x1f72, 0x1f75, 0x1fd0, 0x1fd1, 0x1f76, 0x1f77, 0x1fe0,
        0x1fe1, 0x1f7a, 0x1f7b, 0x1fe5, 0x1f78, 0x1f79, 0x1f7c, 0x1f7d, 0x03c9, 0x006b, 0x00e5, 0x214e, 0x2170,
        0x217f, 0x2184, 0x24d0, 0x24e9, 0x2c30, 0x2c5e, 0x2c61, 0x026b, 0x1d7d, 0x027d, 0x2c68, 0x2c6c, 0x0251,
        0x0271, 0x0250, 0x0252, 0x2c73, 0x2c76, 0x023f, 0x0240, 0x2c81, 0x2ce3, 0x2cec, 0x2cee, 0x2cf3, 0xa641,
        0xa66d, 0xa681, 0xa69b, 0xa723, 0xa72f, 0xa733, 0xa76f, 0xa77a, 0xa77c, 0x1d79, 0xa77f, 0xa787, 0xa78c,
        0x0265, 0xa791, 0xa793, 0xa797, 0xa7a9, 0x0266, 0x025c, 0x0261, 0x026c, 0x026a, 0x029e, 0x0287, 0x029d,
        0xab53, 0xa7b5, 0xa7bf, 0xa7c3, 0xa794, 0x0282, 0x1d8e, 0xa7c8, 0xa7ca, 0xa7f6, 0x13a0, 0x13ef, 0xff41,
        0xff5a, 0x10428, 0x1044f, 0x104d8, 0x104fb, 0x10cc0, 0x10cf2, 0x118c0, 0x118df, 0x16e60, 0x16e7f, 0x1e922,
        0x1e943
    ];

    let mut FOLD_MAP_2: Vec<u32> = vec![
        S(0x00df), S(0x0130), S(0x0149), S(0x01f0), S(0x0587), S(0x1e96), S(0x1e97), S(0x1e98), S(0x1e99),
        S(0x1e9a), S(0x1e9e), S(0x1f50), R(0x1f80,0x1f87), R(0x1f88,0x1f8f), R(0x1f90,0x1f97), R(0x1f98,0x1f9f),
        R(0x1fa0,0x1fa7), R(0x1fa8,0x1faf), S(0x1fb2), S(0x1fb3), S(0x1fb4), S(0x1fb6), S(0x1fbc), S(0x1fc2),
        S(0x1fc3), S(0x1fc4), S(0x1fc6), S(0x1fcc), S(0x1fd6), S(0x1fe4), S(0x1fe6), S(0x1ff2), S(0x1ff3),
        S(0x1ff4), S(0x1ff6), S(0x1ffc), S(0xfb00), S(0xfb01), S(0xfb02), S(0xfb05), S(0xfb06), S(0xfb13),
        S(0xfb14), S(0xfb15), S(0xfb16), S(0xfb17)
    ];

    let mut FOLD_MAP_2_DATA: Vec<u32> = vec![
        0x0073,0x0073, 0x0069,0x0307, 0x02bc,0x006e, 0x006a,0x030c, 0x0565,0x0582, 0x0068,0x0331, 0x0074,0x0308,
        0x0077,0x030a, 0x0079,0x030a, 0x0061,0x02be, 0x0073,0x0073, 0x03c5,0x0313, 0x1f00,0x03b9, 0x1f07,0x03b9,
        0x1f00,0x03b9, 0x1f07,0x03b9, 0x1f20,0x03b9, 0x1f27,0x03b9, 0x1f20,0x03b9, 0x1f27,0x03b9, 0x1f60,0x03b9,
        0x1f67,0x03b9, 0x1f60,0x03b9, 0x1f67,0x03b9, 0x1f70,0x03b9, 0x03b1,0x03b9, 0x03ac,0x03b9, 0x03b1,0x0342,
        0x03b1,0x03b9, 0x1f74,0x03b9, 0x03b7,0x03b9, 0x03ae,0x03b9, 0x03b7,0x0342, 0x03b7,0x03b9, 0x03b9,0x0342,
        0x03c1,0x0313, 0x03c5,0x0342, 0x1f7c,0x03b9, 0x03c9,0x03b9, 0x03ce,0x03b9, 0x03c9,0x0342, 0x03c9,0x03b9,
        0x0066,0x0066, 0x0066,0x0069, 0x0066,0x006c, 0x0073,0x0074, 0x0073,0x0074, 0x0574,0x0576, 0x0574,0x0565,
        0x0574,0x056b, 0x057e,0x0576, 0x0574,0x056d
    ];

    let mut FOLD_MAP_3: Vec<u32> = vec![
        S(0x0390), S(0x03b0), S(0x1f52), S(0x1f54), S(0x1f56), S(0x1fb7), S(0x1fc7), S(0x1fd2), S(0x1fd3),
        S(0x1fd7), S(0x1fe2), S(0x1fe3), S(0x1fe7), S(0x1ff7), S(0xfb03), S(0xfb04)
    ];

    let mut FOLD_MAP_3_DATA: Vec<u32> = vec![
        0x03b9,0x0308,0x0301, 0x03c5,0x0308,0x0301, 0x03c5,0x0313,0x0300, 0x03c5,0x0313,0x0301,
        0x03c5,0x0313,0x0342, 0x03b1,0x0342,0x03b9, 0x03b7,0x0342,0x03b9, 0x03b9,0x0308,0x0300,
        0x03b9,0x0308,0x0301, 0x03b9,0x0308,0x0342, 0x03c5,0x0308,0x0300, 0x03c5,0x0308,0x0301,
        0x03c5,0x0308,0x0342, 0x03c9,0x0342,0x03b9, 0x0066,0x0066,0x0069, 0x0066,0x0066,0x006c
    ];

    let mut FOLD_MAP_LIST: Vec<MDFoldMap> = vec![
        MDFoldMap {
            map: FOLD_MAP_1,
            data: FOLD_MAP_1_DATA,
            n_codepoints: 1
        },
        MDFoldMap {
            map: FOLD_MAP_2,
            data: FOLD_MAP_2_DATA,
            n_codepoints: 2
        },
        MDFoldMap {
            map: FOLD_MAP_3,
            data: FOLD_MAP_3_DATA,
            n_codepoints: 3
        },
    ];

    if codepoint <= 0x7f {
        info.codepoints[0] = codepoint;

        if is_upper(codepoint) {
            info.codepoints[0] += 'a' as u32 - 'A' as u32;
        }
        info.n_codepoints = 1;
        return;
    }

    for i in 0..FOLD_MAP_LIST.len() {
        let mut index = md_unicode_bsearch__(codepoint, &FOLD_MAP_LIST[i as usize].map);

        if index >= 0 {
            let n_codepoints = FOLD_MAP_LIST[i].n_codepoints;
            let map = &FOLD_MAP_LIST[i].map;
            let cp_idx = (index as u32  * n_codepoints) as usize;
            let codepoints = &FOLD_MAP_LIST[i].data[cp_idx..];

            info.codepoints = <[u32; 3]>::try_from(codepoints).unwrap();
            info.n_codepoints = n_codepoints;

            if FOLD_MAP_LIST[i].map[index as usize] != codepoint {
                /* The found mapping maps whole range of codepoints,
                     * i.e. we have to offset info->codepoints[0] accordingly. */
                if (map[index as usize] & 0x00ffffff) + 1 == codepoints[0] {
                    /* Alternating type of the range. */
                    info.codepoints[0] = codepoint + (if (codepoint & 0x1) == (map[index as usize] & 0x1) {1} else {0});
                } else {
                    /* Range to range kind of mapping. */
                    info.codepoints[0] += (codepoint - (map[index as usize] & 0x00ffffff));
                }
            }
            return;
        }
    }

    /* No mapping found. Map the codepoint to itself. */
    info.codepoints[0] = codepoint;
    info.n_codepoints = 1;
}

pub fn md_decode_utf8__(text: &str, mut p_size: i32) -> u32 {
    let mut chars = text.chars();
    if !is_utf8_lead1(chars.nth(0).unwrap() as u32) {
        if is_utf8_lead2(chars.nth(0).unwrap() as u32) {
            if 1 < text.len() && is_utf8_tail(chars.nth(1).unwrap() as u32) {
                if p_size >= 0 {
                    p_size = 2;
                }
                return
                    ((chars.nth(0).unwrap() as u32 & 0x1f) << 6) |
                    ((chars.nth(1).unwrap() as u32 & 0x3f) << 0) ;
            }
        } else if is_utf8_lead3(chars.nth(0).unwrap() as u32) {
            if 2 < text.len() &&
                is_utf8_tail(chars.nth(1).unwrap() as u32) &&
                is_utf8_tail(chars.nth(2).unwrap() as u32)
            {
                if p_size >= 0 {
                    p_size = 3;
                }
                return
                    ((chars.nth(0).unwrap() as u32 & 0x0f) << 12) |
                        ((chars.nth(1).unwrap() as u32 & 0x3f) << 6) |
                        ((chars.nth(2).unwrap() as u32 & 0x3f) << 0) ;
            }
        } else if is_utf8_lead4(chars.nth(0).unwrap() as u32) {
            if 3 < text.len() &&
                is_utf8_tail(chars.nth(1).unwrap() as u32) &&
                is_utf8_tail(chars.nth(2).unwrap() as u32) &&
                is_utf8_tail(chars.nth(3).unwrap() as u32)
            {
                if p_size >= 0 {
                    p_size = 4;
                }
                return
                    ((chars.nth(0).unwrap() as u32 & 0x07) << 18) |
                        ((chars.nth(1).unwrap() as u32 & 0x3f) << 12) |
                        ((chars.nth(2).unwrap() as u32 & 0x3f) << 6) |
                        ((chars.nth(3).unwrap() as u32 & 0x3f) << 0);
            }
        }
    }

    if p_size >= 0 {
        p_size = 1;
    }
    return chars.nth(0).unwrap() as u32;
}

pub fn md_decode_utf8_before__(ctx: MDCtx, off: u32) -> u32 {
    if !is_utf8_lead1(ch(off - 1, &ctx)) {
        if off > 1 &&
            is_utf8_lead2(ch(off - 2, &ctx)) &&
            is_utf8_tail(ch(off - 1, &ctx))
        {
            return (ch(off - 2, &ctx) & 0x1f) << 6 |
                (ch(off - 1, &ctx) & 0x3f) << 0;
        }

        if off > 2 &&
            is_utf8_lead3(ch(off - 3, &ctx)) &&
            is_utf8_tail(ch(off - 2, &ctx)) &&
            is_utf8_tail(ch(off - 2, &ctx))
        {
            return (ch(off - 3, &ctx) & 0x0f) << 12 |
                (ch(off - 2, &ctx) & 0x3f) << 6 |
                (ch(off - 1, &ctx) & 0x3f) << 0;
        }

        if off > 3 &&
            is_utf8_lead4(ch(off - 4, &ctx)) &&
            is_utf8_tail(ch(off - 3, &ctx)) &&
            is_utf8_tail(ch(off - 2, &ctx)) &&
            is_utf8_tail(ch(off - 1, &ctx))
        {
            return (ch(off - 4, &ctx) & 0x07) << 18 |
                (ch(off - 3, &ctx) & 0x3f) << 12 |
                (ch(off - 2, &ctx) & 0x3f) << 6 |
                (ch(off - 1, &ctx) & 0x3f) << 0;
        }
    }

    return ch(off - 1, &ctx);
}


/*************************************
 ***  Helper string manipulations  ***
 *************************************/

pub fn md_merge_lines(ctx: MDCtx, beg: u32, end: u32,
                      lines: Vec<&MDLine>, n_lines: u32,
                      line_break_replacement_char: u32,

)
{

}



/* Parse the Markdown document stored in the string 'text' of size 'size'.
 * The parser provides callbacks to be called during the parsing so the
 * caller can render the document on the screen or convert the Markdown
 * to another format.
 *
 * Zero is returned on success. If a runtime error occurs (e.g. a memory
 * fails), -1 is returned. If the processing is aborted due any callback
 * returning non-zero, the return value of the callback is returned.
 */
pub fn md_parse(input: &str, parser: &MDParser, userdata: &MDHTMLTag) -> u32 {
    return 0;
}