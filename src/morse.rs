use hound::{Error as HoundError, WavSpec, WavWriter};

use rodio::{
    source::{Amplify, SineWave, Source, TakeDuration},
    OutputStream, Sink,
};

use std::{
    collections::HashSet,
    fs::File,
    io::BufWriter,
    str::{Split, SplitWhitespace},
    time::Duration,
};

pub struct Morse {
    morse_code: HashSet<(char, String)>,
    short_beep: Amplify<TakeDuration<SineWave>>,
    long_beep: Amplify<TakeDuration<SineWave>>,
    silence: Amplify<TakeDuration<SineWave>>,
    duration: Duration,
    sampling_rate: u32,
}

impl Morse {
    /// Crée une nouvelle instance de la structure Morse et initialise le HashSet morse_code avec les caractères Morse associés à leurs codes correspondants.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// fn main() {
    /// 	let morse = Morse::new();
    /// }
    /// ```
    pub fn new() -> Self {
        let mut morse_code = HashSet::new();
        morse_code.insert(('A', ".-".to_string()));
        morse_code.insert(('B', "-...".to_string()));
        morse_code.insert(('C', "-.-.".to_string()));
        morse_code.insert(('D', "-..".to_string()));
        morse_code.insert(('E', ".".to_string()));
        morse_code.insert(('F', "..-.".to_string()));
        morse_code.insert(('G', "--.".to_string()));
        morse_code.insert(('H', "....".to_string()));
        morse_code.insert(('I', "..".to_string()));
        morse_code.insert(('J', ".---".to_string()));
        morse_code.insert(('K', "-.-".to_string()));
        morse_code.insert(('L', ".-..".to_string()));
        morse_code.insert(('M', "--".to_string()));
        morse_code.insert(('N', "-.".to_string()));
        morse_code.insert(('O', "---".to_string()));
        morse_code.insert(('P', ".--.".to_string()));
        morse_code.insert(('Q', "--.-".to_string()));
        morse_code.insert(('R', ".-.".to_string()));
        morse_code.insert(('S', "...".to_string()));
        morse_code.insert(('T', "-".to_string()));
        morse_code.insert(('U', "..-".to_string()));
        morse_code.insert(('V', "...-".to_string()));
        morse_code.insert(('W', ".--".to_string()));
        morse_code.insert(('X', "-..-".to_string()));
        morse_code.insert(('Y', "-.--".to_string()));
        morse_code.insert(('Z', "--..".to_string()));
        morse_code.insert(('0', "-----".to_string()));
        morse_code.insert(('1', ".----".to_string()));
        morse_code.insert(('2', "..---".to_string()));
        morse_code.insert(('3', "...--".to_string()));
        morse_code.insert(('4', "....-".to_string()));
        morse_code.insert(('5', ".....".to_string()));
        morse_code.insert(('6', "-....".to_string()));
        morse_code.insert(('7', "--...".to_string()));
        morse_code.insert(('8', "---..".to_string()));
        morse_code.insert(('9', "----.".to_string()));
        morse_code.insert((',', "--..--".to_string()));
        morse_code.insert(('.', ".-.-.-".to_string()));
        morse_code.insert(('?', "..--..".to_string()));
        morse_code.insert(('\'', ".----.".to_string()));
        morse_code.insert(('!', "-.-.--".to_string()));
        morse_code.insert(('/', "-..-.".to_string()));
        morse_code.insert(('(', "-.--.".to_string()));
        morse_code.insert((')', "-.--.-".to_string()));
        morse_code.insert(('&', ".-...".to_string()));
        morse_code.insert((':', "---...".to_string()));
        morse_code.insert((';', "-.-.-.".to_string()));
        morse_code.insert(('=', "-...-".to_string()));
        morse_code.insert(('+', ".-.-.".to_string()));
        morse_code.insert(('-', "-....-".to_string()));
        morse_code.insert(('_', "..--.-".to_string()));
        morse_code.insert(('"', ".-..-.".to_string()));
        morse_code.insert(('$', "...-..-".to_string()));
        morse_code.insert(('@', ".--.-.".to_string()));
        morse_code.insert((' ', "/".to_string()));

        let short_beep: Amplify<TakeDuration<SineWave>> = SineWave::new(329.63)
            .take_duration(std::time::Duration::from_secs_f32(0.5))
            .amplify(0.20);
        let long_beep: Amplify<TakeDuration<SineWave>> = SineWave::new(392.0)
            .take_duration(std::time::Duration::from_secs_f32(1.))
            .amplify(0.20);
        let silence: Amplify<TakeDuration<SineWave>> = SineWave::new(0.0)
            .take_duration(std::time::Duration::from_secs_f32(1.))
            .amplify(0.20);

        Morse {
            morse_code,
            short_beep,
            long_beep,
            silence,
            duration: Duration::from_secs_f32(0.5),
            sampling_rate: 44100,
        }
    }

    /// Encode une chaîne de texte en code Morse.
    ///
    /// # Arguments
    ///
    /// * `text` - La chaîne de texte à encoder.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// fn main() {
    ///     let morse = Morse::new();
    ///     let encoded_text = morse.encode("HELLO WORLD");
    ///     assert_eq!(encoded_text, ".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
    /// }
    /// ```
    pub fn encode(&self, text: &str) -> String {
        let mut encoded: String = String::new();

        for c in text.chars() {
            if let Some(code) = self.get_morse_code(c) {
                encoded.push_str(&code);
                encoded.push(' '); // espace entre les caractères encodés
            }
        }

        encoded.trim().to_string() // supprime l'espace final et retourne le résultat
    }

    /// Décode une chaîne de code Morse en texte.
    ///
    /// # Arguments
    ///
    /// * `morse` - La chaîne de code Morse à décoder.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// fn main() {
    ///     let morse = Morse::new();
    ///     let decoded_text = morse.decode(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
    ///     assert_eq!(decoded_text, "HELLO WORLD");
    /// }
    /// ```
    pub fn decode(&self, morse: &str) -> String {
        let mut decoded: String = String::new();

        let words: Split<'_, &str> = morse.split("/"); // sépare les mots par le caractère '/'
        for word in words {
            let characters: SplitWhitespace<'_> = word.split_whitespace(); // sépare les caract
            for character in characters {
                if let Some(ch) = self.get_morse_character(character) {
                    decoded.push(ch);
                }
            }
            decoded.push(' '); // espace entre les mots décodés
        }

        decoded.trim().to_string() // supprime l'espace final et retourne le résultat
    }

    /// Vérifie si une chaîne de texte est constituée uniquement de caractères valides en code Morse (".", "-", "/").
    ///
    /// # Arguments
    ///
    /// * `text` - La chaîne de texte à vérifier.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// fn main() {
    ///     let morse = Morse::new();
    ///     assert_eq!(morse.is_morse(".... . .-.. .-.. ---"), true);
    ///     assert_eq!(morse.is_morse("HELLO"), false);
    /// }
    /// ```
    pub fn is_morse(&self, text: &str) -> bool {
        for c in text.chars() {
            if !['.', '-', '/', ' '].contains(&c) {
                return false;
            }
        }
        true
    }

    /// Vérifie si une chaîne de texte contient au moins un caractère valide en code Morse (".", "-", "/").
    ///
    /// # Arguments
    ///
    /// * `text` - La chaîne de texte à vérifier.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    ///     assert!(morse.contains_morse("HELLO .- WORLD"));
    ///     assert!(!morse.contains_morse("HELLO WORLD"));
    /// }
    /// ```
    pub fn contains_morse(&self, text: &str) -> bool {
        for c in text.chars() {
            if ['.', '-', '/'].contains(&c) {
                return true;
            }
        }
        false
    }

    /// Recherche et renvoie le code Morse associé à un caractère donné.
    ///
    /// # Arguments
    ///
    /// * `ch` - Le caractère pour lequel rechercher le code Morse.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    ///     assert_eq!(morse.get_morse_code('A'), Some(&".-".to_string()));
    ///     assert_eq!(morse.get_morse_code('Z'), Some(&"--..".to_string()));
    ///     assert_eq!(morse.get_morse_code('5'), Some(&".....".to_string()));
    ///     assert_eq!(morse.get_morse_code(' '), Some(&"/".to_string()));
    /// }
    /// ```
    pub fn get_morse_code(&self, ch: char) -> Option<&String> {
        self.morse_code
            .iter()
            .find_map(|(c, code)| if *c == ch { Some(code) } else { None })
    }

    /// Recherche et renvoie le caractère associé à un code Morse donné.
    ///
    /// # Arguments
    ///
    /// * `code` - Le code Morse pour lequel rechercher le caractère associé.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    ///     assert_eq!(morse.get_morse_character(".-"), Some('A'));
    ///     assert_eq!(morse.get_morse_character("--.."), Some('Z'));
    ///     assert_eq!(morse.get_morse_character("....."), Some('5'));
    ///     assert_eq!(morse.get_morse_character("/"), Some(' '));
    ///     assert_eq!(morse.get_morse_character("-..-..-"), None);
    /// }
    /// ```
    pub fn get_morse_character(&self, code: &str) -> Option<char> {
        self.morse_code
            .iter()
            .find_map(|(c, c_code)| if c_code == code { Some(*c) } else { None })
    }

    /// Joue un bip court.
    ///
    /// Cette fonction joue un bip court en utilisant le dispositif audio par défaut. Le bip court est
    /// obtenu à partir du son préenregistré `short_beep` de la structure `Morse`.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    /// 	// Exemple de bip court
    ///		println!("Exemple de bip court :");
    ///		morse.play_short_beep();
    /// }
    /// ```
    pub fn play_short_beep(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink : Sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(self.short_beep.clone());
        sink.sleep_until_end();
    }

    /// Joue un bip long.
    ///
    /// Cette fonction joue un bip long en utilisant le dispositif audio par défaut. Le bip long est
    /// obtenu à partir du son préenregistré `long_beep` de la structure `Morse`.   
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    /// 	// Exemple de bip long
    ///		println!("Exemple de bip long :");
    ///		morse.play_long_beep();
    /// }
    /// ```
    pub fn play_long_beep(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink : Sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(self.long_beep.clone());
        sink.sleep_until_end();
    }

    /// Joue un bip silencieux.
    ///
    /// Cette fonction joue un bip silencieux.   
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    /// 	// Exemple de bip long
    ///		println!("Exemple de bip silencieux :");
    ///		morse.play_silence();
    /// }
    /// ```
    pub fn play_silence(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink : Sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(self.silence.clone());
        sink.sleep_until_end();
    }

    /// Change le son du un bip court.
    /// Cette fonction remplace le son à jouer lors d'un bip court.   
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// use std::{ time::Duration};
    ///
    /// fn main() {
    ///     let mut morse = Morse::new();
    ///		morse.set_short_beep(300., Duration::from_secs_f32(0.5), 0.20);
    /// }
    /// ```
    pub fn set_short_beep(&mut self, frequency: f32, duration: Duration, amplification: f32) {
        self.short_beep = SineWave::new(frequency)
            .take_duration(duration)
            .amplify(amplification);
    }

    /// Change le son du un bip long.
    /// Cette fonction remplace le son à jouer lors d'un bip long.   
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// use std::{ time::Duration};
    ///
    /// fn main() {
    ///     let mut morse = Morse::new();
    ///		morse.set_long_beep(300., Duration::from_secs_f32(0.5), 0.20);
    /// }
    ///
    /// ```
    pub fn set_long_beep(&mut self, frequency: f32, duration: Duration, amplification: f32) {
        self.long_beep = SineWave::new(frequency)
            .take_duration(duration)
            .amplify(amplification);
    }

    /// Change le son du un bip silencieux.
    /// Cette fonction remplace le son à jouer lors d'un bip silencieux.   
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// use std::{ time::Duration};
    ///
    /// fn main() {
    ///     let mut morse = Morse::new();
    ///		morse.set_silence(300., Duration::from_secs_f32(0.5), 0.20);
    /// }
    ///
    pub fn set_silence(&mut self, frequency: f32, duration: Duration, amplification: f32) {
        self.silence = SineWave::new(frequency)
            .take_duration(duration)
            .amplify(amplification);
    }
    /// Change le temps entre chaque son lorsqu'on joue le morse en audio.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// use std::{ time::Duration};
    ///
    /// fn main() {
    ///     let mut morse = Morse::new();
    ///		morse.set_duration(Duration::from_secs_f32(0.5));
    /// }
    ///
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }
    /// Change le taux d'échantillonnage
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    /// use std::{ time::Duration};
    ///
    /// fn main() {
    ///     let mut morse = Morse::new();
    ///		morse.set_sampling_rate(8);
    /// }
    ///
    pub fn set_sampling_rate(&mut self, sampling_rate: u32) {
        self.sampling_rate = sampling_rate;
    }

    /// Joue un code Morse.
    ///
    /// Cette fonction joue un code Morse en utilisant des bips courts pour les points ('.') et des
    /// bips longs pour les tirets ('-'). Les pauses entre les signaux sont marquées par un espace (' ').
    ///
    /// # Arguments
    ///
    /// * `morse_code` - Le code Morse à jouer.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    ///     morse.play_morse_code(".- / -... / -.-.");  // Joue le code Morse "A B C"
    /// }
    /// ```
    pub fn play_morse_code(&self, morse_code: &str) {
        for c in morse_code.chars() {
            match c {
                '.' => self.play_short_beep(), // Joue un bip court pour le point '.'
                '-' => self.play_long_beep(),  // Joue un bip long pour le tiret '-'
                _ => self.play_silence(),      // Ignorer les autres caractères
            }
            // Attendre une courte pause entre chaque signal
            std::thread::sleep(self.duration);
        }
    }

    /// Traduit le texte entre le code Morse et le texte normal.
    ///
    /// Cette méthode prend une chaîne de caractères `text` en paramètre et effectue
    /// la traduction entre le texte normal et le code Morse. Chaque mot dans la chaîne
    /// est analysé pour déterminer s'il est en code Morse ou en texte normal. Les mots
    /// en code Morse sont décodés en texte normal, tandis que les mots en texte normal
    /// sont encodés en code Morse. La traduction résultante est retournée sous forme de
    /// chaîne de caractères.
    ///
    /// # Arguments
    ///
    /// * `text` - Le texte à traduire entre le code Morse et le texte normal.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    ///     let text = "HELLO WORLD ... --- ...";
    ///     let translated = morse.translate(text);
    ///     println!("Traduction : {}", translated);
    ///     let morse_code = "... --- ...";
    ///     let translated = morse.translate(morse_code);
    ///     println!("Traduction : {}", translated);
    /// }
    /// ```
    pub fn translate(&self, text: &str) -> String {
        let mut translated: String = String::new();
        let words: Vec<&str> = text.split(' ').collect();

        for word in words {
            if self.is_morse(word) {
                // Le mot est en code Morse, le traduire en texte normal
                let decoded_word = self.decode(word);
                translated.push_str(&decoded_word);
            } else {
                // Le mot est en texte normal, le traduire en code Morse
                let encoded_word = self.encode(word);
                translated.push_str(&encoded_word);
            }
            translated.push(' ');
        }

        translated.trim().to_string()
    }

    /// Exporter un code Morse en audio.
    ///
    /// # Arguments
    ///
    /// * `morse_code` - Le code Morse à jouer.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    ///     let _ = morse.to_audio(".- / -... / -.-.", "output.wav");  // exporte le son dans le fichier output.wav
    /// }
    /// ```
    pub fn to_audio(&self, morse_code: &str, filename: &str) -> Result<(), HoundError> {
        let mut samples: Vec<f32> = Vec::new();
        // Créer un flux audio
        let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
        for c in morse_code.chars() {
            match c {
                // Ajoute les échantillons de bip à la liste des échantillons
                '.' => samples
                    .extend_from_slice(&self.short_beep.clone().collect::<Vec<f32>>().as_slice()),
                '-' => samples
                    .extend_from_slice(&self.long_beep.clone().collect::<Vec<f32>>().as_slice()),
                _ => samples
                    .extend_from_slice(&self.silence.clone().collect::<Vec<f32>>().as_slice()),
            }
        }
        // Exporte les échantillons en audio
        self.export_audio(filename, &samples, self.sampling_rate)
    }

    /// Exporte les échantillons audio vers un fichier.
    ///
    /// # Arguments
    ///
    /// * `filename` - Le nom du fichier à créer.
    /// * `samples` - Les échantillons audio à exporter.
    /// * `sampling_rate` - Le taux d'échantillonnage des échantillons audio.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use crate::morseus::Morse;
    ///
    /// fn main() {
    ///     let morse = Morse::new();
    ///     let samples: Vec<f32> = vec![0.1, 0.2, 0.3];
    ///     morse.export_audio("output.wav", &samples, 44100).unwrap();
    /// }
    /// ```
    pub fn export_audio(
        &self,
        filename: &str,
        samples: &[f32],
        sampling_rate: u32,
    ) -> Result<(), HoundError> {
        // Spécification du fichier WAV
        let spec: WavSpec = WavSpec {
            channels: 1,                             // Nombre de canaux audio (mono)
            sample_rate : sampling_rate,                             // Taux d'échantillonnage
            bits_per_sample: 16,                     // Nombre de bits par échantillon (16 bits)
            sample_format: hound::SampleFormat::Int, // Format d'échantillonage (entier)
        };
        // Création du writer pour écrire les données audio dans le fichier
        let mut writer: WavWriter<BufWriter<File>> = WavWriter::create(filename, spec)?;
        // Parcours chaque échantillon dans le tableau de samples
        for &sample in samples {
            // Conversion de l'échantillon en i16 (format attendu par le writer)
            let sample_i16 = (sample * std::i16::MAX as f32) as i16;
            // Écriture de l'échantillon dans le fichier
            writer.write_sample(sample_i16)?;
        }
        // Finalisation de l'écriture du fichier
        writer.finalize()?;
        Ok(())
    }
}
