use rand::seq::SliceRandom; // For randomly selecting from a slice
use rand::thread_rng; // For getting the thread-local random number generator
use serde::Deserialize; // For deserializing JSON
use std::fs; // For file system operations (reading file)
use std::io::{self, Write}; // For standard input/output and flushing

// --- STRUCT DEFINITIONS (Matching JSON structure) ---
#[derive(Debug, Deserialize)]
struct WordLists {
    four_letter_words: Vec<String>,
    five_letter_words: Vec<String>,
    six_letter_words: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Root {
    word_lists: WordLists,
}

// --- GLOBAL CONSTANTS ---
const MAX_WRONG_GUESSES: u8 = 6;
const WORDS_JSON_PATH: &str = "hidden_words.json";

// --- FUNCTION 1: load_words_from_json ---
// Loads and parses word lists from a JSON file.
// Returns a Result type: Ok(Root struct) on success, Err(Error type) on failure.
fn load_words_from_json() -> Result<Root, Box<dyn std::error::Error>> {
    println!("Attempting to load words from: {}", WORDS_JSON_PATH);
    let json_content = fs::read_to_string(WORDS_JSON_PATH)?;
    let root: Root = serde_json::from_str(&json_content)?;
    println!("Words loaded successfully.");
    Ok(root)
}

// --- FUNCTION 2: get_word_list_choice ---
// Prompts the user to select a word list type (4, 5, or 6 letters).
// Returns an Option<&Vec<String>>: Some(reference to list) on valid choice, None otherwise.
fn get_word_list_choice(all_words: &Root) -> Option<&Vec<String>> {
    loop {
        println!("\nChoose a word list for Hangman:");
        println!("1. 4-letter words");
        println!("2. 5-letter words");
        println!("3. 6-letter words");
        println!("Enter your choice (1, 2, or 3, or 'q' to quit):");

        let mut choice_input = String::new();
        io::stdin()
            .read_line(&mut choice_input)
            .expect("Failed to read line");
        let choice = choice_input.trim(); // Remove newline and whitespace

        if choice.eq_ignore_ascii_case("q") {
            return None; // User wants to quit
        }

        match choice {
            "1" => return Some(&all_words.word_lists.four_letter_words),
            "2" => return Some(&all_words.word_lists.five_letter_words),
            "3" => return Some(&all_words.word_lists.six_letter_words),
            _ => {
                println!("Invalid choice. Please enter 1, 2, 3, or 'q'.");
                // Loop continues
            }
        }
    }
}

// --- FUNCTION 3: select_random_word ---
// Selects a random word from a given list.
// Returns a reference to a String (the selected word). Panics if list is empty.
fn select_random_word<'a>(word_list: &'a Vec<String>) -> &'a String {
    let mut rng = thread_rng();
    word_list
        .choose(&mut rng)
        .expect("Word list is empty, cannot select a word.")
}

// --- FUNCTION 4: display_game_state (Helper for play_hangman_round) ---
// Displays the current state of the game to the user.
fn display_game_state(
    hidden_display_chars: &[char], // Using slice for efficiency
    guessed_letters: &[char],     // Using slice for efficiency
    remaining_guesses: u8,
) {
    println!("\nWord: {}", hidden_display_chars.iter().collect::<String>());
    println!(
        "Guessed Letters: {}",
        guessed_letters
            .iter()
            .map(|&c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!("Guesses Left: {}", remaining_guesses);
}

// --- FUNCTION 5: play_hangman_round ---
// Contains the core game logic and user interaction for one round.
// Returns BOOLEAN: TRUE if player wins, FALSE if player loses.
fn play_hangman_round(secret_word_str: &str) -> bool {
    // Convert secret word to uppercase characters for case-insensitive comparison
    let secret_word_chars: Vec<char> = secret_word_str.to_uppercase().chars().collect();
    let word_length = secret_word_chars.len();

    // Initialize the hidden word display with underscores
    let mut hidden_word_display: Vec<char> = vec!['_'; word_length];

    let mut guessed_letters: Vec<char> = Vec::new(); // Stores unique guessed letters
    let mut wrong_guesses_count: u8 = 0;

    println!("\n--- Hangman Round Started! ---");
    println!("Your word has {} letters.", word_length);

    loop {
        // Display current game state
        display_game_state(
            &hidden_word_display,
            &guessed_letters,
            MAX_WRONG_GUESSES - wrong_guesses_count,
        );

        // Check for game over (loss)
        if wrong_guesses_count >= MAX_WRONG_GUESSES {
            println!("\n--- GAME OVER! ---");
            println!("You ran out of guesses. The word was: {}", secret_word_str);
            return false; // Player lost
        }

        // Check for win condition
        if hidden_word_display.iter().all(|&c| c != '_') {
            println!("\n--- CONGRATULATIONS! ---");
            println!("You guessed the word: {}", secret_word_str);
            return true; // Player won
        }

        // Prompt for guess
        print!("Guess a letter: ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt appears

        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");
        let guess_input = guess_input.trim(); // Remove newline and whitespace

        // Validate guess input
        if guess_input.len() != 1 {
            println!("Invalid input. Please enter exactly one letter.");
            continue; // Ask again
        }

        let guessed_char = guess_input
            .chars()
            .next()
            .unwrap()
            .to_ascii_uppercase(); // Get char and convert to uppercase

        if !guessed_char.is_ascii_alphabetic() {
            println!("Invalid input. Please enter an alphabetic character.");
            continue;
        }

        // Check if letter was already guessed
        if guessed_letters.contains(&guessed_char) {
            println!("You already guessed '{}'. Try a new letter.", guessed_char);
            continue;
        }

        // Add the new guess to the list of guessed letters
        guessed_letters.push(guessed_char);
        guessed_letters.sort_unstable(); // Keep list sorted for better display

        // Compare with secret word and update display (handling duplicates)
        let mut found_in_word = false;
        for (i, &secret_char) in secret_word_chars.iter().enumerate() {
            if secret_char == guessed_char {
                hidden_word_display[i] = guessed_char;
                found_in_word = true;
            }
        }

        // Handle correct/incorrect guess
        if found_in_word {
            println!("Good guess! '{}' is in the word.", guessed_char);
        } else {
            println!("'{}' is not in the word. You lose a guess.", guessed_char);
            wrong_guesses_count += 1;
        }
    }
}

// --- MAIN PROGRAM FLOW ---
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_words = match load_words_from_json() {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Failed to start game due to data loading error: {}", e);
            return Err(e); // Exit program with error
        }
    };

    loop {
        let selected_list_option = get_word_list_choice(&all_words);

        let selected_word_list = match selected_list_option {
            Some(list) => list,
            None => {
                // User chose to quit
                println!("Exiting Hangman game. Goodbye!");
                break; // Exit the main game loop
            }
        };

        if selected_word_list.is_empty() {
            println!("The selected word list is empty. Please check your JSON file.");
            continue; // Ask for choice again
        }

        let secret_word = select_random_word(selected_word_list);

        // Play the actual game round
        let _player_won = play_hangman_round(secret_word); // We don't strictly need to use `_player_won` here

        // Ask if user wants to play another round
        println!("\nPlay another round? (yes/no)");
        print!("> "); // Simple prompt for consistency
        io::stdout().flush()?;

        let mut play_again_input = String::new();
        io::stdin().read_line(&mut play_again_input)?;
        if !play_again_input.trim().eq_ignore_ascii_case("yes") {
            println!("Thanks for playing!");
            break; // Exit the main game loop
        }
    }

    Ok(())
}