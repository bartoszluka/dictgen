use clap::Parser;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "lang_words.txt")]
    spellchecking: Option<String>,

    #[arg(short, long, value_name = "lang.txt")]
    frequency: String,

    #[arg(short, long, value_name = "lang.dict")]
    output: String,

    #[arg(long)]
    header: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let cli_args = Args::parse();

    let freq_contents = fs::read_to_string(cli_args.frequency)?;

    let spellchecking_dict_contents;
    let dict_opt = if let Some(dict_file) = cli_args.spellchecking {
        spellchecking_dict_contents = fs::read_to_string(dict_file)?;
        let spellchecking_dict = spellchecking_dict_contents
            .split('\n')
            .collect::<HashSet<_>>();
        Some(spellchecking_dict)
    } else {
        None
    };

    let freqs = parse_freqs(&freq_contents, dict_opt);
    println!("frequencies generated");

    write_to_file(&freqs, "intermediate.txt", &cli_args.header)?;
    println!("running java dicttool to generate binary dictionary");
    if cli_args.header.is_some() {
        build_actual_dict("intermediate.txt", &cli_args.output)?;
    }
    Ok(())
}

fn parse_freqs<'a>(
    freq_contents: &'a str,
    dict_opt: Option<HashSet<&str>>,
) -> Vec<(&'a str, usize)> {
    println!("spell checking frequency file");
    let freqs = freq_contents
        .split('\n')
        .filter(|line| {
            let word = line.split(' ').next().expect("first thing to be a word");
            if let Some(dict) = &dict_opt {
                !word.is_empty() && dict.contains(word)
            } else {
                !word.is_empty()
            }
        })
        .map(|line| {
            let mut split = line.split(' ');
            let word = split.next().expect("first thing to be a word");
            let count = split
                .next()
                .expect("line needs to be in format <word> <count>")
                .parse::<usize>()
                .expect("the number to be parseable to a `usize`");
            (word, count)
        })
        .collect::<Vec<_>>();
    freqs
}

fn write_to_file(
    freqs: &[(&str, usize)],
    filename: &str,
    header: &Option<String>,
) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    let mut buffer = String::new();
    if let Some(h) = header {
        writeln!(&mut buffer, "{h}").unwrap();
    }
    let divider = freqs.iter().map(|(_, freq)| freq).sum::<usize>();
    for (word, freq) in freqs.iter() {
        writeln!(
            &mut buffer,
            " word={word},f={f}",
            f = divide_and_scale(*freq, divider)
        )
        .unwrap();
    }

    file.write_all(buffer.as_bytes())?;
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

fn build_actual_dict(intermediate_dict: &str, out_dict: &str) -> io::Result<()> {
    let mut command = Command::new("java");
    command.args([
        "-jar",
        "dicttool_aosp.jar",
        "makedict",
        "-s",
        intermediate_dict,
        "-d",
        out_dict,
    ]);
    let child = command.spawn()?;
    let output = child.wait_with_output()?;
    if output.status.success() {
        println!("dictionary `{out_dict}` created");
        Ok(())
    } else {
        eprintln!("failed to create `{out_dict}`");
        Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to create dictionary",
        ))
    }
}
