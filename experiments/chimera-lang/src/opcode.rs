use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpCode {
    Push,
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Swap,
    Drop,
    Print,
    Jump,
    Brz,
    Photosynthesize,
    Consume,
    GRead,
    GWrite,
    Radiate,
    Siphon,
    Genome,
    Virus,
    Transcribe,
    JumpS,
    BrzS,
    SLen,
    HelixLen,
    GeneLen,

    // Cortex Features
    #[cfg(feature = "cortex")]
    Link,
    #[cfg(feature = "cortex")]
    Sever,
    #[cfg(feature = "cortex")]
    Spark,
    #[cfg(feature = "cortex")]
    Sense,
    #[cfg(feature = "cortex")]
    Gate,

    // Nova Features
    #[cfg(feature = "nova")]
    Sporulate,
    #[cfg(feature = "nova")]
    Germinate,
    #[cfg(feature = "nova")]
    Incubate,
    #[cfg(feature = "nova")]
    Methylate,
    #[cfg(feature = "nova")]
    Demethylate,
    #[cfg(feature = "nova")]
    Telomerase,
    #[cfg(feature = "nova")]
    TLen,
    #[cfg(feature = "nova")]
    Recombine,
    #[cfg(feature = "nova")]
    SIndex,
    #[cfg(feature = "nova")]
    CrisprScan,
    #[cfg(feature = "nova")]
    Cas9Cut,
    #[cfg(feature = "nova")]
    Ligase,
    #[cfg(feature = "nova")]
    Mitosis,
    #[cfg(feature = "nova")]
    Apoptosis,
    #[cfg(feature = "nova")]
    Integrase,
    #[cfg(feature = "nova")]
    Excision,
    #[cfg(feature = "nova")]
    Secrete,
    #[cfg(feature = "nova")]
    Detect,
    #[cfg(feature = "nova")]
    Absorb,
    #[cfg(feature = "nova")]
    Migrate,
    #[cfg(feature = "nova")]
    Detox,
    #[cfg(feature = "nova")]
    WRead,
    #[cfg(feature = "nova")]
    Call,
    #[cfg(feature = "nova")]
    Ret,
    #[cfg(feature = "nova")]
    Bind,
    #[cfg(feature = "nova")]
    Unbind,
    #[cfg(feature = "nova")]
    Entangle,
    #[cfg(feature = "nova")]
    Decohere,
    #[cfg(feature = "nova")]
    Conjugate,
    #[cfg(feature = "nova")]
    Gravitate,
    #[cfg(feature = "nova")]
    Inject,

    Unknown(String),
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "push" => Ok(OpCode::Push),
            "add" => Ok(OpCode::Add),
            "sub" => Ok(OpCode::Sub),
            "mul" => Ok(OpCode::Mul),
            "div" => Ok(OpCode::Div),
            "dup" => Ok(OpCode::Dup),
            "swap" => Ok(OpCode::Swap),
            "drop" => Ok(OpCode::Drop),
            "print" => Ok(OpCode::Print),
            "jump" => Ok(OpCode::Jump),
            "brz" => Ok(OpCode::Brz),
            "photosynthesize" => Ok(OpCode::Photosynthesize),
            "consume" => Ok(OpCode::Consume),
            "g_read" => Ok(OpCode::GRead),
            "g_write" => Ok(OpCode::GWrite),
            "radiate" => Ok(OpCode::Radiate),
            "siphon" => Ok(OpCode::Siphon),
            "genome" => Ok(OpCode::Genome),
            "virus" => Ok(OpCode::Virus),
            "transcribe" => Ok(OpCode::Transcribe),
            "jump_s" => Ok(OpCode::JumpS),
            "brz_s" => Ok(OpCode::BrzS),
            "s_len" => Ok(OpCode::SLen),
            "helix_len" => Ok(OpCode::HelixLen),
            "gene_len" => Ok(OpCode::GeneLen),

            #[cfg(feature = "cortex")]
            "link" => Ok(OpCode::Link),
            #[cfg(feature = "cortex")]
            "sever" => Ok(OpCode::Sever),
            #[cfg(feature = "cortex")]
            "spark" => Ok(OpCode::Spark),
            #[cfg(feature = "cortex")]
            "sense" => Ok(OpCode::Sense),
            #[cfg(feature = "cortex")]
            "gate" => Ok(OpCode::Gate),

            #[cfg(feature = "nova")]
            "sporulate" => Ok(OpCode::Sporulate),
            #[cfg(feature = "nova")]
            "germinate" => Ok(OpCode::Germinate),
            #[cfg(feature = "nova")]
            "incubate" => Ok(OpCode::Incubate),
            #[cfg(feature = "nova")]
            "methylate" => Ok(OpCode::Methylate),
            #[cfg(feature = "nova")]
            "demethylate" => Ok(OpCode::Demethylate),
            #[cfg(feature = "nova")]
            "telomerase" => Ok(OpCode::Telomerase),
            #[cfg(feature = "nova")]
            "t_len" => Ok(OpCode::TLen),
            #[cfg(feature = "nova")]
            "recombine" => Ok(OpCode::Recombine),
            #[cfg(feature = "nova")]
            "s_index" => Ok(OpCode::SIndex),
            #[cfg(feature = "nova")]
            "crispr_scan" => Ok(OpCode::CrisprScan),
            #[cfg(feature = "nova")]
            "cas9_cut" => Ok(OpCode::Cas9Cut),
            #[cfg(feature = "nova")]
            "ligase" => Ok(OpCode::Ligase),
            #[cfg(feature = "nova")]
            "mitosis" => Ok(OpCode::Mitosis),
            #[cfg(feature = "nova")]
            "apoptosis" => Ok(OpCode::Apoptosis),
            #[cfg(feature = "nova")]
            "integrase" => Ok(OpCode::Integrase),
            #[cfg(feature = "nova")]
            "excision" => Ok(OpCode::Excision),
            #[cfg(feature = "nova")]
            "secrete" => Ok(OpCode::Secrete),
            #[cfg(feature = "nova")]
            "detect" => Ok(OpCode::Detect),
            #[cfg(feature = "nova")]
            "absorb" => Ok(OpCode::Absorb),
            #[cfg(feature = "nova")]
            "migrate" => Ok(OpCode::Migrate),
            #[cfg(feature = "nova")]
            "detox" => Ok(OpCode::Detox),
            #[cfg(feature = "nova")]
            "w_read" => Ok(OpCode::WRead),
            #[cfg(feature = "nova")]
            "call" => Ok(OpCode::Call),
            #[cfg(feature = "nova")]
            "ret" => Ok(OpCode::Ret),
            #[cfg(feature = "nova")]
            "bind" => Ok(OpCode::Bind),
            #[cfg(feature = "nova")]
            "unbind" => Ok(OpCode::Unbind),
            #[cfg(feature = "nova")]
            "entangle" => Ok(OpCode::Entangle),
            #[cfg(feature = "nova")]
            "decohere" => Ok(OpCode::Decohere),
            #[cfg(feature = "nova")]
            "conjugate" => Ok(OpCode::Conjugate),
            #[cfg(feature = "nova")]
            "gravitate" => Ok(OpCode::Gravitate),
            #[cfg(feature = "nova")]
            "inject" => Ok(OpCode::Inject),

            _ => Ok(OpCode::Unknown(s.to_string())),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Push => write!(f, "push"),
            OpCode::Add => write!(f, "add"),
            OpCode::Sub => write!(f, "sub"),
            OpCode::Mul => write!(f, "mul"),
            OpCode::Div => write!(f, "div"),
            OpCode::Dup => write!(f, "dup"),
            OpCode::Swap => write!(f, "swap"),
            OpCode::Drop => write!(f, "drop"),
            OpCode::Print => write!(f, "print"),
            OpCode::Jump => write!(f, "jump"),
            OpCode::Brz => write!(f, "brz"),
            OpCode::Photosynthesize => write!(f, "photosynthesize"),
            OpCode::Consume => write!(f, "consume"),
            OpCode::GRead => write!(f, "g_read"),
            OpCode::GWrite => write!(f, "g_write"),
            OpCode::Radiate => write!(f, "radiate"),
            OpCode::Siphon => write!(f, "siphon"),
            OpCode::Genome => write!(f, "genome"),
            OpCode::Virus => write!(f, "virus"),
            OpCode::Transcribe => write!(f, "transcribe"),
            OpCode::JumpS => write!(f, "jump_s"),
            OpCode::BrzS => write!(f, "brz_s"),
            OpCode::SLen => write!(f, "s_len"),
            OpCode::HelixLen => write!(f, "helix_len"),
            OpCode::GeneLen => write!(f, "gene_len"),

            #[cfg(feature = "cortex")]
            OpCode::Link => write!(f, "link"),
            #[cfg(feature = "cortex")]
            OpCode::Sever => write!(f, "sever"),
            #[cfg(feature = "cortex")]
            OpCode::Spark => write!(f, "spark"),
            #[cfg(feature = "cortex")]
            OpCode::Sense => write!(f, "sense"),
            #[cfg(feature = "cortex")]
            OpCode::Gate => write!(f, "gate"),

            #[cfg(feature = "nova")]
            OpCode::Sporulate => write!(f, "sporulate"),
            #[cfg(feature = "nova")]
            OpCode::Germinate => write!(f, "germinate"),
            #[cfg(feature = "nova")]
            OpCode::Incubate => write!(f, "incubate"),
            #[cfg(feature = "nova")]
            OpCode::Methylate => write!(f, "methylate"),
            #[cfg(feature = "nova")]
            OpCode::Demethylate => write!(f, "demethylate"),
            #[cfg(feature = "nova")]
            OpCode::Telomerase => write!(f, "telomerase"),
            #[cfg(feature = "nova")]
            OpCode::TLen => write!(f, "t_len"),
            #[cfg(feature = "nova")]
            OpCode::Recombine => write!(f, "recombine"),
            #[cfg(feature = "nova")]
            OpCode::SIndex => write!(f, "s_index"),
            #[cfg(feature = "nova")]
            OpCode::CrisprScan => write!(f, "crispr_scan"),
            #[cfg(feature = "nova")]
            OpCode::Cas9Cut => write!(f, "cas9_cut"),
            #[cfg(feature = "nova")]
            OpCode::Ligase => write!(f, "ligase"),
            #[cfg(feature = "nova")]
            OpCode::Mitosis => write!(f, "mitosis"),
            #[cfg(feature = "nova")]
            OpCode::Apoptosis => write!(f, "apoptosis"),
            #[cfg(feature = "nova")]
            OpCode::Integrase => write!(f, "integrase"),
            #[cfg(feature = "nova")]
            OpCode::Excision => write!(f, "excision"),
            #[cfg(feature = "nova")]
            OpCode::Secrete => write!(f, "secrete"),
            #[cfg(feature = "nova")]
            OpCode::Detect => write!(f, "detect"),
            #[cfg(feature = "nova")]
            OpCode::Absorb => write!(f, "absorb"),
            #[cfg(feature = "nova")]
            OpCode::Migrate => write!(f, "migrate"),
            #[cfg(feature = "nova")]
            OpCode::Detox => write!(f, "detox"),
            #[cfg(feature = "nova")]
            OpCode::WRead => write!(f, "w_read"),
            #[cfg(feature = "nova")]
            OpCode::Call => write!(f, "call"),
            #[cfg(feature = "nova")]
            OpCode::Ret => write!(f, "ret"),
            #[cfg(feature = "nova")]
            OpCode::Bind => write!(f, "bind"),
            #[cfg(feature = "nova")]
            OpCode::Unbind => write!(f, "unbind"),
            #[cfg(feature = "nova")]
            OpCode::Entangle => write!(f, "entangle"),
            #[cfg(feature = "nova")]
            OpCode::Decohere => write!(f, "decohere"),
            #[cfg(feature = "nova")]
            OpCode::Conjugate => write!(f, "conjugate"),
            #[cfg(feature = "nova")]
            OpCode::Gravitate => write!(f, "gravitate"),
            #[cfg(feature = "nova")]
            OpCode::Inject => write!(f, "inject"),

            OpCode::Unknown(s) => write!(f, "{}", s),
        }
    }
}
