# Name make

Generate plausible-sounding person names (first names) using Rust

## Why?

This is my first Rust project. I wanted something self-contained and simple, and it let me try out a statistical idea I had some months ago.

## How?

It uses an algorithm a bit like the "Dissociated Press" algorithm used for generating random texts, but with individual letters instead of words. It reads in the sentence data from a file. (The included one is based on US census data, and includes all first names used by at least 1% of the population.) It uses this to build a probability table giving the conditional probability distribution of the next letter given the previous *context_length* (default 2) letters.

Here's some example outputs:

* Jana
* Addi
* Karkleigaleena
* Nicia
* Gre
