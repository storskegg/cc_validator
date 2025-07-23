# cc_validator

## What is it?

This is a CLI implementation of the Luhn algorithm, used to validate credit card numbers, as well as many other types of
numerical strings. You can check out the details over on [Wikipedia](https://en.wikipedia.org/wiki/Luhn_algorithm).

### Note

It's worth explicitly stating that the code contained in this repo is intentionally not optimized...not even slightly.
The purpose of how the code is written is to touch many concepts in both C++ and Rust, and is therefore crufty as shit. 

## Why? (C++)

I've long described my C/C++ as "functional, but not amazing," and I want to change that. This is my first project to
hone my C++ skills towards being hire-worthy, and accomplishes this in the following ways:

- First off, this is done in pure C++, unlike most of my C++ that happens to be a C/C++ hybrid (here's looking at my
  embedded platformio code).
- The actual math of the algorithm isn't exciting. In fact, it's pretty simple to follow and implement in any language 
  of choice. However, it's all the surrounding work that lends to being a good foray into improving my C++, including
  the following C++ basics...
  - Multiple class constructors done right
  - Accessor function that exposes the private `char` array for use by `std::cin.getline()`
  - Nuances between `std::getline()` and `std::cin.getline()`. I use the latter, because I chose to use a `char` array
    (C-style string) for my input text buffer to explicitly constrain memory footprint.
  - Learned the caveats of `cin` to grab user input, namely that it delimits on whitespace, and also can be slower than
    `getline()`. (FWIW, most of what I'd be doing in C++ won't involve obtaining user input in this way, but it's still
    good to learn.)
  - C++ style typecasting
  - `strncpy` over `strcpy` to prevent buffer overrun, followed by explicit null termination of the `char` array after
    copy. (Unrelated, I explicitly fill `digits[]` with zeros, because that's how I like to roll. No random data
    surprises.)

## Why? (Rust)

I've also been wanting to cut my teeth on Rust, so I've decided that this would be a good place to start.

## Style (C++)

The code style in this project is a little different than what you'd find in, say, my Go. I've aimed to be concise, but
readable...unlike, say, my old Perl days, which was concise, and is Perl ever readable? (Mine was, but I definitely laid
on the 1-liners as much as possible.)

## TODO (General)

- [ ] Detect CLI params and piped input, and work accordingly. I also want data truncation detection with warning to the
      user.
- [ ] Have a cli flag that stands it up as a microservice, exposing the following APIs:
  - [ ] REST endpoint: accepts plaintext numerical string for validation
  - [ ] REST endpoint: accepts json object containing numerical string for validation
  - [ ] gRPC endpoint: accepts a protocol buffer or similar style marshaling of a numerical string for validation
  - [ ] CGI endpoint: accepts XML arg--I'm just kidding. I'm not going to do this.
- [ ] Unit tests! Inspired by [Manu Sánchez's CppCon presentation](https://www.youtube.com/watch?v=8adO3fN1Igg) on the 
      use of static reflection in unit tests, I figure I'll explore his `tinyrefl` [library](https://github.com/Manu343726/tinyrefl),
      and see wat do. That said, it looks like it was last updated 6 years ago, and listed as WiP, so...we'll see.

