use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{PathBuf};
use detect_newline_style::*;
use in_place::InPlace;

use crate::error::ConfitError;

pub(crate) fn perform_set(
    file_path: &PathBuf,
    key: &str,
    value: &str,
    separator: &str,
) -> Result<(), ConfitError> {


    // read the config file to gather some details about that file
    let contents = fs::read_to_string(file_path)?;
    let ending = LineEnding::find_or_use_lf(contents);

    // set the value in the file, editing it in place
    let inp = InPlace::new(file_path).open()?;
    let reader = BufReader::new(inp.reader());
    let mut writer = inp.writer();

    let mut found = false;
    
    for line in reader.lines() {
        let line = line?;
        let mut line_written = false;

        // handwritten matching spaces, then key, then spaces, and then separator
        let line = line.trim();
        if line.starts_with(key) {
            let line = line[key.len() ..].trim_start();
            if line.starts_with(separator) {
                // match : replace line with new value
                write!(writer, "{key}{separator}{value}{:}", ending)?;
                found = true;
                line_written = true;
            }
        }
        if !line_written {
            write!(writer, "{}{}", line, ending)?;
        }
    }
    if !found {
        write!(writer, "{key}{separator} {value}{:}", ending)?;
    }
    inp.save()?;


    Ok(())
}


enum InsertState {
    Searching,
    MarkerFound,
    Inserted
}

/// perform the insert action: insert a line within the file, overriding a previous set line if any.
/// The previous line if exists is found using the tag, which is generally a comment line
/// containing a string that is guaranteed (by the user) to be unique.

pub(crate) fn perform_insert(
    file_path: &PathBuf,
    line_to_insert: &str,
    marker: &str,
) -> Result<(), ConfitError> {

    // read the config file to gather some details about that file
    let contents = fs::read_to_string(file_path)?;
    let ending = LineEnding::find_or_use_lf(contents);


    let trimmed_marker = marker.trim();

    // set the value in the file, editing it in place
    let inp = InPlace::new(file_path).open()?;
    let reader = BufReader::new(inp.reader());
    let mut writer = inp.writer();

    let mut state = InsertState::Searching;

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        match state {
            InsertState::Searching => {
                if trimmed_marker == trimmed_line {
                    state = InsertState::MarkerFound;
                }
                write!(writer, "{line}{ending}")?;
            }
            InsertState::MarkerFound => {
                write!(writer, "{line_to_insert}{ending}")?;
                state = InsertState::Inserted;
            }
            InsertState::Inserted => {
                write!(writer, "{line}{ending}")?;
            }
        }
    }
    if matches!(state, InsertState::Searching) {
        write!(writer, "{marker}{ending}")?;
        write!(writer, "{line_to_insert}{ending}")?;
    }

    inp.save()?;

    Ok(())

}

