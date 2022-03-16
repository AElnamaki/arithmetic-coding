use std::ops::Range;

use arithmetic_coding::{Decoder, Encoder, Model};
use bitstream_io::{BigEndian, BitReader, BitWrite, BitWriter};

#[derive(Debug, Clone)]
pub struct StringModel {
    alphabet: Vec<char>,
}

impl StringModel {
    pub fn new(alphabet: Vec<char>) -> Self {
        Self { alphabet }
    }
}

impl Model for StringModel {
    type Symbol = char;

    fn probability(&self, symbol: Option<&Self::Symbol>) -> Range<u32> {
        if let Some(char) = symbol {
            let index = self
                .alphabet
                .iter()
                .position(|x| x == char)
                .unwrap_or_else(|| panic!("unexpected character: {}", char))
                as u32;
            index..(index + 1)
        } else {
            let alphabet_length = self.alphabet.len() as u32;
            alphabet_length..(alphabet_length + 1)
        }
    }

    fn symbol(&self, value: u32) -> Option<Self::Symbol> {
        self.alphabet.get(value as usize).copied()
    }

    fn denominator(&self) -> u32 {
        self.alphabet.len() as u32 + 1
    }
}

fn main() {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \
                            .,\n-':()[]#*;\"!?*&é/àâè%@$";
    println!("number of symbols: {}", ALPHABET.chars().count());
    let input = include_str!("./sherlock.txt");
    let input_bytes = input.bytes().len();

    println!(
        "file bytes: {}",
        std::fs::metadata("./examples/sherlock.txt").unwrap().len()
    );

    let model = StringModel::new(ALPHABET.chars().collect());

    let output = Vec::default();
    let mut bitwriter = BitWriter::endian(output, BigEndian);
    let mut encoder = Encoder::new(model.clone());

    println!("encoding...");
    encoder.encode(input.chars(), &mut bitwriter).unwrap();
    bitwriter.byte_align().unwrap();

    let buffer = bitwriter.into_writer();

    let output_bytes = buffer.len();

    println!("input bytes: {}", input_bytes);
    println!("output bytes: {}", output_bytes);

    println!(
        "compression ratio: {}",
        input_bytes as f32 / output_bytes as f32
    );

    // println!("buffer: {:?}", &buffer);

    let bitreader = BitReader::endian(buffer.as_slice(), BigEndian);

    println!("\ndecoding...");
    let mut decoder = Decoder::new(model, bitreader).unwrap();
    let mut output = String::new();
    while let Some(symbol) = decoder.decode_symbol().unwrap() {
        output.push(symbol);
    }

    // println!("{}", output);
}
