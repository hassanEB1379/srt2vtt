use std::env::Args;
use std::error::Error;
use std::fs;
use std::io::Write;
use regex::Regex;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(n) => n,
            None => return Err("No filename passed as argument.")
        };

        Ok(Config {filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let srt_content = fs::read_to_string(&config.filename)?;

    let vtt_content = srt2vtt(srt_content)?;

    let vtt_filename = create_vtt_filename(&config.filename)?;

    let mut file = fs::File::create(vtt_filename)?;

    file.write_all(vtt_content.as_bytes())?;

    Ok(())
}

fn create_vtt_filename(srt_filename: &String) -> Result<String, &'static str> {
    let mut name = srt_filename.get(0..srt_filename.len() - 4).unwrap().to_string();

    name.push_str(".vtt");

    Ok(name)
}

fn srt2vtt(content: String) -> Result<String, Box<dyn Error>> {
    let mut res: String = String::from("WEBVTT\r\n\r\n");

    res.push_str(&content);

    let regex = Regex::new(r"[0-9][0-9]:[0-9][0-9]:[0-9][0-9],\d*")?;

    Ok(
        res.lines().map(|line| {
            if regex.is_match(line) {
                return line.replace(",", ".");
            } 

            line.to_string()
        })
        .collect::<Vec<String>>()
        .join("\r\n")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_srt_to_vtt_correctly() {
        let srt_filename = "samples/long.srt";
        
        let vtt_filename = "samples/long.vtt";

        let srt_content = fs::read_to_string(srt_filename)
        .expect("Can not open srt file");

        let vtt_content_expected = fs::read_to_string(vtt_filename)
        .expect("Can not open vtt file");

        let vtt_content = srt2vtt(srt_content).unwrap();

        let zip_contents = vtt_content.lines().zip(vtt_content_expected.lines());

        let regex = Regex::new(r"[0-9][0-9]:[0-9][0-9]:[0-9][0-9],\d*").unwrap();

        assert_eq!(vtt_content.lines().nth(0).unwrap(), "WEBVTT");

        for (line, line_expected) in zip_contents {
            if regex.is_match(line_expected) {
                assert_eq!(line, line_expected);
            } 
        }
    }

    #[test]
    fn create_correct_filename() {
        let srt_filename = "filename.ABC.srt";

        let vtt_filename_expected = "filename.ABC.vtt";

        let vtt_filename = create_vtt_filename(&srt_filename.to_string()).unwrap();

        assert_eq!(&vtt_filename, vtt_filename_expected);
    }

    #[test]
    fn create_correct_complex_filename() {
        let srt_filename = "filename.srt.123.ABC.srt";

        let vtt_filename_expected = "filename.srt.123.ABC.vtt";

        let vtt_filename = create_vtt_filename(&srt_filename.to_string()).unwrap();

        assert_eq!(&vtt_filename, vtt_filename_expected);
    }
}