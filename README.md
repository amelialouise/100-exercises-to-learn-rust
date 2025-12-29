# Learning Rust, one exercise at a time

This repo is my fork of the original [`100 Exercises to Learn Rust`](https://github.com/mainmatter/100-exercises-to-learn-rust) project with some comments added to this README. 

# Tips and stuff

- **Rust Playground**\
  Useful for testing out functions while working through the exercises. For example, here's
  an embedded code link with a [solution to exercise 07](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=pub+fn+factorial%28n%3A+u32%29+-%3E+u32+%7B%0A++++let+mut+val%3A+u32+%3D+1%3B%0A%0A++++for+i+in+1..%3Dn++%7B%0A++++++++val+*%3D+i%3B%0A++++%7D%0A++++val%0A%7D%0A%0A%0Afn+main%28%29+%7B%0A++++let+n+%3D+5%3B%0A++++println%21%28%22factorial+of+%7B%7D+is+%7B%7D%22%2C+n%2C+factorial%28n%29%29%3B%0A%7D) that you can run. NEAT. 

- **Intro to Rust for R Developers**\
  Day-long workshop offered by Josiah Parry at the 2025 Cascadia R Conference. Gotta say it was a good day. [Materials online here](https://josiahparry.github.io/2025-cascadia-rust-for-r-devs/). 

- **Rustlings**\
  Recommended for learning Rust, though I haven't checked it out yet, tbh. Except to [link it here](https://rustlings.rust-lang.org/). 

> [!NOTE]
> This course has been written by [Mainmatter](https://mainmatter.com/rust-consulting/).\
> It's one of the trainings in [their portfolio of Rust workshops](https://mainmatter.com/services/workshops/rust/).\
> Check out their [landing page](https://mainmatter.com/rust-consulting/) if you're looking for Rust consulting or
> training!

## Getting started

Go to [rust-exercises.com](https://rust-exercises.com) and follow the instructions there
to get started with the course.

## Requirements

- **Rust** (follow instructions [here](https://www.rust-lang.org/tools/install)).\
  If `rustup` is already installed on your system, run `rustup update` (or another appropriate command depending on how
  you installed Rust on your system)
  to make sure you're running on the latest stable version.
- _(Optional but recommended)_ An IDE with Rust autocompletion support.
  We recommend one of the following:
  - [RustRover](https://www.jetbrains.com/rust/);
  - [Visual Studio Code](https://code.visualstudio.com) with
    the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.

## Solutions

You can find the original author's solutions to the exercises in
the [`solutions` branch](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions) of the original repository. My solutions are in the [`alv-solutions` branch](https://github.com/amelialouise/100-exercises-to-learn-rust/tree/alv-solutions) of this fork-generated repo. 

# License

Copyright © 2024- Mainmatter GmbH (https://mainmatter.com), released under the
[Creative Commons Attribution-NonCommercial 4.0 International license](https://creativecommons.org/licenses/by-nc/4.0/).
