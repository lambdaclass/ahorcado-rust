# Hangman Game

## How to build

Install cargo and run `cargo build && cargo run` on the root folder of the project.

---

## About the project

This is a rust learning excercise with basic concepts like: Structs, Modularity and File reading.

### How it works 

The code is separated into a game struct with the basic game logic: taking a letter as a guess and displaying the current state of the game. The main file deals with the start and end of game: it reads a file to get the word that will be guessed and it deals with starting and finishing the game.

### Current problems & possible improvements

Right now, the words that can be guessed must be ASCII, this leaves words with accents out. There is also no sanitization of the input file, for example, spaces and other simbols can form part of a word to be guessed.
The list of words has to be updated and mantained manually; this is something that can be eventually fixed by accessing the computers internal dictionary (which can change depending on the OS) or by looking words up on the internet.
Lastly, a feature that would be good to add is to have the hangman be drawn as the player misses letters.
