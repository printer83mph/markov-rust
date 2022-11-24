use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use rand::prelude::random;

// TODO: use this babey
enum Capitalization {
    Ignore,
    Match,
    Capitalize,
}

pub struct Config {
    paragraph_delimiter: char,
    word_delimiters: HashSet<char>,
}

impl Config {
    pub fn prose() -> Config {
        Config {
            paragraph_delimiter: '\n',
            word_delimiters: HashSet::from([' ', '\t']),
        }
    }
}

struct WordStats {
    word: String,
    occurrences: i32,
    next_occurrences: Vec<i32>,
    next_total: i32,
    start_occurrences: i32,
    end_occurrences: i32,
}

impl WordStats {
    fn new(word: &str, index: usize) -> WordStats {
        WordStats {
            word: word.to_string(),
            occurrences: 0,
            next_occurrences: vec![0; index + 1],
            next_total: 0,
            start_occurrences: 0,
            end_occurrences: 0,
        }
    }

    fn get_next_probabilities(&self) -> Vec<f64> {
        self.next_occurrences
            .iter()
            .map(|o| *o as f64 / self.next_total as f64)
            .collect()
    }

    fn get_end_probability(&self) -> f64 {
        self.end_occurrences as f64 / self.occurrences as f64
    }
}

/// Picks a weighted random index from a `Vec` of probabilities
fn pick_random(probabilities: &Vec<f64>) -> usize {
    let r = random();

    // cumulatively add until we reach random number
    let mut cum: f64 = 0.0;
    for i in 0..probabilities.len() {
        let p = probabilities[i];
        cum += p;
        if cum > r {
            return i;
        }
    }

    // has to be the last guy if we get here
    probabilities.len() - 1
}

/// Markov chain model. Contains known words, and stats connecting them.
pub struct Model {
    config: Config,
    words: HashMap<String, usize>,
    start_words: HashSet<usize>,
    start_total: i32,
    end_words: HashSet<usize>,
    end_total: i32,
    stats: Vec<WordStats>,
}

impl Model {
    pub fn new(cfg: Config) -> Model {
        Model {
            config: cfg,
            words: HashMap::new(),
            start_words: HashSet::new(),
            start_total: 0,
            end_words: HashSet::new(),
            end_total: 0,
            stats: vec![],
        }
    }

    pub fn new_prose() -> Model {
        Self::new(Config::prose())
    }

    pub fn train_buf<R: BufRead>(&mut self, reader: &mut R) -> () {
        let mut bytes: Vec<u8> = Vec::new();

        // iterate over paragraphs
        while let Ok(_) = reader.read_until(self.config.paragraph_delimiter as u8, &mut bytes) {
            if bytes.is_empty() {
                continue;
            }

            // get paragraph as string
            let full_paragraph: String = bytes.iter().map(|b| *b as char).collect();

            // do training LOL
            self.train_paragraph(&full_paragraph)
        }
    }

    pub fn train_paragraph(&mut self, paragraph: &str) -> () {
        // split string into words, filter whitespace
        let words: Vec<&str> = paragraph
            .split(|c| self.config.word_delimiters.contains(&c))
            .filter(|&w| w != "")
            .collect();

        if words.is_empty() {
            return;
        }

        // add first and last word
        let first = words[0];
        let last = words[words.len() - 1];
        self.touch_word(first);
        self.touch_word(last);

        self.count_start_occurrence(first);
        // since we miss this in the loop:
        self.count_occurrence(first);

        self.count_end_occurrence(last);

        // add pair to stats
        for i in 1..words.len() {
            let (w1, w2) = (words[i - 1], words[i]);
            self.touch_word(w2);
            self.count_pair(w1, w2);
            self.count_occurrence(w2);
        }
    }

    pub fn generate_paragraph(&self) -> String {
        let mut words: Vec<&str> = Vec::new();

        // figure out first word
        let start_probs = self.get_start_probabilities();
        let mut cur_idx = pick_random(&start_probs);

        // keep adding words
        loop {
            let stats = &self.stats[cur_idx];
            words.push(&stats.word[..]);

            let end_chance = stats.get_end_probability();
            if random::<f64>() < end_chance {
                break;
            }

            let word_probs = stats.get_next_probabilities();
            cur_idx = pick_random(&word_probs);
        }

        words.join(" ")
    }

    fn get_start_probabilities(&self) -> Vec<f64> {
        self.stats
            .iter()
            .map(|s| s.start_occurrences as f64 / self.start_total as f64)
            .collect()
    }

    fn get_stats<'a>(&'a self, word: &str) -> Option<(usize, &'a WordStats)> {
        let idx = *self.words.get(word)?;
        Some((idx, &self.stats[idx]))
    }

    fn get_stats_mut<'a>(&'a mut self, word: &str) -> Option<(usize, &'a mut WordStats)> {
        let idx = *self.words.get(word)?;
        Some((idx, &mut self.stats[idx]))
    }

    /// Adds a word to all necessary fields and returns its index
    fn touch_word(&mut self, word: &str) -> usize {
        // if we already have it, dont care LOL
        if let Some((idx, _)) = self.get_stats(word) {
            return idx;
        }

        // add to words map
        self.words.insert(word.to_string(), self.words.len());

        // add to size of stats of other word vectors
        for stat in &mut self.stats {
            stat.next_occurrences.push(0)
        }

        // add to stats vec
        self.stats.push(WordStats::new(word, self.stats.len()));

        // return its index
        self.words.len() - 1
    }

    fn count_start_occurrence(&mut self, word: &str) -> () {
        let (idx, stats) = self.get_stats_mut(word).unwrap();
        stats.start_occurrences += 1;
        self.start_total += 1;
        self.start_words.insert(idx);
    }

    fn count_end_occurrence(&mut self, word: &str) -> () {
        let (idx, stats) = self.get_stats_mut(word).unwrap();
        stats.end_occurrences += 1;
        self.end_total += 1;
        self.end_words.insert(idx);
    }

    fn count_pair(&mut self, w1: &str, w2: &str) -> () {
        let (w2_idx, _) = self.get_stats(w2).unwrap();
        let (_, stats) = self.get_stats_mut(w1).unwrap();
        stats.next_occurrences[w2_idx] += 1;
        stats.next_total += 1;
    }

    /// add a single occurrance to a word, used for ending words
    fn count_occurrence(&mut self, word: &str) -> () {
        let (_, stats) = self.get_stats_mut(word).unwrap();
        stats.occurrences += 1;
    }
}
