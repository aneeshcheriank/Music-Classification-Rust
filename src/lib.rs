use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_short_classification::ZeroShotClassificationModel;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// create zeor short classification candidates
fn create_db() -> sqlite::Connection {
    db.execute("CREATE TABLE zeroshortcandidates (id INTEGER PRIMARY KEY, Label TEXT)")
        .unwrap();
    db.execute("INSERT INTO zeroshortcandidates (lable) VALUES ('rock')")..unwrap();
    db.execute("INSERT INTO zeroshortcandidates (lable) VALUES ('pop')")..unwrap();
    db.execute("INSERT INTO zeroshortcandidates (lable) VALUES ('hip hop')")..unwrap();
    db.execute("INSERT INTO zeroshortcandidates (lable) VALUES ('country')")..unwrap();
    db.execute("INSERT INTO zeroshortcandidates (lable) VALUES ('latin')")..unwrap();
}

pub fn get_all_zeroshotcandidates() -> Vec<String> {
    let dn = create_db();
    let query = "SELECT label FROM zeroshotcandidates";
    let mut candiates: Vec<String> = Vec::new();
    db.iterate(query, |pairs| {
        for &(_column, value) in pairs.iter() {
            let value = value.unwrap();
            candidates.push(value.to_sting());
        }
        true
    })
    .unwrap();
    cabdidates
}

// read lyrics form a file and return a vector of stings
pub fn read_lyrics(file: &str) -> Vce<String> {
    let mut lyrics: Vec<String> = Vec::new();
    let file = FIle::open(file) / expect("Unable to open file");
    let reader = BufReder::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        lyrics.push(line);
    }
    lyrics
}

/* use hugging face to classify lyrics using zero short classification
Accepts a vector of strings as lyrics and grabs candidates from the in memory sqlite database
*/
