use morseus::Morse;

fn main() {
    let morse : Morse = Morse::new();

    // Exemple d'encodage
    let encoded_text : String = morse.encode("HELLO WORLD");
    println!("Texte encodé : {}", encoded_text);

    // Exemple de décodage
    let decoded_morse : String = morse.decode(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
    println!("Code Morse décodé : {}", decoded_morse);

    // Exemple de vérification de chaîne Morse
    let morse_text : &str = "... --- ...";
    println!("Est-ce que '{}' est une chaîne Morse valide ? {}", morse_text, morse.is_morse(morse_text));

    let invalid_text : &str = "HELLO";
    println!("Est-ce que '{}' est une chaîne Morse valide ? {}", invalid_text, morse.is_morse(invalid_text));

    // Exemple de recherche de code Morse associé à un caractère
    let morse_code : Option<&String> = morse.get_morse_code('A');
    match morse_code {
        Some(code) => println!("Code Morse associé à 'A' : {}", code),
        None => println!("Aucun code Morse trouvé pour 'A'."),
    }

    // Exemple de recherche de caractère associé à un code Morse
    let character : Option<char> = morse.get_morse_character(".-");
    match character {
        Some(ch) => println!("Caractère associé à '.-' : {}", ch),
        None => println!("Aucun caractère trouvé pour le code Morse '.-'."),
    }
	
	// Exemple de traduction de texte entre le code Morse et le texte normal
    let text : &str  = "HELLO WORLD ... --- ...";
    let translated : String  = morse.translate(text);
    println!("Traduction : {}", translated);

    let morse_code : &str = "... --- ... HELLO WORD";
    let translated :  String = morse.translate(morse_code);
    println!("Traduction : {}", translated);

    // Exemple pour jouer le son du code
    morse.play_morse_code(".. -- ."); 
    // exporte le code morse en audio
    let _ = morse.to_audio(".. -- .", "output.wav");
}
