use names::{ADJECTIVES, Generator, NOUNS, Name};

#[test]
fn plain_names_use_the_supplied_dictionaries() {
    let mut generator = Generator::new(&["imaginary"], &["roll"], Name::Plain);
    assert!(
        generator
            .by_ref()
            .take(100)
            .all(|name| name == "imaginary-roll")
    );
}

#[test]
fn numbered_names_have_a_four_digit_suffix_in_range() {
    let mut generator = Generator::new(&["imaginary"], &["roll"], Name::Numbered);
    generator.by_ref().take(100).for_each(|name| {
        let suffix = name.strip_prefix("imaginary-roll-").unwrap();
        assert_eq!(suffix.len(), 4);
        let number: u16 = suffix.parse().unwrap();
        assert!((1..10000).contains(&number));
    });
}

#[test]
fn default_names_use_the_bundled_dictionaries() {
    Generator::default().take(100).for_each(|name| {
        assert!(ADJECTIVES.iter().any(|adjective| {
            name.strip_prefix(adjective)
                .and_then(|rest| rest.strip_prefix('-'))
                .is_some_and(|noun| NOUNS.contains(&noun))
        }));
    });
}
