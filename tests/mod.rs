#[cfg(test)]
mod tests {
    use hhkodo::{Frag, Param, parse_frags};

    #[test]
    fn empty_input() {
        assert_eq!(parse_frags(""), Vec::<Frag>::new());
    }

    #[test]
    fn raw_only() {
        let input = "plain text";
        let expected = vec![Frag::Raw(input.to_string())];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn simple_tag() {
        let input = "[b]bold[/b]";
        let expected = vec![Frag::Tag {
            name: "b".to_string(),
            val: None,
            params: vec![],
            subfrags: vec![Frag::Raw("bold".to_string())],
        }];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn nested_tags() {
        let input = "[b]bold [i]and italic[/i][/b]";
        let expected = vec![Frag::Tag {
            name: "b".to_string(),
            val: None,
            params: vec![],
            subfrags: vec![
                Frag::Raw("bold ".to_string()),
                Frag::Tag {
                    name: "i".to_string(),
                    val: None,
                    params: vec![],
                    subfrags: vec![Frag::Raw("and italic".to_string())],
                },
            ],
        }];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn tag_with_value() {
        let input = "[url=https://saikuru.net]link[/url]";
        let expected = vec![Frag::Tag {
            name: "url".to_string(),
            val: Some("https://saikuru.net".to_string()),
            params: vec![],
            subfrags: vec![Frag::Raw("link".to_string())],
        }];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn tag_with_params() {
        let input = "[tag dai=ki rai]content[/tag]";
        let expected = vec![Frag::Tag {
            name: "tag".to_string(),
            val: None,
            params: vec![
                Param::Pair {
                    key: "dai".to_string(),
                    val: "ki".to_string(),
                },
                Param::Free("rai".to_string()),
            ],
            subfrags: vec![Frag::Raw("content".to_string())],
        }];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn unmatched_closing_tag_at_root() {
        let input = "[/b]hello";
        let expected = vec![
            Frag::Raw("[/b]".to_string()),
            Frag::Raw("hello".to_string()),
        ];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn missing_closing_tag() {
        let input = "[b]hello";
        let expected = vec![Frag::Tag {
            name: "b".to_string(),
            val: None,
            params: vec![],
            subfrags: vec![Frag::Raw("hello".to_string())],
        }];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn multiple_missing_closing_tags() {
        let input = "[b]hello[i]world";
        let expected = vec![Frag::Tag {
            name: "b".to_string(),
            val: None,
            params: vec![],
            subfrags: vec![
                Frag::Raw("hello".to_string()),
                Frag::Tag {
                    name: "i".to_string(),
                    val: None,
                    params: vec![],
                    subfrags: vec![Frag::Raw("world".to_string())],
                },
            ],
        }];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn mismatched_closing_within_tag() {
        let input = "[b]hello[/i]world[/b]";
        let expected = vec![Frag::Tag {
            name: "b".to_string(),
            val: None,
            params: vec![],
            subfrags: vec![
                Frag::Raw("hello".to_string()),
                Frag::Raw("[/i]".to_string()),
                Frag::Raw("world".to_string()),
            ],
        }];
        assert_eq!(parse_frags(input), expected);
    }

    #[test]
    fn multiple_segments() {
        let input = "ichi [b]ni[/b] san";
        let expected = vec![
            Frag::Raw("ichi ".to_string()),
            Frag::Tag {
                name: "b".to_string(),
                val: None,
                params: vec![],
                subfrags: vec![Frag::Raw("ni".to_string())],
            },
            Frag::Raw(" san".to_string()),
        ];
        assert_eq!(parse_frags(input), expected);
    }
}
