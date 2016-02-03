//! Contains `enum` for phrase tags

use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phrase {
    ADJP,
    ADVP,
    CONJP,
    FRAG,
    INTJ,
    LST,
    NAC,
    NP,
    NX,
    PP,
    PRN,
    PRT,
    QP,
    RRC,
    S,
    SBAR,
    SBARQ,
    SINV,
    SQ,
    UCP,
    VP,
    WHADJP,
    WHADVP,
    WHNP,
    WHPP,
    X,
}

impl Phrase {
    pub fn to_str(&self) -> &'static str {
        match self {
            &Phrase::ADJP => "ADJP",
            &Phrase::ADVP => "ADVP",
            &Phrase::CONJP => "CONJP",
            &Phrase::FRAG => "FRAG",
            &Phrase::INTJ => "INTJ",
            &Phrase::LST => "LST",
            &Phrase::NAC => "NAC",
            &Phrase::NP => "NP",
            &Phrase::NX => "NX",
            &Phrase::PP => "PP",
            &Phrase::PRN => "PRN",
            &Phrase::PRT => "PRT",
            &Phrase::QP => "QP",
            &Phrase::RRC => "RRC",
            &Phrase::S => "S",
            &Phrase::SBAR => "SBAR",
            &Phrase::SBARQ => "SBARQ",
            &Phrase::SINV => "SINV",
            &Phrase::SQ => "SQ",
            &Phrase::UCP => "UCP",
            &Phrase::VP => "VP",
            &Phrase::WHADJP => "WHADJP",
            &Phrase::WHADVP => "WHADVP",
            &Phrase::WHNP => "WHNP",
            &Phrase::WHPP => "WHPP",
            &Phrase::X => "X",
        }
    }

    pub fn generate_str_to_phrase_map<'a>() -> HashMap<&'a str, Phrase> {
        let mut map = HashMap::new();
        map.insert("ADJP", Phrase::ADJP);
        map.insert("ADVP", Phrase::ADVP);
        map.insert("CONJP", Phrase::CONJP);
        map.insert("FRAG", Phrase::FRAG);
        map.insert("INTJ", Phrase::INTJ);
        map.insert("LST", Phrase::LST);
        map.insert("NAC", Phrase::NAC);
        map.insert("NP", Phrase::NP);
        map.insert("NX", Phrase::NX);
        map.insert("PP", Phrase::PP);
        map.insert("PRN", Phrase::PRN);
        map.insert("PRT", Phrase::PRT);
        map.insert("QP", Phrase::QP);
        map.insert("RRC", Phrase::RRC);
        map.insert("S", Phrase::S);
        map.insert("SBAR", Phrase::SBAR);
        map.insert("SBARQ", Phrase::SBARQ);
        map.insert("SINV", Phrase::SINV);
        map.insert("SQ", Phrase::SQ);
        map.insert("UCP", Phrase::UCP);
        map.insert("VP", Phrase::VP);
        map.insert("WHADJP", Phrase::WHADJP);
        map.insert("WHADVP", Phrase::WHADVP);
        map.insert("WHNP", Phrase::WHNP);
        map.insert("WHPP", Phrase::WHPP);
        map.insert("X", Phrase::X);
        map
    }
}

