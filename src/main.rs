use std::env;
use std::io::{self, BufRead};

use pss::parser::{slp1 as slp1_parser, iast as iast_parser};
use pss::emitter::{slp1 as slp1_emitter, iast as iast_emitter};
use pss::varna::Varna;
use pss::encode;

const USAGE: &str = "\
pss — Paniniya Shiksha Serialization

Usage:
  pss <from> <to> <text>       Convert text between formats
  pss <from> <to>              Read from stdin
  pss bytes <from> <text>      Show PSS byte encoding (hex)
  pss inspect <from> <text>    Show phonological features

Formats: iast, slp1

Examples:
  pss iast slp1 'kṛṣṇa'
  pss slp1 iast 'kfzRa'
  pss bytes iast 'śiva'
  pss inspect iast 'dharma'
  echo 'agnimīḷe' | pss iast slp1";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
        eprintln!("{}", USAGE);
        std::process::exit(if args.is_empty() { 1 } else { 0 });
    }

    match args[0].as_str() {
        "bytes" => {
            if args.len() < 3 {
                let input = read_stdin();
                cmd_bytes(&args[1], &input);
            } else {
                cmd_bytes(&args[1], &args[2..].join(" "));
            }
        }
        "inspect" => {
            if args.len() < 3 {
                let input = read_stdin();
                cmd_inspect(&args[1], &input);
            } else {
                cmd_inspect(&args[1], &args[2..].join(" "));
            }
        }
        _ => {
            if args.len() < 2 {
                eprintln!("{}", USAGE);
                std::process::exit(1);
            }
            let from = &args[0];
            let to = &args[1];
            let text = if args.len() >= 3 {
                args[2..].join(" ")
            } else {
                read_stdin()
            };
            cmd_convert(from, to, &text);
        }
    }
}

fn parse(format: &str, text: &str) -> Vec<Varna> {
    match format {
        "iast" => iast_parser::parse(text),
        "slp1" => slp1_parser::parse(text),
        _ => {
            eprintln!("unknown format: '{}'. supported: iast, slp1", format);
            std::process::exit(1);
        }
    }
}

fn emit(format: &str, varnas: &[Varna]) -> String {
    match format {
        "iast" => iast_emitter::emit(varnas),
        "slp1" => slp1_emitter::emit(varnas),
        _ => {
            eprintln!("unknown format: '{}'. supported: iast, slp1", format);
            std::process::exit(1);
        }
    }
}

fn cmd_convert(from: &str, to: &str, text: &str) {
    let varnas = parse(from, text);
    println!("{}", emit(to, &varnas));
}

fn cmd_bytes(from: &str, text: &str) {
    let varnas = parse(from, text);
    let bytes = encode::encode(&varnas);
    let hex: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    println!("{}", hex.join(" "));
}

fn cmd_inspect(from: &str, text: &str) {
    let varnas = parse(from, text);
    for v in &varnas {
        match v {
            Varna::Svara { sthana, kala, vivrti, pitch, modifiers } => {
                let mut desc = format!("{:?} {:?}", sthana, kala);
                if let Some(vv) = vivrti {
                    desc.push_str(&format!(" {:?}", vv));
                }
                if let Some(p) = pitch {
                    desc.push_str(&format!(" {:?}", p));
                    // Show musical notes (PS.12)
                    let notes = p.musical_notes().join("+");
                    desc.push_str(&format!(" ({})", notes));
                }
                if modifiers.kampa {
                    desc.push_str(" kampa");
                }
                if modifiers.ranga {
                    desc.push_str(" ranga");
                }
                println!("  svara    {}  [{}]",
                    emit_single(v),
                    desc,
                );
            }
            Varna::Vyanjana { sthana, prayatna, ghosha, prana, nasika } => {
                let mut desc = format!("{:?} {:?} {:?} {:?}", sthana, prayatna, ghosha, prana);
                if *nasika {
                    desc.push_str(" nasika");
                }
                println!("  vyanjana {}  [{}]",
                    emit_single(v),
                    desc,
                );
            }
            Varna::Ayogavaha(typ) => {
                println!("  ayogavaha {}  [{:?}]",
                    emit_single(v),
                    typ,
                );
            }
        }
    }
}

fn emit_single(v: &Varna) -> String {
    let iast = iast_emitter::emit(&[*v]);
    let slp1 = slp1_emitter::emit(&[*v]);
    if iast == slp1 {
        format!("{:<4}", iast)
    } else {
        format!("{:<4} ({})", iast, slp1)
    }
}

fn read_stdin() -> String {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(_) => break,
        }
    }
    lines.join("\n")
}
