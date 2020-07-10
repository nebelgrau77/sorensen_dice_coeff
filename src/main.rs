extern crate clap;

use clap::{AppSettings, App, Arg};
use std::borrow::Cow;

fn main() {

    let matches = App::new("Sorensen-Dice coefficient")
                    .about("Calculates Sorensen-Dice coefficient between two words.")
                    .setting(AppSettings::DisableVersion)
                    .arg(
                        Arg::with_name("word_A")
                        .help("First word")
                        .use_delimiter(false)
                        .required(true),
                    )
                    .arg(
                        Arg::with_name("word_B")
                        .help("Second word")
                        .use_delimiter(false)
                        .required(true),
                    )
                    .get_matches();

    let word_a = matches.value_of("word_A").unwrap();
    let word_b = matches.value_of("word_B").unwrap();

    let word_a = word_a.trim().to_ascii_lowercase();

    let word_b = word_b.trim().to_ascii_lowercase();

    let coeff: f32 = sd_coeff(&word_a, &word_b);
    
    println!("Sørensen-Dice coefficient is {:.03}", coeff);
      
}

fn sd_coeff(word_a: &str, word_b: &str) -> f32 {
    
    // calculate Sørensen–Dice coefficient

    let bigrams_a = get_bigrams(word_a);
    let bigrams_b = get_bigrams(word_b);
    
    let common = compare_bigrams(&bigrams_a, &bigrams_b);
    
    let coeff: f32 = (2.0 * common as f32) / (bigrams_a.len() + bigrams_b.len()) as f32;
    
    return coeff;
    
}

fn get_bigrams(word: &str) -> Vec<Cow<str>> {
    
    // split a word into bigrams

    let mut bigrams: Vec<_> = Vec::new();
    
    let word_vec = word.chars().collect::<Vec<_>>();

    for i in 0..(word_vec.len()-1) {

        let bigram = &word_vec[i..i+2];

        let mut new_bigram = "".to_string();

        for item in bigram.iter() {
            new_bigram.push(*item);
        }

        bigrams.push(Cow::Owned(new_bigram));

    }

    return bigrams;

}

fn compare_bigrams(bigrams_a: &[Cow<str>], bigrams_b: &[Cow<str>]) -> u8 {
    
    // count how many bigrams are common between two words
    
    let mut common_count: u8 = 0;
        
    for item_a in bigrams_a.iter() {
            
        if bigrams_b.contains(item_a) {
            common_count += 1;        
        }    
    }
        
    return common_count;
}