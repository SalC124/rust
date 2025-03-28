fn main() {
    for count in 1..=1_000_000 {
        let mut out = String::from("");

        fizzbuzz("fizz",    3, count, &mut out);
        fizzbuzz("buzz",    5, count, &mut out);
        fizzbuzz("amogus",  7, count, &mut out);
        fizzbuzz("sus",     2, count, &mut out);

        handle_print(count, &mut out); // printing logic
    }
}

fn fizzbuzz(word:&str, mod_what:u32, count:u32, out:&mut String) -> () {
    if count % mod_what == 0 { out.push_str(word); }
}
fn handle_print(count:u32, out:&mut String) -> () {
    if out == "" { out.push_str(&count.to_string()); }
    println!("{}", out);
}
