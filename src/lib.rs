use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_short_classification::ZeroShotClassificationModel;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// create zeor short classification candidates
fn create_db() -> sqlite::Connection {
    let db = sqlite::Connection::open_in_memory().unwrap();
    db.execute("CREATE TABLE zeroshortcandidates (id INTEGER PRIMARY KEY, label TEXT)")
        .unwrap();
    db.execute("INSERT INTO zeroshortcandidates (label) VALUES ('rock')")
        .unwrap();
    db.execute("INSERT INTO zeroshortcandidates (label) VALUES ('pop')")
        .unwrap();
    db.execute("INSERT INTO zeroshortcandidates (label) VALUES ('hip hop')")
        .unwrap();
    db.execute("INSERT INTO zeroshortcandidates (label) VALUES ('country')")
        .unwrap();
    db.execute("INSERT INTO zeroshortcandidates (label) VALUES ('latin')")
        .unwrap();
    db
}

pub fn get_all_zeroshotcandidates() -> Vec<String> {
    let db = create_db();
    let query = "SELECT label FROM zeroshortcandidates";
    let mut candidates: Vec<String> = Vec::new();
    db.iterate(query, |pairs| {
        for &(_column, value) in pairs.iter() {
            if let Some(value) = value {
                candidates.push(value.to_string());
            }
        }
        true
    })
    .unwrap();
    candidates
}

// read lyrics form a file and return a vector of stings
pub fn read_lyrics(file: &str) -> Vec<String> {
    let mut lyrics: Vec<String> = Vec::new();
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        lyrics.push(line);
    }
    lyrics
}

/* use hugging face to classify lyrics using zero short classification
Accepts a vector of strings as lyrics and grabs candidates from the in memory sqlite database
*/
