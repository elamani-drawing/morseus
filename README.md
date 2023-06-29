# MORSEUS

Morseus est une bibliothèque Rust permettant d'encoder et de décoder du texte en code Morse. Elle offre également la possibilité de jouer le code Morse en émettant des signaux sonores à l'aide du dispositif audio par défaut de votre système, ainsi que d'exporter le code Morse en fichiers audio WAV.

## Fonctionnalités
- Encode du texte en code Morse.
- Décode du code Morse en texte.
- Vérifie si une chaîne de texte est constituée uniquement de caractères valides en code Morse.
- Vérifie si une chaîne de texte contient au moins un caractère valide en code Morse.
- Joue des bips courts, longs et des silences en utilisant le dispositif audio par défaut.

## Installation
Pour utiliser Morseus dans votre projet, ajoutez la dépendance suivante à votre fichier Cargo.toml :
```toml
[dependencies]
morseus = "0.1.0"
```

## Contributions
Les contributions sont les bienvenues! Si vous souhaitez améliorer Morseus, veuillez ouvrir une pull request sur GitHub.

## License
Ce projet est sous [``licence MIT``](LICENSE). Veuillez consulter le fichier [``LICENSE``](LICENSE) pour plus d'informations.