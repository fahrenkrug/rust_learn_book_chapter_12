use std::env;
use std::process;
use std::thread;
use std::time::Duration;

use minigrep::Config;
use std::collections::HashMap;
use std::hash::Hash;

mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut cacher = Cacher::new(|x|x);
        let v1 = cacher.value(1);
        let v2 = cacher.value(2);
        let v3 = cacher.value(3);
        let v4 = cacher.value(555);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        assert_eq!(v3, 3);
        assert_eq!(v4, 555);
    }

    #[test]
    fn call_cacher_with_different_types() {
        let mut cacher = Cacher::new(|x| format!("{}", x));
        let v1 = cacher.value(1);
        assert_eq!(v1, "1")
    }

    // #[test]
    // fn call_hashmap_cacher_with_different_values() {
    //     let mut cacher = HashCacher::new(|x| x);
    //     let v1 = cacher.value(1);
    //     let v2 = cacher.value(2);
    //     assert_eq!(v1, 1);
    //     assert_eq!(v2, 2);
    // }
}

fn main() {
    // chapter_12_main();
    chapter_13_main();
}

fn chapter_12_main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn chapter_13_main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number= 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // ---------------------------------------------------------------------------------------------
    // First version
    /*if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today. Remember to stay hidrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    } */
    // ---------------------------------------------------------------------------------------------
    // Second version: Storing the expensive result in a variable
    /* let expensive_result = simulated_expensive_calculation(intensity);
    // This is an improvement to version 1, but there is one case where this expensive method
    // has not to be called. Which makes it 2 seconds more slowly than needed here.
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today. Remember to stay hidrated");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }*/
    // ---------------------------------------------------------------------------------------------
    // Version 3: we use a closure
    // This closure replaces the simulated_expensive_calculation-function for solution 3.
    // Variante: Types could be added to a closure like so:
    // let expensive_closure = |num: u32| -> u32 {
    // But since a closed is only used in a small scope and not exposed to outer usage the compiler
    // is able to infer the type on it's own
    // We even have a 4th version of this using the Cacher (memoization) below.
    /*let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today. Remember to stay hidrated");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }*/
    // ---------------------------------------------------------------------------------------------
    //Version 4
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today. Remember to stay hidrated");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T, U, V>
where T: Fn(U) -> V
{
    calculation: T,
    values: HashMap<U, V>,
}

// Another Bonus
// The second problem with the current Cacher implementation is that it only accepts closures
// that take one parameter of type u32 and return a u32. We might want to cache the results of
// closures that take a string slice and return usize values, for example. To fix this issue, try
// introducing more generic parameters to increase the flexibility of the Cacher functionality.
impl<T, U, V: Clone> Cacher<T, U, V>
where T: Fn(U) -> V, U: Hash, U: Eq, U: Clone, V: Clone {
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg.clone());
                self.values.insert(arg, v.clone());
                v.clone()
            }
        }
    }
}

// Bonus quest: The problem is that the first time we called c.value with 1, the Cacher
// instance saved Some(1) in self.value. Thereafter, no matter what we pass into the value method,
// it will always return 1.
//
// Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map
// will be the arg values that are passed in, and the values of the hash map will be the result
// of calling the closure on that key
struct HashCacher<T>
where T: Fn(u32) -> u32 {
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> HashCacher<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> HashCacher<T> {
        HashCacher {
            calculation,
            values: HashMap::new()
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

// This function would cause the compilation to fail, because the closure is used with two
// different types here.
/* fn bad_closure_type_example() {
    let do_return_the_param = |x| x;
    let s = do_return_the_param(String::from("Moin"));
    let n = do_return_the_param(5);
}*/

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
