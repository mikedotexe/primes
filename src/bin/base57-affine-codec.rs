//! Base57 affine codec experiment CLI.

use clap::{Parser, Subcommand, ValueEnum};
use primes::validation::base57_affine_codec::{
    build_base_invariant_map, bytes_to_text, decode_affine, decode_canonical_payload,
    decode_to_bytes, encode_affine, encode_bytes, format_hex, parse_hex, AffineCodecMode,
    CanonicalInputFormat, CodecAlphabet,
};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Base57 baseline transcoding and affine identifier experiment"
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Encode arbitrary bytes into base57 or base58.
    BaselineEncode {
        #[arg(long)]
        input: String,

        #[arg(long, value_enum, default_value_t = InputFormat::Text)]
        input_format: InputFormat,

        #[arg(long, value_enum, default_value_t = AlphabetArg::Base57)]
        alphabet: AlphabetArg,
    },

    /// Decode a base57 or base58 string into bytes.
    BaselineDecode {
        #[arg(long)]
        input: String,

        #[arg(long, value_enum, default_value_t = AlphabetArg::Base57)]
        alphabet: AlphabetArg,

        #[arg(long, value_enum, default_value_t = OutputFormat::Hex)]
        output_format: OutputFormat,
    },

    /// Encode arbitrary bytes as framed base57 affine chunks.
    AffineEncode {
        #[arg(long)]
        input: String,

        #[arg(long, value_enum, default_value_t = InputFormat::Text)]
        input_format: InputFormat,

        #[arg(long, value_enum, default_value_t = ModeArg::Residue)]
        mode: ModeArg,
    },

    /// Decode framed base57 affine chunks back to bytes.
    AffineDecode {
        #[arg(long)]
        input: String,

        #[arg(long, value_enum, default_value_t = OutputFormat::Hex)]
        output_format: OutputFormat,
    },

    /// Show the same base-invariant payload through ordinary and affine renderings.
    ValueMap {
        #[arg(long)]
        input: String,

        #[arg(long, value_enum, default_value_t = InputFormat::Text)]
        input_format: InputFormat,

        #[arg(long, value_enum, default_value_t = AffineMapArg::Residue)]
        affine: AffineMapArg,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum InputFormat {
    Hex,
    Text,
    Base58,
    Base57,
    Affine,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Hex,
    Text,
    Base58,
    Base57,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum AlphabetArg {
    Base58,
    Base57,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ModeArg {
    Residue,
    Prime,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum AffineMapArg {
    None,
    Residue,
    Prime,
    Both,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.command {
        Command::BaselineEncode {
            input,
            input_format,
            alphabet,
        } => {
            let bytes = parse_input(&input, input_format)?;
            let encoded = encode_bytes(&bytes, alphabet.into())?;
            println!("{encoded}");
        }
        Command::BaselineDecode {
            input,
            alphabet,
            output_format,
        } => {
            let bytes = decode_to_bytes(&input, alphabet.into())?;
            println!("{}", format_output(&bytes, output_format)?);
        }
        Command::AffineEncode {
            input,
            input_format,
            mode,
        } => {
            let bytes = parse_input(&input, input_format)?;
            let encoded = encode_affine(&bytes, mode.into())?;
            println!("{}", encoded.notation);
        }
        Command::AffineDecode {
            input,
            output_format,
        } => {
            let decoded = decode_affine(&input)?;
            println!("{}", format_output(&decoded.payload, output_format)?);
        }
        Command::ValueMap {
            input,
            input_format,
            affine,
        } => {
            let bytes = parse_input(&input, input_format)?;
            let map = build_base_invariant_map(
                &bytes,
                matches!(affine, AffineMapArg::Residue | AffineMapArg::Both),
                matches!(affine, AffineMapArg::Prime | AffineMapArg::Both),
            )?;
            println!("{}", serde_json::to_string_pretty(&map)?);
        }
    }
    Ok(())
}

fn parse_input(input: &str, input_format: InputFormat) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(match input_format {
        InputFormat::Hex => parse_hex(input)?,
        InputFormat::Text => input.as_bytes().to_vec(),
        InputFormat::Base58 => decode_to_bytes(input, CodecAlphabet::Base58)?,
        InputFormat::Base57 => decode_to_bytes(input, CodecAlphabet::Base57)?,
        InputFormat::Affine => decode_canonical_payload(input, CanonicalInputFormat::Affine)?,
    })
}

fn format_output(bytes: &[u8], output_format: OutputFormat) -> Result<String, Box<dyn Error>> {
    Ok(match output_format {
        OutputFormat::Hex => format_hex(bytes),
        OutputFormat::Text => bytes_to_text(bytes)?,
        OutputFormat::Base58 => encode_bytes(bytes, CodecAlphabet::Base58)?,
        OutputFormat::Base57 => encode_bytes(bytes, CodecAlphabet::Base57)?,
    })
}

impl From<AlphabetArg> for CodecAlphabet {
    fn from(value: AlphabetArg) -> Self {
        match value {
            AlphabetArg::Base58 => CodecAlphabet::Base58,
            AlphabetArg::Base57 => CodecAlphabet::Base57,
        }
    }
}

impl From<ModeArg> for AffineCodecMode {
    fn from(value: ModeArg) -> Self {
        match value {
            ModeArg::Residue => AffineCodecMode::Residue,
            ModeArg::Prime => AffineCodecMode::Prime,
        }
    }
}
