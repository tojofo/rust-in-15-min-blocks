fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("Learning rust in {} minutes per day?", 15);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string
    println!("{0} is learning {1} and {2} his dog is interrupting the {1} learning of {0}", "tojofo", "rust", "Pockets");

    // As can named arguments. (This I like)
    println!("{subject} {verb} {object}", object="the lazy dog (sorry Pockets)", subject="the quick red fox", verb="jumps over");

    // Different formatting can be invoked by specifying the format character after a
    // `:`.
    println!("Base 10 () repr:                 {}", 69420);
    println!("Base 2 (b) (binary) repr:        {:b}", 69420);
    println!("Base 8 (o) (octal) repr:         {:o}", 69420);
    println!("Base 16 (x) (hexadecimal) repr:  {:x}", 69420);
    println!("Base 16 (X) (hexadecimal) repr:  {:X}", 69420);

    // You can right-align text with a specified width. This will output
    // "    1". 4 white spaces and a "1", for a total width of 5.
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes. This will output "00001".
    println!("{number:0>5}", number=1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=8);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display
    // println!("This struct `{}` won't print...", Structure(3));

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {pi:.0$}", 3);
}