use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use unic_langid::LanguageIdentifier;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LangIdTestInputData {
    string: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LangIdTestOutputObject {
    language: Option<String>,
    script: Option<String>,
    region: Option<String>,
    variants: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum LangIdTestOutput {
    String(String),
    Object(LangIdTestOutputObject),
}

#[derive(Serialize, Deserialize)]
struct LangIdTestSet {
    input: LangIdTestInputData,
    output: LangIdTestOutput,
}

fn read_langid_testsets<P: AsRef<Path>>(path: P) -> Result<Vec<LangIdTestSet>, Box<Error>> {
    let file = File::open(path)?;
    let sets = serde_json::from_reader(file)?;
    Ok(sets)
}

fn test_langid_fixtures(path: &str) {
    let tests = read_langid_testsets(path).unwrap();

    for test in tests {
        let s = test.input.string;

        let langid = LanguageIdentifier::try_from(s).expect("Parsing failed.");

        match test.output {
            LangIdTestOutput::Object(o) => {
                let expected = LanguageIdentifier::from_parts(
                    o.language.as_ref(),
                    o.script.as_ref(),
                    o.region.as_ref(),
                    o.variants.as_ref().unwrap_or(&vec![]),
                )
                .expect("Parsing failed.");
                assert_eq!(langid, expected);
            }
            LangIdTestOutput::String(s) => {
                assert_eq!(langid.to_string(), s);
            }
        }
    }
}

#[test]
fn parse() {
    test_langid_fixtures("./tests/fixtures/parsing.json");
}
