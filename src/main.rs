#![allow(non_upper_case_globals)]
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
struct FrequencyTable<T: Hash + Eq + Copy> {
    observations: HashMap<T, u32>,
    population: u32
}

impl<T: Hash + Eq + Copy> Sample<T> for FrequencyTable<T> {
    fn sample<R: Rng>(&mut self, rng: &mut R) -> T {
        self.rand(rng)
    }
}

impl<T: Hash + Eq + Copy> FrequencyTable<T> {
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

    fn rand<R: Rng>(&self, rng: &mut R) -> T {
        let mut index = rng.gen_range(0, self.population);
        for (key, freq) in self.observations.iter() {
            if index < *freq {
                return *key
            } else {
                index -= *freq
            }
        }
        panic!()
    }
}

#[cfg(test)]
mod test_frequency_table {
    use super::FrequencyTable;
    use rand;

    #[test]
    fn singleton() {
        let mut rng = rand::weak_rng();
        let mut table = FrequencyTable::new();
        table.observe('a');
        let result = table.rand(&mut rng);
        assert_eq!(result, 'a');
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

    fn observe(&mut self, name: String) {
        let name = name.to_uppercase();
        if self.existing_outputs.contains(&name) {
            return
        }
        let mut my_context = ['^'; context_length];
        for character in name.chars() {
            let mut frequencies = self.contexts.entry(my_context).or_insert_with(FrequencyTable::new);
            frequencies.observe(character);

            // now update the context for next time
            for i in 0..context_length - 1 {
                my_context[i] = my_context[i + 1]
            }
            my_context[context_length - 1] = character;
        }
        let mut frequencies = self.contexts.entry(my_context).or_insert_with(FrequencyTable::new);
        frequencies.observe('$');
        self.existing_outputs.insert(name);
    }
}

#[cfg(test)]
mod test_data {
    use super::context_length;
    use super::Data;

    #[test]
    fn singleton_title_case() {
        let mut data = Data::new();
        data.observe(String::from("Dan"));
        assert!(data.existing_outputs.contains(&String::from("DAN")));
        assert!(data.existing_outputs.len() == 1);
        let start_context = ['^'; context_length];
        let start_table = data.contexts.get(&start_context).unwrap();
        assert_eq!(start_table.population, 1);

    }
}

fn read_census() -> Result<Data, io::Error> {
    let name_file = try!(File::open("census-derived-all-first.txt"));
    let name_file = io::BufReader::new(name_file);
    let mut result = Data::new();
    for line in name_file.lines() {
        let line = try!(line);
        let name = String::from(line.split_whitespace().next().unwrap());
        result.observe(name);
    }
    Ok(result)
}

fn generate_name(data: &Data) -> String {
    let mut rng = rand::weak_rng();
    let mut my_context = ['^'; context_length];
    let mut result = String::new();
    loop {
        let next = data.contexts.get(&my_context).unwrap().rand(&mut rng);
        if next == '$' {
            break
        }
        // if this is the first character
        if my_context[context_length - 1] == '^' {
            result.extend(next.to_uppercase());
        } else {
            result.extend(next.to_lowercase());
        }
        // now update the context for next time
        for i in 0..context_length - 1 {
            my_context[i] = my_context[i + 1]
        }
        my_context[context_length - 1] = next;
    }
    result
}

fn main() {
    let data = read_census().expect("Couldn't read name list");
    loop {
        let generated = generate_name(&data);
        let upper = generated.to_uppercase();
        if !data.existing_outputs.contains(&upper) {
            println!("{}", generated);
            break
        }
    }
}
