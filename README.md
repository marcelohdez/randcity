# Randcity
A simple command line program which creates a randomized city skape.
I coded it in about an hour or two while trying to learn some Rust.

## Running
In order to run either program you will need [Rust](https://www.rust-lang.org/learn/get-started) (Cargo)
installed.

Then you may clone this repo and cd into the root 
directory (above the ```src``` folder) in your desired terminal and run ```cargo run 24 24```.
The two 24's can be changed to any number you like, they represent the amount of buildings to generate
and their max height respectively. An amount too high to fit in the terminal window's width will produce
funky, not-so-good-looking results.

Inside the program you may press enter to get new buildings, or type 'q' and then press enter to stop
the application.

## License
Randcity is licensed under the GPLv3, a free and open source license. See the 
[LICENSE file](https://github.com/marcelohdez/randcity/blob/master/LICENSE).