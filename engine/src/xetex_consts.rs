use crate::xetex_ini::EQTB;

pub trait IsTexNull {
    fn is_texnull(&self) -> bool;
}
impl IsTexNull for i32 {
    fn is_texnull(&self) -> bool {
        *self == TEX_NULL
    }
}

pub(crate) type placeholdertype = i32;
pub(crate) const MIN_HALFWORD: placeholdertype = -0x0FFFFFFF;
pub(crate) const MAX_HALFWORD: placeholdertype = 0x3FFFFFFF;

/// a null "pointer"
pub(crate) const TEX_NULL: placeholdertype = MIN_HALFWORD;
/// "the largest positive value that TeX knows"
pub(crate) const TEX_INFINITY: placeholdertype = 0x7FFFFFFF;
/// "signifies a missing item" in rule nodes */
pub(crate) const NULL_FLAG: placeholdertype = -0x40000000;
/// "denotes default_rule_thickness"
pub(crate) const DEFAULT_CODE: placeholdertype = 0x40000000;

/* characters
 *
 * TeX thinks there are only 256 character but we know better. We use UTF16
 * codepoints. Actual Unicode character codes can exceed this, up to
 * BIGGEST_USV. "USV" here means Unicode Scalar Value. */

/// must be <= max_quarterword
pub(crate) const BIGGEST_CHAR: placeholdertype = u16::MAX as i32;
pub(crate) const BIGGEST_USV: usize = 0x10FFFF;
pub(crate) const NUMBER_USVS: usize = BIGGEST_USV + 1;
pub(crate) const TOO_BIG_USV: usize = BIGGEST_USV + 1;

/* Various buffer sizes */

/// max number of control sequences
pub(crate) const HASH_SIZE: usize = 15000;
/// "a prime number equal to about 85% of hash_size
pub(crate) const HASH_PRIME: usize = 8501;

pub(crate) const MAX_FONT_MAX: usize = 9000;

pub(crate) const NUMBER_MATH_FAMILIES: usize = 256;
pub(crate) const TEXT_SIZE: usize = 0;
pub(crate) const SCRIPT_SIZE: usize = NUMBER_MATH_FAMILIES;
pub(crate) const SCRIPT_SCRIPT_SIZE: usize = 2 * NUMBER_MATH_FAMILIES;
pub(crate) const NUMBER_MATH_FONTS: usize = 3 * NUMBER_MATH_FAMILIES;

pub(crate) const NUMBER_REGS: usize = 256;

/// the size of our main "mem" array, minus 1; classically this is
/// configurable, but we hardcode it.
pub(crate) const MEM_TOP: usize = 4999999;

/* fixed locations in the "mem" array */
pub(crate) const PAGE_INS_HEAD: usize = MEM_TOP;
pub(crate) const CONTRIB_HEAD: usize = MEM_TOP - 1;
pub(crate) const PAGE_HEAD: usize = MEM_TOP - 2;
pub(crate) const TEMP_HEAD: usize = MEM_TOP - 3;
pub(crate) const HOLD_HEAD: usize = MEM_TOP - 4;
pub(crate) const ADJUST_HEAD: usize = MEM_TOP - 5;
/// note: two words
pub(crate) const ACTIVE_LIST: usize = MEM_TOP - 7;
pub(crate) const ALIGN_HEAD: usize = MEM_TOP - 8;
pub(crate) const END_SPAN: usize = MEM_TOP - 9;
pub(crate) const OMIT_TEMPLATE: usize = MEM_TOP - 10;
pub(crate) const NULL_LIST: usize = MEM_TOP - 11;
pub(crate) const LIG_TRICK: usize = MEM_TOP - 12;
/// note: same as LIG_TRICK
pub(crate) const GARBAGE: usize = MEM_TOP - 12;
pub(crate) const BACKUP_HEAD: usize = MEM_TOP - 13;
pub(crate) const PRE_ADJUST_HEAD: usize = MEM_TOP - 14;

/* equivalents table offsets */

/// "region 1": active character equivalents
pub(crate) const ACTIVE_BASE: usize = 1;
pub(crate) const SINGLE_BASE: usize = ACTIVE_BASE + NUMBER_USVS;
pub(crate) const NULL_CS: usize = SINGLE_BASE + NUMBER_USVS;
/// "region 2": hash table
pub(crate) const HASH_BASE: usize = (NULL_CS + 1) as usize;
pub(crate) const FROZEN_CONTROL_SEQUENCE: usize = HASH_BASE + HASH_SIZE;
pub(crate) const FROZEN_PROTECTION: usize = FROZEN_CONTROL_SEQUENCE + 0;
pub(crate) const FROZEN_CR: usize = FROZEN_CONTROL_SEQUENCE + 1;
pub(crate) const FROZEN_END_GROUP: usize = FROZEN_CONTROL_SEQUENCE + 2;
pub(crate) const FROZEN_RIGHT: usize = FROZEN_CONTROL_SEQUENCE + 3;
pub(crate) const FROZEN_FI: usize = FROZEN_CONTROL_SEQUENCE + 4;
pub(crate) const FROZEN_END_TEMPLATE: usize = FROZEN_CONTROL_SEQUENCE + 5;
pub(crate) const FROZEN_ENDV: usize = FROZEN_CONTROL_SEQUENCE + 6;
pub(crate) const FROZEN_RELAX: usize = FROZEN_CONTROL_SEQUENCE + 7;
pub(crate) const END_WRITE: usize = FROZEN_CONTROL_SEQUENCE + 8;
pub(crate) const FROZEN_DONT_EXPAND: usize = FROZEN_CONTROL_SEQUENCE + 9;
pub(crate) const FROZEN_SPECIAL: usize = FROZEN_CONTROL_SEQUENCE + 10;
pub(crate) const FROZEN_PRIMITIVE: usize = FROZEN_CONTROL_SEQUENCE + 11;
pub(crate) const FROZEN_NULL_FONT: usize = FROZEN_CONTROL_SEQUENCE + 12;
/// nominally minus FONT_BASE, but that's 0
pub(crate) const FONT_ID_BASE: usize = FROZEN_NULL_FONT;
pub(crate) const UNDEFINED_CONTROL_SEQUENCE: usize = FROZEN_NULL_FONT + MAX_FONT_MAX + 1;

/// "region 3": glue values
pub(crate) const GLUE_BASE: usize = UNDEFINED_CONTROL_SEQUENCE + 1;

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, enumn::N)]
pub(crate) enum GluePar {
    line_skip = 0,
    baseline_skip = 1,
    par_skip = 2,
    above_display_skip = 3,
    below_display_skip = 4,
    above_display_short_skip = 5,
    below_display_short_skip = 6,
    left_skip = 7,
    right_skip = 8,
    top_skip = 9,
    split_top_skip = 10,
    tab_skip = 11,
    space_skip = 12,
    xspace_skip = 13,
    par_fill_skip = 14,
    xetex_linebreak_skip = 15,
    thin_mu_skip = 16,
    med_mu_skip = 17,
    thick_mu_skip = 18,
}

pub(crate) const GLUE_PARS: usize = 19;

pub(crate) unsafe fn GLUEPAR(s: GluePar) -> &'static mut i32 {
    &mut EQTB[GLUE_BASE + s as usize].val
}

pub(crate) const SKIP_BASE: usize = GLUE_BASE + GLUE_PARS;
pub(crate) unsafe fn SKIP_REG(n: usize) -> &'static mut i32 {
    &mut EQTB[SKIP_BASE + n].val
}

pub(crate) const MU_SKIP_BASE: usize = SKIP_BASE + NUMBER_REGS;
pub(crate) unsafe fn MU_SKIP_REG(n: usize) -> &'static mut i32 {
    &mut EQTB[MU_SKIP_BASE + n].val
}

/* "region 4": local halfword values like baselineskip. Some of these are
 * used as arguments to ASSIGN_TOKS, SET_SHAPE, etc. */

pub(crate) const LOCAL_BASE: usize = MU_SKIP_BASE + NUMBER_REGS;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, enumn::N)]
pub(crate) enum Local {
    par_shape = 0,
    output_routine = 1,
    every_par = 2,
    every_math = 3,
    every_display = 4,
    every_hbox = 5,
    every_vbox = 6,
    every_job = 7,
    every_cr = 8,
    err_help = 9,
    every_eof = 10,
    xetex_inter_char = 11,
    TectonicCodaTokens = 12,
}

pub(crate) const NUM_LOCALS: usize = 13;

pub(crate) fn LOCAL(n: Local) -> &'static mut i32 {
    unsafe { &mut EQTB[LOCAL_BASE + n as usize].val }
}

pub(crate) const TOKS_BASE: usize = LOCAL_BASE + NUM_LOCALS;
pub(crate) unsafe fn TOKS_REG(n: usize) -> &'static mut i32 {
    &mut EQTB[TOKS_BASE + n].val
}

pub(crate) const ETEX_PEN_BASE: usize = TOKS_BASE + NUMBER_REGS;
pub(crate) const INTER_LINE_PENALTIES_LOC: usize = ETEX_PEN_BASE + 0;
pub(crate) const CLUB_PENALTIES_LOC: usize = ETEX_PEN_BASE + 1;
pub(crate) const WIDOW_PENALTIES_LOC: usize = ETEX_PEN_BASE + 2;
pub(crate) const DISPLAY_WIDOW_PENALTIES_LOC: usize = ETEX_PEN_BASE + 3;
pub(crate) const ETEX_PENS: usize = ETEX_PEN_BASE + 4;

pub(crate) const BOX_BASE: usize = ETEX_PENS;
pub(crate) unsafe fn BOX_REG(n: usize) -> &'static mut i32 {
    &mut EQTB[BOX_BASE + n].val
}

pub(crate) const CUR_FONT_LOC: usize = BOX_BASE + NUMBER_REGS;
pub(crate) const MATH_FONT_BASE: usize = CUR_FONT_LOC + 1;

pub(crate) unsafe fn MATH_FONT(n: usize) -> usize {
    EQTB[MATH_FONT_BASE + n].val as usize
}

pub(crate) const CAT_CODE_BASE: usize = MATH_FONT_BASE + NUMBER_MATH_FONTS;
pub(crate) unsafe fn CAT_CODE(n: usize) -> &'static mut i32 {
    &mut EQTB[CAT_CODE_BASE + n].val
}

pub(crate) const LC_CODE_BASE: usize = CAT_CODE_BASE + NUMBER_USVS;
pub(crate) unsafe fn LC_CODE(n: usize) -> &'static mut i32 {
    &mut EQTB[LC_CODE_BASE + n].val
}

pub(crate) const UC_CODE_BASE: usize = LC_CODE_BASE + NUMBER_USVS;
pub(crate) unsafe fn UC_CODE(n: usize) -> &'static mut i32 {
    &mut EQTB[UC_CODE_BASE + n].val
}

pub(crate) const SF_CODE_BASE: usize = UC_CODE_BASE + NUMBER_USVS;
pub(crate) unsafe fn SF_CODE(n: usize) -> &'static mut i32 {
    &mut EQTB[SF_CODE_BASE + n].val
}

pub(crate) const MATH_CODE_BASE: usize = SF_CODE_BASE + NUMBER_USVS;
pub(crate) unsafe fn MATH_CODE(n: usize) -> &'static mut i32 {
    &mut EQTB[MATH_CODE_BASE + n].val
}

pub(crate) const CHAR_SUB_CODE_BASE: usize = MATH_CODE_BASE + NUMBER_USVS;
pub(crate) unsafe fn CHAR_SUB_CODE(n: usize) -> &'static mut i32 {
    &mut EQTB[CHAR_SUB_CODE_BASE + n].val
}

/* "region 5": current fullword integers like hyphenation penalty */

pub(crate) const INT_BASE: usize = CHAR_SUB_CODE_BASE + NUMBER_USVS;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, enumn::N)]
pub(crate) enum IntPar {
    pretolerance = 0,
    tolerance = 1,
    line_penalty = 2,
    hyphen_penalty = 3,
    ex_hyphen_penalty = 4,
    club_penalty = 5,
    widow_penalty = 6,
    display_widow_penalty = 7,
    broken_penalty = 8,
    bin_op_penalty = 9,
    rel_penalty = 10,
    pre_display_penalty = 11,
    post_display_penalty = 12,
    inter_line_penalty = 13,
    double_hyphen_demerits = 14,
    final_hyphen_demerits = 15,
    adj_demerits = 16,
    mag = 17,
    delimiter_factor = 18,
    looseness = 19,
    time = 20,
    day = 21,
    month = 22,
    year = 23,
    show_box_breadth = 24,
    show_box_depth = 25,
    hbadness = 26,
    vbadness = 27,
    pausing = 28,
    tracing_online = 29,
    tracing_macros = 30,
    tracing_stats = 31,
    tracing_paragraphs = 32,
    tracing_pages = 33,
    tracing_output = 34,
    tracing_lost_chars = 35,
    tracing_commands = 36,
    tracing_restores = 37,
    uc_hyph = 38,
    output_penalty = 39,
    max_dead_cycles = 40,
    hang_after = 41,
    floating_penalty = 42,
    global_defs = 43,
    cur_fam = 44,
    escape_char = 45,
    default_hyphen_char = 46,
    default_skew_char = 47,
    end_line_char = 48,
    new_line_char = 49,
    language = 50,
    left_hyphen_min = 51,
    right_hyphen_min = 52,
    holding_inserts = 53,
    error_context_lines = 54,
    /// TEX_INT_PARS = WEB2C_INT_BASE
    char_sub_def_min = 55,
    char_sub_def_max = 56,
    tracing_char_sub_def = 57,
    /// = WEB2C_INT_PARS = ETEX_INT_BASE
    tracing_assigns = 58,
    tracing_groups = 59,
    tracing_ifs = 60,
    tracing_scan_tokens = 61,
    tracing_nesting = 62,
    pre_display_correction = 63,
    last_line_fit = 64,
    saving_vdiscards = 65,
    saving_hyphs = 66,
    suppress_fontnotfound_error = 67,
    xetex_linebreak_locale = 68,
    xetex_linebreak_penalty = 69,
    xetex_protrude_chars = 70,
    texxet = 71,
    xetex_dash_break = 72,
    xetex_upwards = 73,
    xetex_use_glyph_metrics = 74,
    xetex_inter_char_tokens = 75,
    xetex_input_normalization = 76,
    xetex_default_input_mode = 77,
    xetex_default_input_encoding = 78,
    xetex_tracing_fonts = 79,
    xetex_interword_space_shaping = 80,
    xetex_generate_actual_text = 81,
    xetex_hyphenatable_length = 82,
    synctex = 83,
    pdfoutput = 84,
}

pub(crate) const INT_PARS: usize = 85;

pub(crate) unsafe fn INTPAR(x: IntPar) -> &'static mut i32 {
    &mut EQTB[INT_BASE + x as usize].val
}

pub(crate) const COUNT_BASE: usize = INT_BASE + INT_PARS;
pub(crate) unsafe fn COUNT_REG(n: usize) -> &'static mut i32 {
    &mut EQTB[COUNT_BASE + n].val
}

pub(crate) const DEL_CODE_BASE: usize = COUNT_BASE + NUMBER_REGS;
pub(crate) unsafe fn DEL_CODE(n: usize) -> &'static mut i32 {
    &mut EQTB[DEL_CODE_BASE + n].val
}

/* "region 6": current fullword dimensions like hsize */

pub(crate) const DIMEN_BASE: usize = DEL_CODE_BASE + NUMBER_USVS;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, enumn::N)]
pub(crate) enum DimenPar {
    par_indent = 0,
    math_surround = 1,
    line_skip_limit = 2,
    hsize = 3,
    vsize = 4,
    max_depth = 5,
    split_max_depth = 6,
    box_max_depth = 7,
    hfuzz = 8,
    vfuzz = 9,
    delimiter_shortfall = 10,
    null_delimiter_space = 11,
    script_space = 12,
    pre_display_size = 13,
    display_width = 14,
    display_indent = 15,
    overfull_rule = 16,
    hang_indent = 17,
    h_offset = 18,
    v_offset = 19,
    emergency_stretch = 20,
    pdf_page_width = 21,
    pdf_page_height = 22,
}

pub(crate) const DIMEN_PARS: usize = 23;

pub(crate) fn DIMENPAR(x: DimenPar) -> &'static mut i32 {
    unsafe { &mut EQTB[DIMEN_BASE + x as usize].val }
}

pub(crate) const SCALED_BASE: usize = DIMEN_BASE + DIMEN_PARS;
pub(crate) unsafe fn SCALED_REG(n: usize) -> &'static mut i32 {
    &mut EQTB[SCALED_BASE + n].val
}

pub(crate) const EQTB_SIZE: usize = SCALED_BASE + NUMBER_REGS - 1;

/// "really" MIN_QUARTERWORD
pub(crate) const LEVEL_ZERO: u16 = 0;
pub(crate) const LEVEL_ONE: u16 = 1;

/* Cmd::SetInteraction */
pub(crate) const BATCH_MODE: u8 = 0;
pub(crate) const NONSTOP_MODE: u8 = 1;
pub(crate) const SCROLL_MODE: u8 = 2;
pub(crate) const ERROR_STOP_MODE: u8 = 3;
pub(crate) const UNSPECIFIED_MODE: u8 = 4;

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, enumn::N)]
pub(crate) enum LR {
    LeftToRight = 0,
    RightToLeft = 1,
}

impl core::ops::Not for LR {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::LeftToRight => Self::RightToLeft,
            Self::RightToLeft => Self::LeftToRight,
        }
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, enumn::N)]
pub(crate) enum LRMode {
    Normal = 0, // TODO: check name
    Reversed = 1,
    DList = 2,
}

impl From<u16> for LRMode {
    fn from(n: u16) -> Self {
        Self::n(n).expect(&format!("Incorrect LRMode = {}", n))
    }
}

/* How many memory words are needed for storing synctex information on various
 * kinds of nodes. This extra size is already included in the *_NODE_SIZE
 * definitions below.
 */
pub(crate) const SYNCTEX_FIELD_SIZE: placeholdertype = 1;

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
pub(crate) enum TextNode {
    HList = 0,
    VList = 1,
    Rule = 2,
    Ins = 3,
    Mark = 4,
    Adjust = 5,
    Ligature = 6,
    Disc = 7,
    WhatsIt = 8,
    Math = 9,
    Glue = 10,
    Kern = 11,
    Penalty = 12,
    Unset = 13,
    Style = 14,
    Choice = 15,
    MarginKern = 40,
}

pub(crate) const INSERTING: TextNode = TextNode::HList;
pub(crate) const SPLIT_UP: TextNode = TextNode::VList;
pub(crate) const DELTA_NODE: TextNode = TextNode::Rule;
pub(crate) const EDGE_NODE: TextNode = TextNode::Style;

impl From<u16> for TextNode {
    fn from(n: u16) -> Self {
        Self::n(n).expect(&format!("Incorrect TextNode = {}", n))
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
pub(crate) enum WhatsItNST {
    Open = 0,
    Write = 1,
    Close = 2,
    Special = 3,
    Language = 4,
    PdfSavePos = 6,
    NativeWord = 40,
    NativeWordAt = 41,
    Glyph = 42,
    Pic = 43,
    Pdf = 44,
}

impl From<u16> for WhatsItNST {
    fn from(n: u16) -> Self {
        Self::n(n).expect(&format!("Incorrect WhatsItNST = {}", n))
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
pub(crate) enum InsNST {
    NS100 = 100, // Unknown
    NS200 = 200,
    NS253 = 253,
}

impl From<u16> for InsNST {
    fn from(n: u16) -> Self {
        Self::n(n).expect(&format!("Incorrect InsNST = {}", n))
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
pub(crate) enum KernNST {
    Normal = 0,
    Explicit = 1,
    AccKern = 2,
    SpaceAdjustment = 3,
}

impl From<u16> for KernNST {
    fn from(n: u16) -> Self {
        Self::n(n).expect(&format!("Incorrect KernNST = {}", n))
    }
}

pub(crate) const IF_NODE_SIZE: placeholdertype = 2;
pub(crate) const PASSIVE_NODE_SIZE: placeholdertype = 2;
pub(crate) const POINTER_NODE_SIZE: placeholdertype = 2;
pub(crate) const SMALL_NODE_SIZE: placeholdertype = 2;
pub(crate) const SPAN_NODE_SIZE: placeholdertype = 2;
pub(crate) const WRITE_NODE_SIZE: placeholdertype = 2;
pub(crate) const ACTIVE_NODE_SIZE_NORMAL: placeholdertype = 3;
pub(crate) const EDGE_NODE_SIZE: placeholdertype = 3;
pub(crate) const MARGIN_KERN_NODE_SIZE: placeholdertype = 3;
pub(crate) const MEDIUM_NODE_SIZE: placeholdertype = 3;
pub(crate) const MOVEMENT_NODE_SIZE: placeholdertype = 3;
pub(crate) const OPEN_NODE_SIZE: placeholdertype = 3;
pub(crate) const STYLE_NODE_SIZE: placeholdertype = 3;
pub(crate) const WORD_NODE_SIZE: placeholdertype = 3;
pub(crate) const EXPR_NODE_SIZE: placeholdertype = 4;
pub(crate) const GLUE_SPEC_SIZE: placeholdertype = 4;
pub(crate) const MARK_CLASS_NODE_SIZE: placeholdertype = 4;
pub(crate) const PAGE_INS_NODE_SIZE: placeholdertype = 4;
pub(crate) const ACTIVE_NODE_SIZE_EXTENDED: placeholdertype = 5;
pub(crate) const GLYPH_NODE_SIZE: placeholdertype = 5;
pub(crate) const INS_NODE_SIZE: placeholdertype = 5;
pub(crate) const RULE_NODE_SIZE: placeholdertype = 5;
pub(crate) const ALIGN_STACK_NODE_SIZE: placeholdertype = 6;
pub(crate) const NATIVE_NODE_SIZE: placeholdertype = 6;
pub(crate) const DELTA_NODE_SIZE: placeholdertype = 7;
pub(crate) const BOX_NODE_SIZE: placeholdertype = 8;
pub(crate) const PIC_NODE_SIZE: placeholdertype = 9;
pub(crate) const INDEX_NODE_SIZE: placeholdertype = 33;

pub(crate) const NOAD_SIZE: placeholdertype = 4;
pub(crate) const ACCENT_NOAD_SIZE: placeholdertype = 5;
pub(crate) const RADICAL_NOAD_SIZE: placeholdertype = 5;
pub(crate) const FRACTION_NOAD_SIZE: placeholdertype = 6;

/* Cmd::MathComp and others */
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
pub(crate) enum MathNode {
    Ord = 16,
    Op = 17,
    Bin = 18,
    Rel = 19,
    Open = 20,
    Close = 21,
    Punct = 22,
    Inner = 23,
    Radical = 24,
    Fraction = 25,
    Under = 26,
    Over = 27,
    Accent = 28,
    VCenter = 29,
    Left = 30,
    Right = 31,
}

impl From<u16> for MathNode {
    fn from(n: u16) -> Self {
        Self::n(n).expect(&format!("Incorrect MathNode = {}", n))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ND {
    Text(TextNode),
    Math(MathNode),
    Unknown(u16),
}

impl From<u16> for ND {
    fn from(n: u16) -> Self {
        match n {
            0..=15 | 40 => Self::Text(TextNode::from(n)),
            16..=31 => Self::Math(MathNode::from(n)),
            _ => Self::Unknown(n),
        }
    }
}
impl From<TextNode> for ND {
    fn from(n: TextNode) -> Self {
        Self::Text(n)
    }
}

impl ND {
    pub fn u16(self) -> u16 {
        match self {
            Self::Text(n) => n as u16,
            Self::Math(n) => n as u16,
            Self::Unknown(n) => n,
        }
    }
}

/* args to Cmd::TopBotMark */
pub(crate) const TOP_MARK_CODE: usize = 0;
pub(crate) const FIRST_MARK_CODE: usize = 1;
pub(crate) const BOT_MARK_CODE: usize = 2;
pub(crate) const SPLIT_FIRST_MARK_CODE: usize = 3;
pub(crate) const SPLIT_BOT_MARK_CODE: usize = 4;

/* MATH_NODE stuff with L/R typesetting extras */
pub(crate) const BEFORE: u16 = 0;
pub(crate) const AFTER: u16 = 1;
pub(crate) const BEGIN_M_CODE: u16 = 2;
pub(crate) const END_M_CODE: u16 = 3;
pub(crate) const L_CODE: u16 = 4;
pub(crate) const R_CODE: u16 = 8;

#[repr(i16)]
#[derive(Clone, Copy, PartialEq, enumn::N)]
pub(crate) enum Expr {
    None = 0,
    Add = 1,
    Sub = 2,
    Mult = 3,
    Div = 4,
    Scale = 5,
}
impl From<i32> for Expr {
    fn from(n: i32) -> Self {
        Self::n(n as i16).expect(&format!("incorrect expression = {}", n))
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, enumn::N)]
pub(crate) enum GroupCode {
    BottomLevel = 0,
    Simple = 1,
    HBox = 2,
    AdjustedHBox = 3,
    VBox = 4,
    VTop = 5,
    Align = 6,
    NoAlign = 7,
    Output = 8,
    Math = 9,
    Disc = 10,
    Insert = 11,
    VCenter = 12,
    MathChoice = 13,
    SemiSimple = 14,
    MathShift = 15,
    MathLeft = 16,
}
impl From<u16> for GroupCode {
    fn from(n: u16) -> Self {
        Self::n(n as u8).unwrap()
    }
}

pub(crate) const SUP_CMD: placeholdertype = 0;
pub(crate) const SUB_CMD: placeholdertype = 1;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, enumn::N)]
pub(crate) enum GlueOrder {
    Normal = 0,
    Fil = 1,
    Fill = 2,
    Filll = 3,
    Incorrect = 4,
}

impl From<u16> for GlueOrder {
    fn from(n: u16) -> Self {
        Self::n(n as u8).unwrap_or(GlueOrder::Incorrect)
    }
}

pub(crate) const FIL: GlueOrder = GlueOrder::Fil;
pub(crate) const FILL: GlueOrder = GlueOrder::Fill;
pub(crate) const FILLL: GlueOrder = GlueOrder::Filll;

pub(crate) const LIG_TAG: placeholdertype = 1;
pub(crate) const LIST_TAG: placeholdertype = 2;
pub(crate) const EXT_TAG: placeholdertype = 3;

pub(crate) const NORMAL: u16 = 0;

/* scanner_status values: */
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ScannerStatus {
    Normal = 0,
    Skipping = 1,
    Defining = 2,
    Matching = 3,
    Aligning = 4,
    Absorbing = 5,
}

/* commands */

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, enumn::N)]
pub(crate) enum Cmd {
    Relax = 0,
    LeftBrace = 1,
    RightBrace = 2,
    MathShift = 3,
    TabMark = 4,
    CarRet = 5,
    MacParam = 6,
    SupMark = 7,
    SubMark = 8,
    EndV = 9,
    Spacer = 10,
    Letter = 11,
    OtherChar = 12,
    ActiveChar = 13,
    Comment = 14,
    DelimNum = 15,
    CharNum = 16,
    MathCharNum = 17,
    Mark = 18,
    XRay = 19,
    MakeBox = 20,
    HMove = 21,
    VMove = 22,
    UnHBox = 23,
    UnVBox = 24,
    RemoveItem = 25,
    HSkip = 26,
    VSkip = 27,
    MSkip = 28,
    Kern = 29,
    MKern = 30,
    LeaderShip = 31,
    HAlign = 32,
    VAlign = 33,
    NoAlign = 34,
    VRule = 35,
    HRule = 36,
    Insert = 37,
    VAdjust = 38,
    IgnoreSpaces = 39,
    AfterAssignment = 40,
    AfterGroup = 41,
    BreakPenalty = 42,
    StartPar = 43,
    ItalCorr = 44,
    Accent = 45,
    MathAccent = 46,
    Discretionary = 47,
    EqNo = 48,
    LeftRight = 49,
    MathComp = 50,
    LimitSwitch = 51,
    Above = 52,
    MathStyle = 53,
    MathChoice = 54,
    NonScript = 55,
    VCenter = 56,
    CaseShift = 57,
    Message = 58,
    Extension = 59,
    InStream = 60,
    BeginGroup = 61,
    EndGroup = 62,
    Omit = 63,
    ExSpace = 64,
    NoBoundary = 65,
    Radical = 66,
    EndCSName = 67,
    CharGiven = 68,
    MathGiven = 69,
    XetexMathGiven = 70,
    LastItem = 71,
    ToksRegister = 72,
    AssignToks = 73,
    AssignInt = 74,
    AssignDimen = 75,
    AssignGlue = 76,
    AssignMuGlue = 77,
    AssignFontDimen = 78,
    AssignFontInt = 79,
    SetAux = 80,
    SetPrevGraf = 81,
    SetPageDimen = 82,
    SetPageInt = 83,
    SetBoxDimen = 84,
    SetShape = 85,
    DefCode = 86,
    XetexDefCode = 87,
    DefFamily = 88,
    SetFont = 89,
    DefFont = 90,
    Register = 91,
    Advance = 92,
    Multiply = 93,
    Divide = 94,
    Prefix = 95,
    Let = 96,
    ShorthandDef = 97,
    ReadToCS = 98,
    Def = 99,
    SetBox = 100,
    HyphData = 101,
    SetInteraction = 102,
    UndefinedCS = 103,
    ExpandAfter = 104,
    NoExpand = 105,
    Input = 106,
    IfTest = 107,
    FiOrElse = 108,
    CSName = 109,
    Convert = 110,
    The = 111,
    TopBotMark = 112,
    Call = 113,
    LongCall = 114,
    OuterCall = 115,
    LongOuterCall = 116,
    EndTemplate = 117,
    DontExpand = 118,
    GlueRef = 119,
    ShapeRef = 120,
    BoxRef = 121,
    Data = 122,
}

impl From<u16> for Cmd {
    fn from(n: u16) -> Self {
        Self::n(n as u8).expect(&format!("incorrect command = {}", n))
    }
}

pub(crate) const ESCAPE: Cmd = Cmd::Relax;
pub(crate) const OUT_PARAM: Cmd = Cmd::CarRet;
pub(crate) const IGNORE: Cmd = Cmd::EndV;

pub(crate) const PAR_END: Cmd = Cmd::ActiveChar;
pub(crate) const MATCH: Cmd = Cmd::ActiveChar;

pub(crate) const END_MATCH: Cmd = Cmd::Comment;
pub(crate) const STOP: Cmd = Cmd::Comment;

pub(crate) const INVALID_CHAR: Cmd = Cmd::DelimNum;

pub(crate) const MAX_NON_PREFIXED_COMMAND: Cmd = Cmd::LastItem;
pub(crate) const MIN_INTERNAL: Cmd = Cmd::CharGiven;
pub(crate) const MAX_INTERNAL: Cmd = Cmd::Register;
pub(crate) const MAX_COMMAND: Cmd = Cmd::SetInteraction;

/* args to Cmd::SetBoxDimen */
pub(crate) const WIDTH_OFFSET: placeholdertype = 1;
pub(crate) const DEPTH_OFFSET: placeholdertype = 2;
pub(crate) const HEIGHT_OFFSET: placeholdertype = 3;

/* args to Cmd::LastItem -- heavily overloaded by (X)eTeX for extensions */
pub(crate) const LAST_NODE_TYPE_CODE: u8 = 3;
pub(crate) const INPUT_LINE_NO_CODE: placeholdertype = 4;
pub(crate) const BADNESS_CODE: placeholdertype = 5;
pub(crate) const ETEX_VERSION_CODE: placeholdertype = 6;
pub(crate) const CURRENT_GROUP_LEVEL_CODE: placeholdertype = 7;
pub(crate) const CURRENT_GROUP_TYPE_CODE: placeholdertype = 8;
pub(crate) const CURRENT_IF_LEVEL_CODE: placeholdertype = 9;
pub(crate) const CURRENT_IF_TYPE_CODE: placeholdertype = 10;
pub(crate) const CURRENT_IF_BRANCH_CODE: placeholdertype = 11;
pub(crate) const GLUE_STRETCH_ORDER_CODE: placeholdertype = 12;
pub(crate) const GLUE_SHRINK_ORDER_CODE: placeholdertype = 13;
pub(crate) const XETEX_VERSION_CODE: placeholdertype = 14;
pub(crate) const XETEX_COUNT_GLYPHS_CODE: placeholdertype = 15;
pub(crate) const XETEX_COUNT_VARIATIONS_CODE: placeholdertype = 16;
pub(crate) const XETEX_VARIATION_CODE: placeholdertype = 17;
pub(crate) const XETEX_FIND_VARIATION_BY_NAME_CODE: placeholdertype = 18;
pub(crate) const XETEX_VARIATION_MIN_CODE: placeholdertype = 19;
pub(crate) const XETEX_VARIATION_MAX_CODE: placeholdertype = 20;
pub(crate) const XETEX_VARIATION_DEFAULT_CODE: placeholdertype = 21;
pub(crate) const XETEX_COUNT_FEATURES_CODE: placeholdertype = 22;
pub(crate) const XETEX_FEATURE_CODE_CODE: placeholdertype = 23;
pub(crate) const XETEX_FIND_FEATURE_BY_NAME_CODE: placeholdertype = 24;
pub(crate) const XETEX_IS_EXCLUSIVE_FEATURE_CODE: placeholdertype = 25;
pub(crate) const XETEX_COUNT_SELECTORS_CODE: placeholdertype = 26;
pub(crate) const XETEX_SELECTOR_CODE_CODE: placeholdertype = 27;
pub(crate) const XETEX_FIND_SELECTOR_BY_NAME_CODE: placeholdertype = 28;
pub(crate) const XETEX_IS_DEFAULT_SELECTOR_CODE: placeholdertype = 29;
pub(crate) const XETEX_OT_COUNT_SCRIPTS_CODE: placeholdertype = 30;
pub(crate) const XETEX_OT_COUNT_LANGUAGES_CODE: placeholdertype = 31;
pub(crate) const XETEX_OT_COUNT_FEATURES_CODE: placeholdertype = 32;
pub(crate) const XETEX_OT_SCRIPT_CODE: placeholdertype = 33;
pub(crate) const XETEX_OT_LANGUAGE_CODE: placeholdertype = 34;
pub(crate) const XETEX_OT_FEATURE_CODE: placeholdertype = 35;
pub(crate) const XETEX_MAP_CHAR_TO_GLYPH_CODE: placeholdertype = 36;
pub(crate) const XETEX_GLYPH_INDEX_CODE: placeholdertype = 37;
pub(crate) const XETEX_FONT_TYPE_CODE: placeholdertype = 38;
pub(crate) const XETEX_FIRST_CHAR_CODE: placeholdertype = 39;
pub(crate) const XETEX_LAST_CHAR_CODE: placeholdertype = 40;
pub(crate) const PDF_LAST_X_POS_CODE: placeholdertype = 41;
pub(crate) const PDF_LAST_Y_POS_CODE: placeholdertype = 42;
pub(crate) const PDF_SHELL_ESCAPE_CODE: placeholdertype = 45;
pub(crate) const XETEX_PDF_PAGE_COUNT_CODE: placeholdertype = 46;
pub(crate) const XETEX_GLYPH_BOUNDS_CODE: placeholdertype = 47;
pub(crate) const FONT_CHAR_WD_CODE: placeholdertype = 48;
pub(crate) const FONT_CHAR_HT_CODE: placeholdertype = 49;
pub(crate) const FONT_CHAR_DP_CODE: placeholdertype = 50;
pub(crate) const FONT_CHAR_IC_CODE: placeholdertype = 51;
pub(crate) const PAR_SHAPE_LENGTH_CODE: placeholdertype = 52;
pub(crate) const PAR_SHAPE_INDENT_CODE: placeholdertype = 53;
pub(crate) const PAR_SHAPE_DIMEN_CODE: placeholdertype = 54;
pub(crate) const GLUE_STRETCH_CODE: placeholdertype = 55;
pub(crate) const GLUE_SHRINK_CODE: placeholdertype = 56;
pub(crate) const MU_TO_GLUE_CODE: placeholdertype = 57;
pub(crate) const GLUE_TO_MU_CODE: placeholdertype = 58;
pub(crate) const ETEX_EXPR: placeholdertype = 59;

/* args to Cmd::Convert -- also heavily overloaded */
pub(crate) const NUMBER_CODE: placeholdertype = 0;
pub(crate) const ROMAN_NUMERAL_CODE: placeholdertype = 1;
pub(crate) const STRING_CODE: placeholdertype = 2;
pub(crate) const MEANING_CODE: placeholdertype = 3;
pub(crate) const FONT_NAME_CODE: placeholdertype = 4;
pub(crate) const ETEX_REVISION_CODE: placeholdertype = 5;
pub(crate) const XETEX_REVISION_CODE: placeholdertype = 6;
pub(crate) const XETEX_VARIATION_NAME_CODE: placeholdertype = 7;
pub(crate) const XETEX_FEATURE_NAME_CODE: placeholdertype = 8;
pub(crate) const XETEX_SELECTOR_NAME_CODE: placeholdertype = 9;
pub(crate) const XETEX_GLYPH_NAME_CODE: placeholdertype = 10;
pub(crate) const LEFT_MARGIN_KERN_CODE: placeholdertype = 11;
pub(crate) const RIGHT_MARGIN_KERN_CODE: placeholdertype = 12;
pub(crate) const XETEX_UCHAR_CODE: placeholdertype = 13;
pub(crate) const XETEX_UCHARCAT_CODE: placeholdertype = 14;
pub(crate) const JOB_NAME_CODE: placeholdertype = 15;
pub(crate) const PDF_STRCMP_CODE: placeholdertype = 43;
pub(crate) const PDF_MDFIVE_SUM_CODE: placeholdertype = 44;

/* args to Cmd::IfTest */
pub(crate) const IF_CHAR_CODE: i16 = 0;
pub(crate) const IF_CODE: u8 = 1;
pub(crate) const IF_CAT_CODE: i16 = 1;
pub(crate) const IF_INT_CODE: i16 = 2;
pub(crate) const IF_DIM_CODE: i16 = 3;
pub(crate) const IF_ODD_CODE: i16 = 4;
pub(crate) const IF_VMODE_CODE: i16 = 5;
pub(crate) const IF_HMODE_CODE: i16 = 6;
pub(crate) const IF_MMODE_CODE: i16 = 7;
pub(crate) const IF_INNER_CODE: i16 = 8;
pub(crate) const IF_VOID_CODE: i16 = 9;
pub(crate) const IF_HBOX_CODE: i16 = 10;
pub(crate) const IF_VBOX_CODE: i16 = 11;
pub(crate) const IFX_CODE: i16 = 12;
pub(crate) const IF_EOF_CODE: i16 = 13;
pub(crate) const IF_TRUE_CODE: i16 = 14;
pub(crate) const IF_FALSE_CODE: i16 = 15;
pub(crate) const IF_CASE_CODE: i16 = 16;
pub(crate) const IF_DEF_CODE: i16 = 17;
pub(crate) const IF_CS_CODE: i16 = 18;
pub(crate) const IF_FONT_CHAR_CODE: i16 = 19;
pub(crate) const IF_IN_CSNAME_CODE: i16 = 20;
pub(crate) const IF_PRIMITIVE_CODE: i16 = 21;

/* args to Cmd::FiOrElse */
pub(crate) const FI_CODE: u8 = 2;
pub(crate) const ELSE_CODE: u8 = 3;
pub(crate) const OR_CODE: u8 = 4;

/* special args for Cmd::TabMark, Cmd::CarRet */
pub(crate) const SPAN_CODE: placeholdertype = BIGGEST_USV as i32 + 2;
pub(crate) const CR_CODE: placeholdertype = BIGGEST_USV as i32 + 3;
pub(crate) const CR_CR_CODE: placeholdertype = BIGGEST_USV as i32 + 4;

/* Cmd::HSkip, Cmd::VSkip, Cmd::MSkip */
pub(crate) const FIL_CODE: placeholdertype = 0;
pub(crate) const FILL_CODE: placeholdertype = 1;
pub(crate) const SS_CODE: placeholdertype = 2;
pub(crate) const FIL_NEG_CODE: placeholdertype = 3;
pub(crate) const SKIP_CODE: placeholdertype = 4;
pub(crate) const MSKIP_CODE: placeholdertype = 5;

/* Cmd::MakeBox, Cmd::UnHBox, Cmd::UnVBox */
pub(crate) const BOX_CODE: placeholdertype = 0;
pub(crate) const COPY_CODE: placeholdertype = 1;
pub(crate) const LAST_BOX_CODE: placeholdertype = 2;
pub(crate) const VSPLIT_CODE: placeholdertype = 3;
pub(crate) const VTOP_CODE: placeholdertype = 4;

/* Cmd::LeaderShip */
// NORMAL
// GluePar
pub(crate) const COND_MATH_GLUE: u16 = 98;
pub(crate) const MU_GLUE: u16 = 99;
pub(crate) const A_LEADERS: u16 = 100;
pub(crate) const C_LEADERS: u16 = 101;
pub(crate) const X_LEADERS: u16 = 102;

/* Cmd::LimitSwitch */
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, enumn::N)]
pub(crate) enum Limit {
    Normal = 0,
    Limits = 1,
    NoLimits = 2,
}

impl From<u16> for Limit {
    fn from(n: u16) -> Self {
        Self::n(n).unwrap()
    }
}

/* Cmd::MathStyle */
pub(crate) const DISPLAY_STYLE: placeholdertype = 0;
pub(crate) const TEXT_STYLE: placeholdertype = 2;
pub(crate) const SCRIPT_STYLE: placeholdertype = 4;
pub(crate) const SCRIPT_SCRIPT_STYLE: placeholdertype = 6;

/* Cmd::Above */
pub(crate) const ABOVE_CODE: placeholdertype = 0;
pub(crate) const OVER_CODE: placeholdertype = 1;
pub(crate) const ATOP_CODE: placeholdertype = 2;
pub(crate) const DELIMITED_CODE: placeholdertype = 3;

/* Cmd::ShorthandDef */
pub(crate) const CHAR_DEF_CODE: placeholdertype = 0;
pub(crate) const MATH_CHAR_DEF_CODE: placeholdertype = 1;
pub(crate) const COUNT_DEF_CODE: placeholdertype = 2;
pub(crate) const DIMEN_DEF_CODE: placeholdertype = 3;
pub(crate) const SKIP_DEF_CODE: placeholdertype = 4;
pub(crate) const MU_SKIP_DEF_CODE: placeholdertype = 5;
pub(crate) const TOKS_DEF_CODE: placeholdertype = 6;
pub(crate) const CHAR_SUB_DEF_CODE: placeholdertype = 7;
pub(crate) const XETEX_MATH_CHAR_NUM_DEF_CODE: placeholdertype = 8;
pub(crate) const XETEX_MATH_CHAR_DEF_CODE: placeholdertype = 9;

/* XRAY */
pub(crate) const SHOW_CODE: placeholdertype = 0;
pub(crate) const SHOW_BOX_CODE: placeholdertype = 1;
pub(crate) const SHOW_THE_CODE: placeholdertype = 2;
pub(crate) const SHOW_LISTS: placeholdertype = 3;
pub(crate) const SHOW_GROUPS: placeholdertype = 4;
pub(crate) const SHOW_TOKENS: placeholdertype = 5;
pub(crate) const SHOW_IFS: placeholdertype = 6;

/* Cmd::Extension */
pub(crate) const IMMEDIATE_CODE: u16 = 4;
pub(crate) const SET_LANGUAGE_CODE: u16 = 5;
pub(crate) const PDFTEX_FIRST_EXTENSION_CODE: u16 = 6;
pub(crate) const PIC_FILE_CODE: u16 = 41;
pub(crate) const PDF_FILE_CODE: u16 = 42;
pub(crate) const GLYPH_CODE: u16 = 43;
pub(crate) const XETEX_INPUT_ENCODING_EXTENSION_CODE: u16 = 44;
pub(crate) const XETEX_DEFAULT_ENCODING_EXTENSION_CODE: u16 = 45;
pub(crate) const XETEX_LINEBREAK_LOCALE_EXTENSION_CODE: u16 = 46;

/* Cmd::VAlign overloads */
pub(crate) const BEGIN_L_CODE: placeholdertype = 6;
pub(crate) const END_L_CODE: placeholdertype = 7;
pub(crate) const BEGIN_R_CODE: placeholdertype = 10;
pub(crate) const END_R_CODE: placeholdertype = 11;

/* begin_token_list() types */

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, enumn::N)]
pub(crate) enum Btl {
    Parameter = 0,
    UTemplate = 1,
    VTemplate = 2,
    BackedUp = 3,
    BackedUpChar = 4,
    Inserted = 5,
    Macro = 6,
    OutputText = 7,
    EveryParText = 8,
    EveryMathText = 9,
    EveryDisplayText = 10,
    EveryHBoxText = 11,
    EveryVBoxText = 12,
    EveryJobText = 13,
    EveryCRText = 14,
    MarkText = 15,
    EveryEOFText = 16,
    InterCharText = 17,
    WriteText = 18,
    TectonicCodaText = 19,
}

impl Default for Btl {
    fn default() -> Self {
        Btl::Parameter
    }
}

impl From<u16> for Btl {
    fn from(n: u16) -> Self {
        Self::n(n).unwrap()
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, enumn::N)]
pub(crate) enum GlueSign {
    Normal = 0,
    Stretching = 1,
    Shrinking = 2,
}

impl From<u16> for GlueSign {
    fn from(n: u16) -> Self {
        Self::n(n).unwrap()
    }
}

/* input state */
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, enumn::N)]
pub(crate) enum InputState {
    TokenList = 0,
    MidLine = 1,
    SkipBlanks = 17,
    NewLine = 33,
}

impl Default for InputState {
    fn default() -> Self {
        Self::TokenList
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, enumn::N)]
pub(crate) enum ListMode {
    NoMode = 0,
    VMode = 1,
    HMode = 104,
    MMode = 207,
}

impl Default for ListMode {
    fn default() -> Self {
        Self::NoMode
    }
}

impl From<u8> for ListMode {
    fn from(n: u8) -> Self {
        Self::n(n).unwrap()
    }
}

/* DVI format codes */
pub(crate) const XDV_ID_BYTE: u8 = 7;
pub(crate) const SPX_ID_BYTE: u8 = 100;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, enumn::N)]
pub(crate) enum SaveCmd {
    RestoreOldValue = 0,
    RestoreZero = 1,
    InsertToken = 2,
    LevelBoundary = 3,
    RestoreSA = 4,
}
impl From<u16> for SaveCmd {
    fn from(n: u16) -> Self {
        Self::n(n as u8).expect(&format!("incorrect save command = {}", n))
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, enumn::N)]
pub(crate) enum BreakType {
    Unhyphenated = 0,
    Hyphenated = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, enumn::N)]
pub(crate) enum ValLevel {
    Int = 0,
    Dimen = 1,
    Glue = 2,
    Mu = 3,
    Ident = 4,
    Tok = 5,
    InterChar = 6,
    Mark = 7,
}

impl From<u8> for ValLevel {
    fn from(n: u8) -> Self {
        Self::n(n).expect(&format!("incorrect value level = {}", n))
    }
}

impl ValLevel {
    pub(crate) fn prev(&mut self) {
        // TODO: remove this
        use ValLevel::*;
        *self = match self {
            Int => panic!("Minimal value level"),
            Dimen => Self::Int,
            Glue => Self::Dimen,
            Mu => Self::Glue,
            Ident => Self::Mu,
            Tok => Self::Ident,
            InterChar => Self::Tok,
            Mark => Self::InterChar,
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PageContents {
    Empty = 0,
    InsertsOnly = 1,
    BoxThere = 2,
}

pub(crate) const EMPTY: i32 = 0;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OpenMode {
    Normal = 0,
    JustOpen = 1,
    Closed = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, enumn::N)]
pub(crate) enum PackMode {
    Exactly = 0,
    Additional = 1,
}

impl From<i32> for PackMode {
    fn from(n: i32) -> Self {
        Self::n(n as u8).expect(&format!("incorrect PackMode = {}", n))
    }
}

pub(crate) const DISPLAYOPERATORMINHEIGHT: placeholdertype = 3;
pub(crate) const ACCENTBASEHEIGHT: placeholdertype = 6;
pub(crate) const SUBSCRIPTTOPMAX: placeholdertype = 9;
pub(crate) const SUPERSCRIPTBOTTOMMIN: placeholdertype = 13;
pub(crate) const SUBSUPERSCRIPTGAPMIN: placeholdertype = 15;
pub(crate) const SUPERSCRIPTBOTTOMMAXWITHSUBSCRIPT: placeholdertype = 16;
pub(crate) const STACKGAPMIN: placeholdertype = 26;
pub(crate) const STACKDISPLAYSTYLEGAPMIN: placeholdertype = 27;
pub(crate) const FRACTIONNUMERATORGAPMIN: placeholdertype = 36;
pub(crate) const FRACTIONNUMDISPLAYSTYLEGAPMIN: placeholdertype = 37;
pub(crate) const FRACTIONDENOMINATORGAPMIN: placeholdertype = 39;
pub(crate) const FRACTIONDENOMDISPLAYSTYLEGAPMIN: placeholdertype = 40;
pub(crate) const RADICALVERTICALGAP: placeholdertype = 49;
pub(crate) const RADICALDISPLAYSTYLEVERTICALGAP: placeholdertype = 50;
pub(crate) const RADICALRULETHICKNESS: placeholdertype = 51;

pub(crate) const SET1: u8 = 128;
pub(crate) const SET_RULE: u8 = 132;
pub(crate) const PUT_RULE: u8 = 137;
pub(crate) const BOP: u8 = 139;
pub(crate) const EOP: u8 = 140;
pub(crate) const PUSH: u8 = 141;
pub(crate) const POP: u8 = 142;
pub(crate) const RIGHT1: u8 = 143;
pub(crate) const DOWN1: u8 = 157;
pub(crate) const FNT1: u8 = 235;
pub(crate) const XXX1: u8 = 239;
pub(crate) const XXX4: u8 = 242;
pub(crate) const FNT_DEF1: u8 = 243;
pub(crate) const PRE: u8 = 247;
pub(crate) const POST: u8 = 248;
pub(crate) const POST_POST: u8 = 249;
pub(crate) const DEFINE_NATIVE_FONT: u8 = 252;
pub(crate) const SET_GLYPHS: u8 = 253;
pub(crate) const SET_TEXT_AND_GLYPHS: u8 = 254;

pub(crate) const XETEX_VERSION: placeholdertype = 0;
pub(crate) const FONT_BASE: usize = 0;
pub(crate) const NON_ADDRESS: placeholdertype = 0;
pub(crate) const UNDEFINED_PRIMITIVE: placeholdertype = 0;
pub(crate) const FIXED_ACC: placeholdertype = 1;
pub(crate) const MATH_CHAR: placeholdertype = 1;
pub(crate) const PRIM_BASE: usize = 1;
pub(crate) const SLANT_CODE: placeholdertype = 1;
pub(crate) const BOTTOM_ACC: u16 = 2;
pub(crate) const ETEX_VERSION: placeholdertype = 2;
pub(crate) const SPACE_CODE: placeholdertype = 2;
pub(crate) const SUB_BOX: placeholdertype = 2;
pub(crate) const SUB_MLIST: placeholdertype = 3;
pub(crate) const MATH_TEXT_CHAR: placeholdertype = 4;
pub(crate) const SPACE_SHRINK_CODE: placeholdertype = 4;
pub(crate) const X_HEIGHT_CODE: placeholdertype = 5;
pub(crate) const QUAD_CODE: placeholdertype = 6;
pub(crate) const EXTRA_SPACE_CODE: placeholdertype = 7;
pub(crate) const VAR_FAM_CLASS: placeholdertype = 7;
pub(crate) const NATIVE_GLYPH_INFO_SIZE: placeholdertype = 10;
pub(crate) const CARRIAGE_RETURN: placeholdertype = 13;
pub(crate) const TOTAL_MATHEX_PARAMS: placeholdertype = 13;
pub(crate) const HI_MEM_STAT_USAGE: placeholdertype = 15;
pub(crate) const MAX_CHAR_CODE: placeholdertype = 15;
pub(crate) const TOTAL_MATHSY_PARAMS: placeholdertype = 22;
pub(crate) const UNLESS_CODE: placeholdertype = 32;
pub(crate) const XETEX_DIM: placeholdertype = 47;
pub(crate) const ETEX_GLUE: placeholdertype = 57;
pub(crate) const ETEX_MU: placeholdertype = 58;

pub(crate) const DIMEN_VAL_LIMIT: u16 = 128;
pub(crate) const BIGGEST_LANG: placeholdertype = 255;
pub(crate) const MU_VAL_LIMIT: u16 = 256;
pub(crate) const TOO_BIG_LANG: placeholdertype = 256;
pub(crate) const BOX_VAL_LIMIT: u16 = 320;
pub(crate) const TOK_VAL_LIMIT: u16 = 384;
pub(crate) const PRIM_PRIME: usize = 431;
pub(crate) const PRIM_SIZE: usize = 500;
pub(crate) const HYPH_PRIME: placeholdertype = 607;
pub(crate) const HYPHENATABLE_LENGTH_LIMIT: usize = 4095;
pub(crate) const CHAR_CLASS_LIMIT: placeholdertype = 4096;
pub(crate) const EJECT_PENALTY: placeholdertype = -10000;
pub(crate) const INF_BAD: placeholdertype = 10000;
pub(crate) const INF_PENALTY: placeholdertype = 10000;
pub(crate) const DEFAULT_RULE: placeholdertype = 26214;
pub(crate) const TOO_BIG_CHAR: placeholdertype = 65536;
pub(crate) const NO_EXPAND_FLAG: placeholdertype = BIGGEST_USV as i32 + 2;

pub(crate) const ACTIVE_MATH_CHAR: placeholdertype = 0x1FFFFF;

/* Token codes */

/// 1 << 21
pub(crate) const MAX_CHAR_VAL: placeholdertype = 0x200000;
pub(crate) const CS_TOKEN_FLAG: placeholdertype = 0x1FFFFFF;
/// LEFT_BRACE << 21
pub(crate) const LEFT_BRACE_TOKEN: placeholdertype = 0x200000;
/// (LEFT_BRACE + 1) << 21
pub(crate) const LEFT_BRACE_LIMIT: placeholdertype = 0x400000;
/// RIGHT_BRACE << 21
pub(crate) const RIGHT_BRACE_TOKEN: placeholdertype = 0x400000;
/// (RIGHT_BRACE + 1) << 21
pub(crate) const RIGHT_BRACE_LIMIT: placeholdertype = 0x600000;
/// MATH_SHIFT << 21
pub(crate) const MATH_SHIFT_TOKEN: placeholdertype = 0x600000;
/// TAB_MARK << 21
pub(crate) const TAB_TOKEN: placeholdertype = 0x800000;
/// OUT_PARAM << 21
pub(crate) const OUT_PARAM_TOKEN: placeholdertype = 0xA00000;
/// SPACER << 21 + ord(' ')
pub(crate) const SPACE_TOKEN: placeholdertype = 0x1400020;
/// LETTER << 21
pub(crate) const LETTER_TOKEN: placeholdertype = 0x1600000;
/// OTHER_CHAR << 21
pub(crate) const OTHER_TOKEN: placeholdertype = 0x1800000;
/// MATCH << 21
pub(crate) const MATCH_TOKEN: placeholdertype = 0x1A00000;
/// END_MATCH << 21
pub(crate) const END_MATCH_TOKEN: placeholdertype = 0x1C00000;
pub(crate) const PROTECTED_TOKEN: placeholdertype = END_MATCH_TOKEN + 1;

pub(crate) const A_TOKEN: placeholdertype = LETTER_TOKEN + 'A' as placeholdertype;
pub(crate) const OTHER_A_TOKEN: placeholdertype = OTHER_TOKEN + 'A' as placeholdertype;
pub(crate) const HEX_TOKEN: placeholdertype = OTHER_TOKEN + '"' as placeholdertype;
pub(crate) const OCTAL_TOKEN: placeholdertype = OTHER_TOKEN + '\'' as placeholdertype;
pub(crate) const CONTINENTAL_POINT_TOKEN: placeholdertype = OTHER_TOKEN + ',' as placeholdertype;
pub(crate) const POINT_TOKEN: placeholdertype = OTHER_TOKEN + '.' as placeholdertype;
pub(crate) const ZERO_TOKEN: placeholdertype = OTHER_TOKEN + '0' as placeholdertype;
pub(crate) const ALPHA_TOKEN: placeholdertype = OTHER_TOKEN + '`' as placeholdertype;

pub(crate) const BOX_FLAG: placeholdertype = 0x40000000;
pub(crate) const GLOBAL_BOX_FLAG: placeholdertype = 0x40008000;
pub(crate) const SHIP_OUT_FLAG: placeholdertype = 0x40010000;
pub(crate) const LEADER_FLAG: placeholdertype = 0x40010001;

pub(crate) const LP_CODE_BASE: placeholdertype = 2;
pub(crate) const RP_CODE_BASE: placeholdertype = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Side {
    Left = 0,
    Right = 1,
}

/* modes to do_marks() */
#[repr(i16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MarkMode {
    VSplitInit = 0,
    FireUpInit = 1,
    FireUpDone = 2,
    DestroyMarks = 3,
}

pub(crate) const MARKS_CODE: placeholdertype = 5;

pub(crate) const IGNORE_DEPTH: placeholdertype = -65536000;

pub(crate) const MIDDLE_NOAD: u16 = 1;

/* movement() */

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MoveSeen {
    None = 0,
    Y = 6,
    Z = 12,
}
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, enumn::N)]
pub(crate) enum MoveDir {
    YHere = 1,
    ZHere = 2,
    YZOk = 3,
    YOk = 4,
    ZOk = 5,
    DFixed = 6,
}
impl From<i32> for MoveDir {
    fn from(n: i32) -> Self {
        Self::n(n).expect(&format!("incorrect move direction = {}", n))
    }
}

/* Increase this whenever the engine internals change such that the contents
 * of the "format" files must be regenerated -- this includes changes to the
 * string pool. KEEP SYNCHRONIZED WITH src/lib.rs!!! */

pub(crate) const FORMAT_SERIAL: placeholdertype = 28;

/// Unicode file reading modes
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, enumn::N)]
pub(crate) enum UnicodeMode {
    /// default: will become one of 1..3 at file open time, after sniffing
    Auto = 0,
    Utf8 = 1,
    Utf16be = 2,
    Utf16le = 3,
    Raw = 4,
    ICUMapping = 5,
}
impl From<i32> for UnicodeMode {
    fn from(n: i32) -> Self {
        Self::n(n).expect(&format!("incorrect unicode encoding mode = {}", n))
    }
}
