use snippet_rs::*;

#[test]
fn get_string() {
    let snippet = Snippet::new("Title".to_string(), "This is my church\nThis is where I heal my hurt.".to_string());
    assert_eq!("This is my church\nThis is where I heal my hurt.", snippet.get_string());
}

#[test]
fn iter() {
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
    assert_eq!(Some(Snippet::new("snippet1".to_string(), first_snip.to_string())), parser.next());
    assert_eq!(Some(Snippet::new("snippet2".to_string(), second_snip.to_string())), parser.next());
    assert_eq!(Some(Snippet::new("snippet3 with space".to_string(), third_snip.to_string())), parser.next());
    assert_eq!(None, parser.next());
}

#[test]
fn iter_with_adding_snippet() {
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
    
    let fourth_snippet = Snippet::new("My Snippet".to_string(), "Bury all your secrets in my skin\nCome away with innocence and leave me with my sin\nThe air around me still feels like a cage".to_string());
    parser.add_snippet(fourth_snippet.clone());
    
    assert_eq!(Some(Snippet::new("snippet1".to_string(), first_snip.to_string())), parser.next());
    assert_eq!(Some(Snippet::new("snippet2".to_string(), second_snip.to_string())), parser.next());
    assert_eq!(Some(Snippet::new("snippet3 with space".to_string(), third_snip.to_string())), parser.next());
    assert_eq!(Some(fourth_snippet), parser.next());
    assert_eq!(None, parser.next());
}

#[test]
fn iter_only_manual_adding() {
    let snippet = Snippet::new("Ibiza".to_string(), "What's he fucking doing?\nIbiza".to_string());
    let snippet2 = Snippet::new("The day is my enemy".to_string(), "The day is my enemy\nthe night is my friend".to_string());
    let parser = SnippetParser::from_snippets(vec![snippet.clone(), snippet2.clone()]);
    
    assert_eq!(vec![snippet, snippet2], parser.into_iter().map(|snip| snip).collect::<Vec<Snippet>>());
}

#[test]
fn get_snippets() {
    let parser = SnippetParser::read("./tests/snippets/snippet_test.snip").unwrap();
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
    let first_snip = Snippet::new("snippet1".to_string(), first_snip.to_string());
    let second_snip = Snippet::new("snippet2".to_string(), second_snip.to_string());
    let third_snip = Snippet::new("snippet3 with space".to_string(), third_snip.to_string());
    
    let vec: Vec<Snippet> = vec![first_snip, second_snip, third_snip];
    
    assert_eq!(vec, parser.get_snippets().unwrap());
}

#[test]
fn get_snippets_with_adding_snippets() {
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
    let first_snip = Snippet::new("snippet1".to_string(), first_snip.to_string());
    let second_snip = Snippet::new("snippet2".to_string(), second_snip.to_string());
    let third_snip = Snippet::new("snippet3 with space".to_string(), third_snip.to_string());
    let fourth_snip = Snippet::new("snippet4 :)".to_string(), "Dancing in September".to_string());
    
    let vec: Vec<Snippet> = vec![first_snip, second_snip, third_snip, fourth_snip.clone()];
    parser.add_snippet(fourth_snip);
    
    let snippets = parser.get_snippets().unwrap();
    
    assert_eq!(vec, snippets);
}

#[test]
fn get_snippets_only_manual_adding() {
    let mut parser = SnippetParser::new();
    let snippet = Snippet::new("Blackstar".to_string(), "I'm not a pornstar. I'm a blackstar".to_string());
    let snippet2 = Snippet::new("Rebel Rebel".to_string(), "Rebel Rebel, you've torn your dress\nRebel Rebel, your face is a mess.".to_string());
    
    parser.add_snippet(snippet.clone());
    parser.add_snippet(snippet2.clone());
    
    assert_eq!(vec![snippet, snippet2], parser.get_snippets().unwrap())
}

#[test]
fn get_snippet() {
    let mut parser = SnippetParser::read("./tests/snippets/snippet_test.snip").unwrap();
    let first_snip = "\
Are we human?
Or are we dancer?\
";
    let first_snip = Snippet::new("snippet1".to_string(), first_snip.to_string());
    let man_snippet = Snippet::new("The day is my enemy".to_string(), "The day is my enemy\nthe night is my friend".to_string());
    parser.add_snippet(man_snippet.clone());
    
    assert_eq!(first_snip, parser.get_snippet("snippet1").unwrap().unwrap());
    assert_eq!(man_snippet, parser.get_snippet("The day is my enemy").unwrap().unwrap());
}

#[test]
fn to_string() {
    let mut parser = SnippetParser::read("./tests/snippets/snippet_test.snip").unwrap();
    let extra_snip = Snippet::new("Uprising".to_string(),
        "\
        Rise up and take the power back\n\
        It's time the fat cats had a heart attack".to_string()
    );
    parser.add_snippet(extra_snip);
    
    let file_contents = "\
    -- snippet1 --
Are we human?
Or are we dancer?
-- end --
-- snippet2 --
This is my church.
This is where I heal my hurts.
-- end --
-- snippet3 with space --
Never gonna give you up
Never gonna let you down
Never gonna run around and desert you

Never gonna make you cry
Never gonna say goodbye
Never gonna tell a lie and hurt you

-- end --
-- Uprising --
Rise up and take the power back
It's time the fat cats had a heart attack
-- end --
";
    assert_eq!(file_contents.to_string(), parser.to_string());
}