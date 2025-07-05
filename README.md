# cc_validator

## What is it?

This is a CLI implementation of the Luhn algorithm, used to validate credit card numbers, as well as many other types of
numerical strings. You can check out the details over on [Wikipedia](https://en.wikipedia.org/wiki/Luhn_algorithm).

## Why?

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

## Style

The code style in this project is a little different than what you'd find in, say, my Go. I've aimed to be concise, but
readable...unlike, say, my old Perl days, which was concise, and is Perl ever readable? (Mine was, but I definitely laid
on the 1-liners as much as possible.)

## TODO

Detect CLI params and piped input, and work accordingly. I also want data truncation detection with warning to the user.

