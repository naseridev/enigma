use clap::{Arg, Command};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, write};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::process;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ";
const DEFAULT_ROTOR_FILE: &str = "./daily_key.enigma";
const DEFAULT_PLUGBOARD_FILE: &str = "./plugboard.toml";

#[derive(Debug)]
enum EnigmaError {
    InvalidRotorPosition(char),
    InvalidMessage(String),
    InvalidPlugboardPair(String),
    FileError(String),
    SerializationError(String),
}

impl std::fmt::Display for EnigmaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnigmaError::InvalidRotorPosition(c) => write!(f, "Invalid rotor position: {}", c),
            EnigmaError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            EnigmaError::InvalidPlugboardPair(pair) => {
                write!(f, "Invalid plugboard pair: {}", pair)
            }
            EnigmaError::FileError(msg) => write!(f, "File error: {}", msg),
            EnigmaError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for EnigmaError {}

impl From<std::io::Error> for EnigmaError {
    fn from(err: std::io::Error) -> Self {
        EnigmaError::FileError(err.to_string())
    }
}

impl From<bincode::Error> for EnigmaError {
    fn from(err: bincode::Error) -> Self {
        EnigmaError::SerializationError(err.to_string())
    }
}

impl From<toml::de::Error> for EnigmaError {
    fn from(err: toml::de::Error) -> Self {
        EnigmaError::SerializationError(err.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct RotorState {
    rotor1: String,
    rotor2: String,
    rotor3: String,
}

#[derive(Deserialize)]
struct PlugboardConfig {
    pairs: Vec<String>,
}

struct Rotor {
    wiring: String,
    position: usize,
    notch_position: usize,
}

impl Rotor {
    fn new(wiring: String, notch_position: usize) -> Self {
        Self {
            wiring,
            position: 0,
            notch_position,
        }
    }

    fn set_position(&mut self, pos: char) -> Result<(), EnigmaError> {
        let position = ALPHABET
            .find(pos)
            .ok_or(EnigmaError::InvalidRotorPosition(pos))?;
        self.position = position;
        Ok(())
    }

    fn at_notch(&self) -> bool {
        self.position == self.notch_position
    }

    fn step(&mut self) {
        self.position = (self.position + 1) % ALPHABET.len();
    }

    fn encode_forward(&self, input: usize) -> usize {
        let offset = (input + self.position) % ALPHABET.len();
        let encoded_char = self.wiring.chars().nth(offset).unwrap();
        let encoded_pos = ALPHABET.find(encoded_char).unwrap();
        (encoded_pos + ALPHABET.len() - self.position) % ALPHABET.len()
    }

    fn encode_backward(&self, input: usize) -> usize {
        let offset = (input + self.position) % ALPHABET.len();
        let input_char = ALPHABET.chars().nth(offset).unwrap();
        let pos = self.wiring.find(input_char).unwrap();
        (pos + ALPHABET.len() - self.position) % ALPHABET.len()
    }
}

struct Reflector {
    wiring: String,
}

impl Reflector {
    fn new() -> Self {
        let alphabet_len = ALPHABET.len();
        let mut wiring = vec!['\0'; alphabet_len];

        let alphabet_chars: Vec<char> = ALPHABET.chars().collect();
        let mut used = vec![false; alphabet_len];

        for i in 0..alphabet_len {
            if used[i] {
                continue;
            }

            let mut pair_found = false;
            for j in (i + 1)..alphabet_len {
                if !used[j] {
                    wiring[i] = alphabet_chars[j];
                    wiring[j] = alphabet_chars[i];
                    used[i] = true;
                    used[j] = true;
                    pair_found = true;
                    break;
                }
            }

            if !pair_found && alphabet_len % 2 == 1 && i == alphabet_len - 1 {
                for j in 0..i {
                    if wiring[j] == alphabet_chars[i] {
                        let old_pair = ALPHABET.find(wiring[j]).unwrap();
                        wiring[i] = alphabet_chars[old_pair];
                        wiring[old_pair] = alphabet_chars[i];
                        wiring[j] = alphabet_chars[i];
                        break;
                    }
                }
            }
        }

        Self {
            wiring: wiring.into_iter().collect(),
        }
    }

    fn reflect(&self, input: usize) -> usize {
        let reflected_char = self.wiring.chars().nth(input).unwrap();
        ALPHABET.find(reflected_char).unwrap()
    }
}

struct Plugboard {
    mapping: HashMap<char, char>,
}

impl Plugboard {
    fn new() -> Self {
        let mapping = ALPHABET.chars().map(|c| (c, c)).collect();
        Self { mapping }
    }

    fn from_pairs(pairs: Vec<String>) -> Result<Self, EnigmaError> {
        let mut mapping: HashMap<char, char> = ALPHABET.chars().map(|c| (c, c)).collect();

        for pair in pairs {
            if pair.len() != 2 {
                return Err(EnigmaError::InvalidPlugboardPair(pair));
            }

            let chars: Vec<char> = pair.chars().collect();
            let a = chars[0];
            let b = chars[1];

            if !ALPHABET.contains(a) || !ALPHABET.contains(b) {
                return Err(EnigmaError::InvalidPlugboardPair(pair));
            }

            if mapping.get(&a) != Some(&a) || mapping.get(&b) != Some(&b) {
                return Err(EnigmaError::InvalidPlugboardPair(format!(
                    "Duplicate mapping for {}",
                    pair
                )));
            }

            mapping.insert(a, b);
            mapping.insert(b, a);
        }

        Ok(Self { mapping })
    }

    fn swap(&self, c: char) -> char {
        *self.mapping.get(&c).unwrap_or(&c)
    }
}

struct EnigmaMachine {
    rotor1: Rotor,
    rotor2: Rotor,
    rotor3: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl EnigmaMachine {
    fn new(
        rotor_file: &str,
        plugboard_file: Option<&str>,
        positions: &str,
    ) -> Result<Self, EnigmaError> {
        if !Path::new(rotor_file).exists() {
            return Err(EnigmaError::FileError(format!(
                "Rotor file '{}' not found",
                rotor_file
            )));
        }

        let file = File::open(rotor_file)?;
        let reader = BufReader::new(file);
        let rotor_state: RotorState = bincode::deserialize_from(reader)?;

        let mut rotor1 = Rotor::new(rotor_state.rotor1, 16);
        let mut rotor2 = Rotor::new(rotor_state.rotor2, 4);
        let mut rotor3 = Rotor::new(rotor_state.rotor3, 21);

        if positions.len() != 3 {
            return Err(EnigmaError::InvalidMessage(
                "Rotor positions must be 3 characters".to_string(),
            ));
        }

        let pos_chars: Vec<char> = positions.chars().collect();

        rotor1.set_position(pos_chars[0])?;
        rotor2.set_position(pos_chars[1])?;
        rotor3.set_position(pos_chars[2])?;

        let plugboard = if let Some(pb_file) = plugboard_file {
            if Path::new(pb_file).exists() {
                Self::load_plugboard(pb_file)?
            } else {
                Plugboard::new()
            }
        } else {
            Plugboard::new()
        };

        Ok(Self {
            rotor1,
            rotor2,
            rotor3,
            reflector: Reflector::new(),
            plugboard,
        })
    }

    fn load_plugboard(file_path: &str) -> Result<Plugboard, EnigmaError> {
        let contents = std::fs::read_to_string(file_path)?;
        let config: PlugboardConfig = toml::from_str(&contents)?;

        Plugboard::from_pairs(config.pairs)
    }

    fn step_rotors(&mut self) {
        let right_stepping = true;
        let middle_at_notch = self.rotor2.at_notch();
        let right_at_notch = self.rotor1.at_notch();

        if middle_at_notch {
            self.rotor2.step();
            self.rotor3.step();
        }

        if right_at_notch {
            self.rotor2.step();
        }

        if right_stepping {
            self.rotor1.step();
        }
    }

    fn encode_char(&mut self, c: char) -> Result<char, EnigmaError> {
        if !ALPHABET.contains(c) {
            return Err(EnigmaError::InvalidMessage(format!(
                "Invalid character: {}",
                c
            )));
        }

        self.step_rotors();

        let plugboard_out = self.plugboard.swap(c);
        let mut signal = ALPHABET.find(plugboard_out).unwrap();

        signal = self.rotor1.encode_forward(signal);
        signal = self.rotor2.encode_forward(signal);
        signal = self.rotor3.encode_forward(signal);

        signal = self.reflector.reflect(signal);

        signal = self.rotor3.encode_backward(signal);
        signal = self.rotor2.encode_backward(signal);
        signal = self.rotor1.encode_backward(signal);

        let output_char = ALPHABET.chars().nth(signal).unwrap();
        let final_char = self.plugboard.swap(output_char);

        Ok(final_char)
    }

    fn encode_message(&mut self, message: &str) -> Result<String, EnigmaError> {
        if message.is_empty() {
            return Err(EnigmaError::InvalidMessage("Empty message".to_string()));
        }

        let mut result = String::with_capacity(message.len());

        for c in message.chars() {
            result.push(self.encode_char(c)?);
        }

        Ok(result)
    }
}

fn generate_rotors(output_file: &str) -> Result<(), EnigmaError> {
    let mut rng = thread_rng();
    let mut rotors = Vec::with_capacity(3);

    for _ in 0..3 {
        let mut chars: Vec<char> = ALPHABET.chars().collect();

        loop {
            chars.shuffle(&mut rng);
            let rotor: String = chars.iter().collect();

            let has_fixed_point = ALPHABET
                .chars()
                .enumerate()
                .any(|(i, c)| rotor.chars().nth(i).unwrap() == c);

            if !has_fixed_point {
                rotors.push(rotor);
                break;
            }
        }
    }

    let rotor_state = RotorState {
        rotor1: rotors[0].clone(),
        rotor2: rotors[1].clone(),
        rotor3: rotors[2].clone(),
    };

    let file = File::create(output_file)?;
    let writer = BufWriter::new(file);

    bincode::serialize_into(writer, &rotor_state)?;

    println!("Rotor configuration saved to: {}", output_file);
    Ok(())
}

fn generate_plugboard(output_file: &str) -> Result<(), EnigmaError> {
    let plugboard_content = r#"
# Enigma Plugboard Configuration
# Each pair swaps two characters bidirectionally
# Use two-character strings like "ab", "CD", "X ", etc.

pairs = [
    # "ab",  # a <-> b
    # "CD",  # C <-> D
    # "X ",  # X <-> space
]
"#;

    write(output_file, plugboard_content)?;
    println!("Plugboard configuration generated at: {}", output_file);
    Ok(())
}

fn main() {
    let matches = Command::new("enigma")
        .version("3.0")
        .author("Definitely not the Nazis")
        .about("A proper WWII Enigma machine cipher with rotors and plugboard.")
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .help("Generate new rotor configuration")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("generate_plugboard")
                .short('p')
                .long("generate-plugboard")
                .help("Generate plugboard configuration file")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("rotor_file")
                .short('r')
                .long("rotor-file")
                .value_name("FILE")
                .help("Path to rotor configuration file")
                .default_value(DEFAULT_ROTOR_FILE),
        )
        .arg(
            Arg::new("plugboard_file")
                .short('b')
                .long("plugboard-file")
                .value_name("FILE")
                .help("Path to plugboard configuration file")
                .default_value(DEFAULT_PLUGBOARD_FILE),
        )
        .arg(
            Arg::new("positions")
                .short('s')
                .long("start-positions")
                .value_name("POSITIONS")
                .help("Initial rotor positions (3 chars)")
                .default_value("aaa"),
        )
        .arg(
            Arg::new("message")
                .help("Message to encrypt/decrypt")
                .required_unless_present_any(["generate", "generate_plugboard"]),
        )
        .get_matches();

    let mut generated_something = false;

    if matches.get_flag("generate") {
        let rotor_file = matches.get_one::<String>("rotor_file").unwrap();
        if let Err(e) = generate_rotors(rotor_file) {
            eprintln!("Error generating rotors: {}", e);
            process::exit(1);
        }
        generated_something = true;
    }

    if matches.get_flag("generate_plugboard") {
        let plugboard_file = matches.get_one::<String>("plugboard_file");
        let output_file = plugboard_file
            .map(|s| s.as_str())
            .unwrap_or(DEFAULT_PLUGBOARD_FILE);
        if let Err(e) = generate_plugboard(output_file) {
            eprintln!("Error generating plugboard: {}", e);
            process::exit(1);
        }
        generated_something = true;
    }

    if generated_something {
        return;
    }

    let rotor_file = matches.get_one::<String>("rotor_file").unwrap();
    let plugboard_file = matches.get_one::<String>("plugboard_file");

    let positions = matches.get_one::<String>("positions").unwrap();
    let message = matches.get_one::<String>("message").unwrap();

    let mut enigma =
        match EnigmaMachine::new(rotor_file, plugboard_file.map(|s| s.as_str()), positions) {
            Ok(machine) => machine,
            Err(e) => {
                eprintln!("Error initializing Enigma machine: {}", e);
                process::exit(1);
            }
        };

    match enigma.encode_message(message) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error encoding message: {}", e);
            process::exit(1);
        }
    }
}
