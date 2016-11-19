extern crate rand;
#[macro_use]
extern crate clap;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::hash::Hash;
use std::cmp::Eq;
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;
use rand::Rng;
use rand::distributions::Sample;

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
    context_length: usize,
    existing_outputs: HashSet<String>,
    contexts: HashMap<String, FrequencyTable<char>>,
}

impl Data {
    fn new(context_length: usize) -> Data {
        Data {
            context_length: context_length,
            existing_outputs: HashSet::new(),
            contexts: HashMap::new()
        }
    }

    fn initial_context(&self) -> String {
        let mut result = String::with_capacity(self.context_length);
        for _ in 0..self.context_length {
            result.push('^')
        }
        result
    }

    fn observe(&mut self, name: String) {
        let name = name.to_uppercase();
        if self.existing_outputs.contains(&name) {
            return
        }
        let mut my_context = self.initial_context();
        for character in name.chars() {
            let mut frequencies = self.contexts.entry(my_context.clone()).or_insert_with(FrequencyTable::new);
            frequencies.observe(character);

            // now update the context for next time
            my_context = my_context.chars().skip(1).collect();
            my_context.push(character);
        }
        let mut frequencies = self.contexts.entry(my_context).or_insert_with(FrequencyTable::new);
        frequencies.observe('$');
        self.existing_outputs.insert(name);
    }
}

#[cfg(test)]
mod test_data {
    use super::Data;

    #[test]
    fn singleton_title_case() {
        let context_length = 2;
        let mut data = Data::new(context_length);
        data.observe(String::from("Dan"));
        assert!(data.existing_outputs.contains(&String::from("DAN")));
        assert!(data.existing_outputs.len() == 1);
        let start_context = data.initial_context();
        let start_table = data.contexts.get(&start_context).unwrap();
        assert_eq!(start_table.population, 1);
    }
}

fn read_census(context_length: usize) -> Result<Data, io::Error> {
    let name_file = try!(File::open("census-derived-all-first.txt"));
    let name_file = io::BufReader::new(name_file);
    let mut result = Data::new(context_length);
    for line in name_file.lines() {
        let line = try!(line);
        let name = String::from(line.split_whitespace().next().unwrap());
        result.observe(name);
    }
    Ok(result)
}

fn generate_name(data: &Data) -> String {
    let mut rng = rand::weak_rng();
    let mut my_context = data.initial_context();
    let mut result = String::new();
    let mut first = true;
    loop {
        let next = data.contexts.get(&my_context).unwrap().rand(&mut rng);
        if next == '$' {
            break
        }
        if first {
            result.extend(next.to_uppercase());
            first = false
        } else {
            result.extend(next.to_lowercase());
        }
        // now update the context for next time
        my_context = my_context.chars().skip(1).collect();
        my_context.push(next);
    }
    result
}

fn main() {
    let matches = clap_app!(myapp =>
        (about: "Generate plausible-sounding names")
        (@arg context_length: -c --contextlength +takes_value "Sets the length of context to use")
    ).get_matches();
    let context_length = value_t!(matches.value_of("context_length"), usize).unwrap_or(2);
    let data = read_census(context_length).expect("Couldn't read name list");
    loop {
        let generated = generate_name(&data);
        let upper = generated.to_uppercase();
        if !data.existing_outputs.contains(&upper) {
            println!("{}", generated);
            break
        }
    }
}
