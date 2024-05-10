use std::collections::HashSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::fs::File;
use std::io::Write as _;
use std::time::SystemTime;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = env::args().collect();
    assert_eq!(
        args.len(),
        4,
        "expected 3 arguments: <dictionary> <frequency> <output>"
    );
    let dict_contents = fs::read_to_string(&args[1])?;
    let dict = dict_contents.split("\n").collect::<HashSet<_>>();

    let freq_contents = fs::read_to_string(&args[2])?;

    let freqs = freq_contents
        .split("\n")
        .filter(|line| {
            let word = line.split(" ").next().expect("first thing to be a word");
            word.len() >= 1 && dict.contains(word)
        })
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            let word = split[0];
            let count = split[1]
                .parse::<usize>()
                .expect("the number to be parseable to a `usize`");
            (word, count)
        })
        .collect::<Vec<_>>();

    write_to_file(&freqs, &args[3])?;
    println!("done!");
    Ok(())
}

fn write_to_file(freqs: &Vec<(&str, usize)>, filename: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    let mut buffer = String::new();
    let unix_timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    writeln!(
        &mut buffer,
        "dictionary=duzy:pl,locale=pl,description=Polski słownik od Bartka,date={unix_timestamp},version=1",
    )
    .unwrap();
    let divider = freqs.iter().map(|(_, freq)| freq).sum::<usize>();
    for (word, freq) in freqs.iter() {
        writeln!(
            &mut buffer,
            " word={word},f={f}",
            f = divide_and_scale(*freq, divider)
        )
        .unwrap();
    }
    file.write(buffer.as_bytes())?;
    Ok(())
}
fn divide_and_scale(freq: usize, divider: usize) -> usize {
    let freq = freq as f32;
    let divider = divider as f32;

    let offset = 15.0;
    let max_value = 254.0;
    let logged = freq.log(divider);
    let result = logged * (max_value - offset);

    result.round() as usize + (offset as usize) + 1
}
