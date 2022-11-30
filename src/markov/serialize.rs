use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    paragraph_delimiter: char,
    word_delimiters: HashSet<char>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WordStats {
    word: String,
    occurrences: i32,
    next_occurrences: Vec<i32>,
    next_total: i32,
    start_occurrences: i32,
    end_occurrences: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    config: Config,
    start_words: HashSet<usize>,
    start_total: i32,
    end_words: HashSet<usize>,
    end_total: i32,
    stats: Vec<WordStats>,
}

impl From<Config> for super::Config {
    fn from(cfg: Config) -> Self {
        Self {
            paragraph_delimiter: cfg.paragraph_delimiter,
            word_delimiters: cfg.word_delimiters,
        }
    }
}

impl From<super::Config> for Config {
    fn from(cfg: super::Config) -> Self {
        Self {
            paragraph_delimiter: cfg.paragraph_delimiter,
            word_delimiters: cfg.word_delimiters,
        }
    }
}

impl From<WordStats> for super::WordStats {
    fn from(stat: WordStats) -> Self {
        Self {
            word: stat.word,
            occurrences: stat.occurrences,
            next_occurrences: stat.next_occurrences,
            next_total: stat.next_total,
            start_occurrences: stat.start_occurrences,
            end_occurrences: stat.end_occurrences,
        }
    }
}

impl From<super::WordStats> for WordStats {
    fn from(stat: super::WordStats) -> Self {
        Self {
            word: stat.word,
            occurrences: stat.occurrences,
            next_occurrences: stat.next_occurrences,
            next_total: stat.next_total,
            start_occurrences: stat.start_occurrences,
            end_occurrences: stat.end_occurrences,
        }
    }
}

impl From<Model> for super::Model {
    fn from(data: Model) -> Self {
        super::Model {
            config: super::Config::from(data.config),
            words: data
                .stats
                .iter()
                .enumerate()
                .map(|(idx, stat)| (stat.word.to_owned(), idx))
                .collect(),
            start_words: data.start_words,
            start_total: data.start_total,
            end_words: data.end_words,
            end_total: data.end_total,
            stats: data
                .stats
                .iter()
                .map(|stat| super::WordStats::from(stat.to_owned()))
                .collect(),
        }
    }
}

impl From<super::Model> for Model {
    fn from(data: super::Model) -> Self {
        Model {
            config: Config::from(data.config),
            start_words: data.start_words,
            start_total: data.start_total,
            end_words: data.end_words,
            end_total: data.end_total,
            stats: data
                .stats
                .iter()
                .map(|stat| WordStats::from(stat.to_owned()))
                .collect(),
        }
    }
}
