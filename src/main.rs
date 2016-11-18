extern crate rand;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::hash::Hash;
use std::cmp::Eq;
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;
use rand::Rng;
use rand::distributions::Sample;

const context_length: usize = 2;

#[derive(Debug)]
struct FrequencyTable<T: Hash + Eq> {
    observations: HashMap<T, u32>,
    population: u32
}

impl<T: Hash + Eq> Sample<T> for FrequencyTable<T> {
    fn sample<R: Rng>(&mut self, rng: &mut R) -> T {
        self.rand(rng)
    }
}

impl<T: Hash + Eq> FrequencyTable<T> {
    fn new() -> FrequencyTable<T> {
        FrequencyTable {
            observations: HashMap::new(),
            population: 0
        }
    }

    fn observe(&mut self, key: T) {
        let freq = self.observations.entry(key).or_insert(0);
        *freq += 1;
        self.population += 1;
    }

    fn rand(&self, &mut rng: Rng) {
        let mut index = rng.gen_range(0, population);
        for (key, freq) in self.observations.iter() {
            if index < freq {
                return key
            } else {
                index -= key
            }
        }
    }
}

#[derive(Debug)]
struct Data {
    existing_outputs: HashSet<String>,
    contexts: HashMap<[char;context_length], FrequencyTable<char>>,
}

impl Data {
    fn new() -> Data {
        Data {
            existing_outputs: HashSet::new(),
            contexts: HashMap::new()
        }
    }
}

fn read_census() -> Result<Data, io::Error> {
    let name_file = try!(File::open("census-derived-all-first.txt"));
    let name_file = io::BufReader::new(name_file);
    let mut result = Data::new();
    let mut my_context = ['^'; context_length];
    for line in name_file.lines() {
        let line = try!(line);
        let name = String::from(line.split_whitespace().next().unwrap());

        for character in name.chars() {
            let mut frequencies = result.contexts.entry(my_context).or_insert_with(FrequencyTable::new);
            frequencies.observe(character);

            // now update the context for next time
            for i in 0..context_length - 1 {
                my_context[i] = my_context[i + 1]
            }
            my_context[context_length - 1] = character;
        }
        let mut frequencies = result.contexts.entry(my_context).or_insert_with(FrequencyTable::new);
        frequencies.observe('$');
        result.existing_outputs.insert(name);
    }
    Ok(result)
}

fn generate_name(data: Data) -> String {
    let mut rng = rand::thread_rng();
    let mut my_context = ['^'; context_length];
}

fn main() {
    let data = read_census().expect("Couldn't read name list");
}