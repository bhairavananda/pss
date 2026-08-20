use std::env;
use std::io::{self, BufRead};

use pss::parser::{slp1 as slp1_parser, iast as iast_parser, baraha as baraha_parser};
use pss::emitter::{slp1 as slp1_emitter, iast as iast_emitter, devanagari as dev_emitter};
use pss::varna::Varna;
use pss::encode;

const USAGE: &str = "\
pss — Paniniya Shiksha Serialization

Usage:
  pss <from> <to> <text>       Convert text between formats
  pss <from> <to>              Read from stdin
  pss html <from> <text>       Render as HTML (opens in browser)
  pss bytes <from> <text>      Show PSS byte encoding (hex)
  pss inspect <from> <text>    Show phonological features

Formats: iast, slp1, baraha, devanagari (dev)

Examples:
  pss iast slp1 'kṛṣṇa'
  pss slp1 iast 'kfzRa'
  pss baraha dev 'kRuShNa'
  pss html baraha 'iqShE tvOq,-rjE tvA#'
  pss bytes iast 'śiva'
  pss inspect baraha 'dharma'
  echo 'agnimīḷe' | pss iast slp1";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
        eprintln!("{}", USAGE);
        std::process::exit(if args.is_empty() { 1 } else { 0 });
    }

    match args[0].as_str() {
        "bytes" => {
            let (fmt, text) = subcommand_args(&args);
            cmd_bytes(&fmt, &text);
        }
        "inspect" => {
            let (fmt, text) = subcommand_args(&args);
            cmd_inspect(&fmt, &text);
        }
        "html" => {
            let (fmt, text) = subcommand_args(&args);
            cmd_html(&fmt, &text);
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
        "baraha" => baraha_parser::parse(text),
        _ => {
            eprintln!("unknown format: '{}'. supported: iast, slp1, baraha", format);
            std::process::exit(1);
        }
    }
}

fn emit(format: &str, varnas: &[Varna]) -> String {
    match format {
        "iast" => iast_emitter::emit(varnas),
        "slp1" => slp1_emitter::emit(varnas),
        "dev" | "devanagari" => dev_emitter::emit(varnas),
        // Baraha output not yet implemented — emit as IAST
        "baraha" => iast_emitter::emit(varnas),
        _ => {
            eprintln!("unknown format: '{}'. supported: iast, slp1, baraha, dev", format);
            std::process::exit(1);
        }
    }
}

fn subcommand_args(args: &[String]) -> (String, String) {
    if args.len() < 2 {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    }
    let fmt = args[1].clone();
    let text = if args.len() >= 3 {
        args[2..].join(" ")
    } else {
        read_stdin()
    };
    (fmt, text)
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
            Varna::Passthrough(c) => {
                if !c.is_whitespace() {
                    println!("  punct    '{}'", c);
                }
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

fn cmd_html(from: &str, text: &str) {
    let varnas = parse(from, text);
    let dev = dev_emitter::emit(&varnas);
    let iast = iast_emitter::emit(&varnas);

    let html = format!(r#"<!DOCTYPE html>
<html lang="sa">
<head>
<meta charset="UTF-8">
<title>PSS — Vedic Text</title>
<style>
  @import url('https://fonts.googleapis.com/css2?family=Noto+Sans+Devanagari:wght@400;700&display=swap');
  body {{
    max-width: 800px;
    margin: 40px auto;
    padding: 0 20px;
    background: #1a1a2e;
    color: #e0e0e0;
    font-family: system-ui, sans-serif;
  }}
  h1 {{
    color: #c4a35a;
    font-size: 1.2em;
    border-bottom: 1px solid #333;
    padding-bottom: 8px;
  }}
  .devanagari {{
    font-family: 'Noto Sans Devanagari', 'Siddhanta', 'Sanskrit2003', serif;
    font-size: 2em;
    line-height: 2.2;
    color: #f0e6d3;
    background: #16213e;
    padding: 24px;
    border-radius: 8px;
    border-left: 4px solid #c4a35a;
    margin: 20px 0;
  }}
  .iast {{
    font-family: 'Georgia', serif;
    font-size: 1.1em;
    line-height: 1.8;
    color: #a0a0a0;
    padding: 16px 24px;
    background: #0f3460;
    border-radius: 8px;
    margin: 20px 0;
  }}
  .label {{
    font-size: 0.75em;
    text-transform: uppercase;
    letter-spacing: 2px;
    color: #666;
    margin-bottom: 4px;
  }}
  footer {{
    margin-top: 40px;
    font-size: 0.8em;
    color: #555;
  }}
</style>
</head>
<body>
  <h1>Paniniya Shiksha Serialization</h1>
  <div class="label">Devanagari (with Vedic svaras)</div>
  <div class="devanagari">{dev}</div>
  <div class="label">IAST</div>
  <div class="iast">{iast}</div>
  <footer>Rendered by PSS &mdash; phonological encoding per Paniniya Shiksha</footer>
</body>
</html>"#,
        dev = html_escape(&dev),
        iast = html_escape(&iast),
    );

    // Write to temp file and open in browser
    let path = std::env::temp_dir().join("pss_output.html");
    std::fs::write(&path, &html).expect("failed to write HTML file");
    eprintln!("Written to {}", path.display());

    // Try to open in browser
    #[cfg(target_os = "macos")]
    { let _ = std::process::Command::new("open").arg(&path).spawn(); }
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(&path).spawn(); }

    // Also print to stdout
    println!("{}", html);
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
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
