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

    // https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
}