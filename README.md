# Randcity
A simple Rust command-line program which creates a randomized city skape.
It was created in about an hour or two for me to learn Rust.

**As this was a one-off project, it is archived.**

<img width="794" alt="Screen Shot 2022-03-23 at 2 48 46 AM" src="https://user-images.githubusercontent.com/76508651/159640232-1088c7d2-53a2-44d5-ba42-9fd4f783e08b.png" style="width:40rem">
<img width="794" alt="Screen Shot 2022-03-23 at 2 50 55 AM" src="https://user-images.githubusercontent.com/76508651/159640300-48ac0e2e-cbc7-47cd-b9fb-82ed646cd83a.png" style="width:40rem">

## Running
In order to run the program you will need
[Rust](https://www.rust-lang.org/learn/get-started) (Cargo) installed.

Then you may clone this repo and cd into the root directory (above the
`src` folder) in your desired terminal and run `cargo run 24 24`. The two
24's can be changed to any number you like, they represent the amount of
buildings to generate and their max height respectively. An amount too
high to fit in the terminal window's width will produce funky,
not-so-good-looking results.

Inside the program you may press enter to get new buildings, or type 'q'
and then press enter to stop the application.

## License
Randcity is licensed under the GPLv3, a free and open source license. For
more information please see the 
[LICENSE file](https://github.com/marcelohdez/randcity/blob/master/LICENSE).
