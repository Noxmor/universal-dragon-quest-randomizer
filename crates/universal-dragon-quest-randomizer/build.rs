use std::{env, fs, path::PathBuf};

use quote::quote;
use serde::Deserialize;

const DATABASE_FILE: &str = "data/rom-database.toml";
const GENERATED_FILE: &str = "rom_database.rs";

#[derive(Debug, Deserialize)]
struct Database {
    #[serde(default)]
    rom: Vec<RomEntry>,
}

#[derive(Debug, Deserialize)]
struct RomEntry {
    sha256: String,
    id: String,
    platform: String,
    format: String,
    region: String,
    revision: u16,
}

fn main() {
    println!("cargo:rerun-if-changed={DATABASE_FILE}");

    let input = fs::read_to_string(DATABASE_FILE)
        .unwrap_or_else(|error| panic!("failed to read `{DATABASE_FILE}`: {error}"));

    let mut database: Database = toml::from_str(&input)
        .unwrap_or_else(|error| panic!("failed to parse `{DATABASE_FILE}`: {error}"));

    validate_database(&database);

    database
        .rom
        .sort_unstable_by_key(|entry| parse_hash(&entry.sha256));

    let generated = generate_database(&database);

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set"));

    let output_path = out_dir.join(GENERATED_FILE);

    fs::write(&output_path, generated.to_string())
        .unwrap_or_else(|error| panic!("failed to write `{output_path:?}`: {error}"));
}

fn validate_database(database: &Database) {
    for (index, entry) in database.rom.iter().enumerate() {
        let hash = parse_hash(&entry.sha256);

        if entry.id.is_empty() {
            panic!("[[rom]] entry {index}: `id` must be specified");
        }

        if entry.format.is_empty() {
            panic!("[[rom]] entry {index}: `format` must be specified");
        }

        if entry.region.is_empty() {
            panic!("[[rom]] entry {index}: `region` must be specified");
        }

        let _ = hash;
    }

    for pair in database.rom.windows(2) {
        let first = parse_hash(&pair[0].sha256);
        let second = parse_hash(&pair[1].sha256);

        if first == second {
            panic!("duplicate SHA-256 hash: `{}`", pair[0].sha256);
        }
    }
}

fn parse_hash(value: &str) -> [u8; 32] {
    if value.len() != 64 {
        panic!("SHA-256 hash must contain exactly 64 hexadecimal characters: `{value}`");
    }

    let mut hash = [0u8; 32];

    for (index, byte) in hash.iter_mut().enumerate() {
        let start = index * 2;
        let end = start + 2;

        *byte = u8::from_str_radix(&value[start..end], 16)
            .unwrap_or_else(|_| panic!("invalid SHA-256 hash: `{value}`"));
    }

    hash
}

fn generate_database(database: &Database) -> proc_macro2::TokenStream {
    let entries = database.rom.iter().map(|entry| {
        let hash = parse_hash(&entry.sha256);

        let id = ident(&entry.id);
        let format = ident(&entry.format);
        let region = ident(&entry.region);
        let platform = ident(&entry.platform);
        let revision = entry.revision;

        quote! {
            RomEntry {
                hash: RomHash::from_raw([
                    #(#hash),*
                ]),
                definition: RomDefinition {
                    id: RomId::#id,
                    platform: Platform::#platform,
                    format: RomFormat::#format,
                    region: Region::#region,
                    revision: Revision::new(#revision),
                },
            }
        }
    });

    quote! {
        pub static ROM_DATABASE: RomDatabase = RomDatabase {
            entries: &[
                #(#entries),*
            ],
        };
    }
}

fn ident(value: &str) -> proc_macro2::Ident {
    proc_macro2::Ident::new(value, proc_macro2::Span::call_site())
}
