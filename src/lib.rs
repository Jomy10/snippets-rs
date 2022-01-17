//! This specific implementation of the snippet parser does not read all strings into memory
//! immediately. Rather, it reads lines into memory as needed.

use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug, Clone)]
pub struct SnippetError<'a> {
    message: &'a str
}

impl<'a> Display for SnippetError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> std::error::Error for SnippetError<'a> {}

/// Parses a snippet file, or creates a new struct representing a snippet file.
#[derive(Debug)]
pub struct SnippetParser<'a> {
    path: Option<&'a str>,
    iter_reader: Option<Lines<BufReader<File>>>,
    snippets: Option<Vec<Snippet>>,
    snippet_index: usize
}

// New
impl<'a> SnippetParser<'a> {
    /// Creates a new struct representing a snippet file.
    pub fn new() -> Self {
        Self { path: None, iter_reader: None, snippets: None, snippet_index: 0 }
    }
    
    /// Reads a snippet file into this struct
    pub fn read(path: &'a str) -> std::io::Result<Self> {
        let file = File::open(path);
        if file.is_err() {
            return Err(file.err().unwrap());
        }
        let reader = BufReader::new(file.unwrap());
        Ok(Self { path: Some(path), iter_reader: Some(reader.lines()), snippets: None, snippet_index: 0 })
    }
    
    /// Creates a new struct representing a snippet file containing the given snippets
    pub fn from_snippets(snips: Vec<Snippet>) -> Self {
        Self { path: None, iter_reader: None, snippets: Some(snips), snippet_index: 0 }
    }
}

impl<'a> SnippetParser<'a> {
    /// Adds a [snippet](crate::Snippet) to this SnippetParser.
    pub fn add_snippet(&mut self, snip: Snippet) {
        if let Some(snippets) = &mut self.snippets {
            snippets.push(snip);
        } else {
            self.snippets = Some(vec![snip]);
        }
    }
    
    /// Gets all snippets from this `SnippetParser`. This means snippets defined by the file at the
    /// given `path` and files added using the `add_snippet` method or `from_snippets` method.
    pub fn get_snippets(&self) -> std::io::Result<Vec<Snippet>> {
        return if self.path.is_some() {
            let file = File::open(self.path.unwrap());
            if file.is_err() {
                return Err(file.err().unwrap());
            }
            let reader = BufReader::new(file.unwrap());
            let copy_of_self = Self {
                path: Some(self.path.unwrap()),
                iter_reader: Some(reader.lines()),
                snippets: self.snippets.clone(),
                snippet_index: 0
            };
            let file_snippets: Vec<Snippet> = copy_of_self.into_iter().map(|snippet| snippet.clone()).collect();
            Ok(file_snippets)
        } else {
            if let Some(snippets) = &self.snippets {
                Ok(snippets.clone())
            } else {
                Ok(Vec::new())
            }
        }
    }
    
    /// Returns the snippet matching the given title.
    ///
    /// # Errors
    /// Returns an err if the file specified by the path could not be read. Ok otherwise. If there
    /// was no path specified, then this will always return Ok.
    ///
    /// # Optional
    /// Return `Some(&Snippet)` if the snippet with the specified title could be found, None otherwise
    pub fn get_snippet(&self, title: &str) -> std::io::Result<Option<Snippet>> {
        let snippets = self.get_snippets();
        if let Ok(snippets) = snippets {
            let found_snippet: Option<&Snippet> = snippets.iter().find_map(|snippet| {
               if &snippet.title == title {
                   Some(snippet)
               } else {
                   None
               }
            });
            return if let Some(found_snippet) = found_snippet {
                Ok(Some(found_snippet.clone()))
            } else {
                Ok(None)
            }
        } else {
            Err(snippets.err().unwrap())
        }
    }
}

impl Iterator for SnippetParser<'_> {
    type Item = Snippet;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_reader.is_some() {
            return if let Some(snippet) = &self.read_next_snippet() {
                // println!("There are more snippets to read from file: {}", snippet);
                Some(snippet.clone())
            } else {
                // read next from snippets
                self.read_next_from_snippets()
            }
        } else {
            // Read next from snippets
            self.read_next_from_snippets()
        }
    }
}

#[cfg(test)]
#[test]
fn read_next_snippet_test() {
    let mut parser = SnippetParser::read("./tests/snippets/snippet_test.snip").unwrap();
    let first_snip = "\
Are we human?
Or are we dancer?\
";
    let second_snip = "\
This is my church.
This is where I heal my hurts.";
    
    let third_snip = "\
Never gonna give you up
Never gonna let you down
Never gonna run around and desert you

Never gonna make you cry
Never gonna say goodbye
Never gonna tell a lie and hurt you
\
";
    let first_read_snip = parser.read_next_snippet().unwrap();
    let second_read_snip = parser.read_next_snippet().unwrap();
    let third_read_snip = parser.read_next_snippet().unwrap();
    
    assert_eq!(first_snip, first_read_snip.s);
    assert_eq!(second_snip, second_read_snip.s);
    assert_eq!(third_snip, third_read_snip.s);
    assert_eq!("snippet1", first_read_snip.title);
    assert_eq!("snippet2", second_read_snip.title);
    assert_eq!("snippet3 with space", third_read_snip.title);
    assert_eq!(None, parser.read_next_snippet());
}

#[cfg(test)]
#[test]
fn read_next_snippet_test_with_adding_snippet() {
    // Should not read added snippets
    
    let mut parser = SnippetParser::read("./tests/snippets/snippet_test.snip").unwrap();
    let first_snip = "\
Are we human?
Or are we dancer?\
";
    let second_snip = "\
This is my church.
This is where I heal my hurts.";
    
    let third_snip = "\
Never gonna give you up
Never gonna let you down
Never gonna run around and desert you

Never gonna make you cry
Never gonna say goodbye
Never gonna tell a lie and hurt you
\
";
    
    let fourth_snip = "\
Are you on the square?
Are you on the hammer?
Are you ready to stand right here right now
Before the devil?
\
";
    let fourth_snippet = Snippet::new("Square Hammer".to_string(), fourth_snip.to_string());
    
    parser.add_snippet(fourth_snippet);
    println!("{:?}", parser);
    let first_read_snip = parser.read_next_snippet().unwrap();
    let second_read_snip = parser.read_next_snippet().unwrap();
    let third_read_snip = parser.read_next_snippet().unwrap();
    
    let fourth_read_snip = parser.read_next_snippet();
    
    assert_eq!(first_snip, first_read_snip.s);
    assert_eq!(second_snip, second_read_snip.s);
    assert_eq!(third_snip, third_read_snip.s);
    assert_eq!("snippet1", first_read_snip.title);
    assert_eq!("snippet2", second_read_snip.title);
    assert_eq!("snippet3 with space", third_read_snip.title);
    // assert_eq!("Square Hammer", fourth_read_snip.title);
    // assert_eq!(fourth_snip, fourth_read_snip.s);
    assert_eq!(None, fourth_read_snip);
    assert_eq!(None, parser.read_next_snippet());
}

// next
impl<'a> SnippetParser<'a> {
    /// Reads the next snippet from the file. This is like a `next` method, but only for
    /// snippets in the file.
    fn read_next_snippet(&mut self) -> Option<Snippet> {
        let mut title: String = String::new();
        let mut started = false;
        let mut lines: Vec<String> = Vec::new();
        // Read lines from file into `lines`
        loop {
            if let Some(_lines) = &mut self.iter_reader {
                let line = _lines.next();
                if let Some(line) = line {
                    if line.is_err() {
                        return None;
                    }
                    if started == false {
                        if line.as_ref().unwrap().contains("--") {
                            // Found title
                            let _title = line.unwrap().replace("--", "");
                            title = _title.trim().to_string();
                            started = true;
                        }
                    } else {
                        // Search for ending
                        if line.as_ref().unwrap().contains("-- end --") {
                            break; // end
                        } else {
                            // Line from string
                            lines.push(line.unwrap());
                        }
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        
        let len_of_lines = lines.len();
        let s: String = lines
            .into_iter()
            .enumerate()
            .flat_map(|(index, line)| {
            let mut line = line;
            if index != len_of_lines - 1 {
                line.push_str("\n");
            }
            line.chars().collect::<Vec<char>>()
        }).collect();
        Some(Snippet::new(title,  s))
    }
    
    /// Reads the next snippet from the `snippets` field.
    fn read_next_from_snippets(&mut self) -> Option<Snippet> {
        if let Some(snippets) = &self.snippets {
            let snippet = snippets.get(self.snippet_index);
            self.snippet_index += 1;
            if let Some(snippet) = snippet {
                Some(snippet.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ToString for SnippetParser<'_> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for snip in self.get_snippets().unwrap() {
            s.push_str(snip.to_string().as_str());
            s.push_str("\n");
        }
        
        s
    }
}

#[derive(Clone, PartialEq, Debug)]
/// Represents a snippet, with a `title` and a `string`
pub struct Snippet {
    title: String,
    s: String
}

// impl Display for Snippet {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "-- {} --\n{}\n-- end --", self.title, self.s)
//     }
// }

impl ToString for Snippet {
    fn to_string(&self) -> String {
        String::from(
            format!("-- {} --\n{}\n-- end --", self.title, self.s)
        )
    }
}

impl Snippet {
    /// Creates a new snippet from a title and a string
    pub fn new(title: String, s: String) -> Snippet {
        Snippet { title, s }
    }
    
    /// Appends a string to the snippet
    pub fn append(&mut self, s: &str) {
        self.s += s;
    }
    
    /// Gets the string from the snippet
    pub fn get_string(&self) -> &str {
        &self.s
    }
}