use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(serde_derive::Deserialize)]
struct Emoji {
    emoji: Option<String>,
    aliases: Vec<String>,
}

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("data_generated.rs");
    let mut file = BufWriter::new(File::create(&dest).unwrap());

    writeln!(&mut file, "/// Compile time generated lookup table for emoji.").unwrap();
    writeln!(&mut file, "///").unwrap();
    writeln!(&mut file, "/// Taken from <https://github.com/github/gemoji>").unwrap();
    write!(&mut file, "pub static EMOJI: phf::Map<&'static str, &'static str> = ").unwrap();
    let source = fs::read(parent.join("gemoji/db/emoji.json"))
        .expect("Can't load ../gemoji/db/emoji.json. Try git submodule update --init");
    let emoji: Vec<Emoji> = serde_json::from_slice(&source).unwrap();

    let mut entries = Vec::new();
    for e in &emoji {
        if let Some(unicode_emoji) = &e.emoji {
            let code = format!("\"{}\"", unicode_emoji);
            for name in &e.aliases {
                entries.push((name.clone(), code.clone()));
            }
        }
    }

    let mut m = phf_codegen::Map::new();
    for (name, code) in &entries {
        m.entry(name, code);
    }
    let m = m.build();
    writeln!(&mut file, "{};", m).unwrap();
}
