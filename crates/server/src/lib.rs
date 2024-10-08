use core::{error, panic};
use std::fmt::Display;

pub struct HttpRequest {
    method: HttpMethodEnum,
    target: Uri,
}

pub enum HttpMethodEnum {
    GET,
    HEAD,
    POST,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpSchemeEnum {
    Unknown,
    HTTP,
    HTTPS,
}

#[derive(Debug)]
pub struct Uri {
    scheme: HttpSchemeEnum,
    host: String,
    port: u16,
    query: Option<String>,
}

impl Uri {
    pub fn scheme(&self) -> HttpSchemeEnum {
        self.scheme
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn query(&self) -> Option<&str> {
        match &self.query {
            Some(s) => Some(s),
            None => None,
        }
    }

    /// Fixed!
    pub fn parse_tokens(tokenizer: &mut Tokenizer) -> Result<Self, InvalidToken> {
        let mut scheme = HttpSchemeEnum::Unknown;
        let mut host = String::new();
        let mut port = 0 as u16;
        let mut query = None;

        let tokens = match tokenizer.tokens() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        for token in tokens {
            let token_str = &tokenizer.buffer[token.location().start()..token.location().end()];

            match token.tag() {
                Tag::Scheme => {
                    // Matches scheme and sets the default ports in case a
                    // Tag::Port Token is not generated from the raw URI.
                    match token_str {
                        "http" => {
                            scheme = HttpSchemeEnum::HTTP;
                            port = 80;
                        }
                        "https" => {
                            scheme = HttpSchemeEnum::HTTPS;
                            port = 443;
                        }
                        &_ => panic!("Invalid token passed somehow."),
                    };
                }
                Tag::Authority => {
                    host = String::from(token_str);
                }
                Tag::Port => {
                    if token_str.len() > 0 {
                        port = String::from(token_str).parse::<u16>().ok().unwrap();
                        continue;
                    }
                    port = match scheme {
                        HttpSchemeEnum::HTTP => 80 as u16,
                        HttpSchemeEnum::HTTPS => 443 as u16,
                        HttpSchemeEnum::Unknown => todo!(),
                    }
                }
                Tag::Query => {
                    query = Some(String::from(token_str));
                }
                _ => continue,
            };
        }

        Ok(Self {
            scheme,
            host,
            port,
            query,
        })
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " [{:?}]  [{:?}] [{:?}]",
            self.scheme, self.host, self.port
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    tag: Tag,
    location: Location,
}

impl Token {
    pub fn new(tag: Tag, location: Location) -> Self {
        Self { tag, location }
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn update_tag_and_end(&mut self, tag: Tag, end_idx: usize) {
        self.tag = tag;
        self.location.end_idx = end_idx;
    }
    pub fn set_end(&mut self, end: usize) {
        self.location.end_idx = end;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tag {
    Start,
    End,

    Scheme,
    UserInfo,
    Authority,
    Port,
    Path,
    Query,
    Fragment,

    Invalid,
}

/// End exclusive
#[derive(Clone, Copy, Debug)]
pub struct Location {
    start_idx: usize,
    end_idx: usize,
}

impl Location {
    pub fn new(start_idx: usize, end_idx: usize) -> Self {
        Self { start_idx, end_idx }
    }

    pub fn start(&self) -> usize {
        self.start_idx
    }

    pub fn end(&self) -> usize {
        self.end_idx
    }
}

#[derive(Clone)]
pub struct Tokenizer {
    buffer: String,
    index: usize,
    processing_tag: Tag,
}

// #[derive(PartialEq, Debug, Clone, Copy)]
// pub enum Tag {
//     // Only needs to handle single lines.
//     // Relevant characters:
//     // :// : / ?
//     // URI = scheme ":" ["//" [userinfo "@"] host [":"]] path ["?" query] ["#" fragment]
//     Start,
//     Scheme,
//     Authority,
//     UserInfo,
//     Port,
//     Path,
//     Query,
//     Fragment,

//     EndOfURI,
// }

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LocalState {
    InToken,
    EndOfToken,
}

// A token is comprised of valid characters, and a delimiter. There is no
// consistant pre- or proceeding of delimiters, and so the state should
// be accurately updated when the token is completed. There is a global
// state and a local state. The global state tracks the section, and the
// local state will throw an error if it encounters an invalid character.
// The global state can be a part of a URI, EndOfURI, or Invalid (based upon
// the local state). The local state can be Valid, Invalid, or EndOfToken.
// EndOfToken signals to close the loop. It should also update the global state
// to the valid next section before closing.

impl Tokenizer {
    pub fn new(buffer: String) -> Self {
        Self {
            buffer,
            index: 0,
            processing_tag: Tag::Start,
        }
    }

    pub fn current_tag(&self) -> Tag {
        self.processing_tag
    }

    pub fn tokens(&mut self) -> Result<Vec<Token>, InvalidToken> {
        let mut tokens: Vec<Token> = vec![];
        self.index = 0;
        self.processing_tag = Tag::Start;

        loop {
            match self.next() {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            };

            // tokens.push(next);
            if self.processing_tag == Tag::End {
                break;
            }
        }

        Ok(tokens)
    }

    pub fn next(&mut self) -> Result<Token, InvalidToken> {
        let mut result = Token::new(
            self.processing_tag,
            Location::new(self.index, self.index + 1),
        );
        let mut local_state = LocalState::InToken;

        let buffer_as_chars: Vec<char> = self.buffer.chars().map(|c| c).collect();

        while local_state != LocalState::EndOfToken {
            if self.index == buffer_as_chars.len() {
                result.set_end(self.index);
                self.processing_tag = Tag::End;

                return Ok(result);
            }

            match self.processing_tag {
                Tag::Start => match buffer_as_chars[self.index] {
                    'h' => {
                        result.tag = Tag::Scheme;
                        result.location.start_idx = self.index;
                        self.processing_tag = Tag::Scheme;
                        self.index += 1;
                    }
                    _ => {
                        return Err(InvalidToken {});
                    }
                },
                Tag::Scheme => match buffer_as_chars[self.index] {
                    // Valid characters; continue
                    't' | 'p' | 's' => self.index += 1,
                    // Post-delimiter; set result tag to current section,
                    // result end location to current index, local state
                    // to EndOfToken, Tag to the next section
                    // (Authority)
                    ':' => {
                        result.set_end(self.index);
                        local_state = LocalState::EndOfToken;
                        self.processing_tag = Tag::Authority;
                        self.index += 3; // skip ://
                    }
                    // Non valid characters; set token to invalid, token
                    // location, local state to invalid, and global state to
                    // invalid.
                    _ => {
                        return Err(InvalidToken {});
                    }
                },
                Tag::Authority => match buffer_as_chars[self.index] {
                    // Post-delimiter; set tag to previous section, set
                    // location, set state to end of token. global state is
                    // already in the correct state. increment index.
                    '@' => {
                        result.update_tag_and_end(Tag::UserInfo, self.index);
                        local_state = LocalState::EndOfToken;
                        // self.state = Tag::Authority; // alread in this state
                        self.index += 1
                    }
                    ':' => {
                        result.set_end(self.index);
                        local_state = LocalState::EndOfToken;
                        self.processing_tag = Tag::Port;
                        self.index += 1;
                    }
                    '/' => {
                        result.set_end(self.index);
                        local_state = LocalState::EndOfToken;
                        self.processing_tag = Tag::Path;
                        self.index += 1;
                    }
                    '['
                    | ']'
                    | '_'
                    | 'A'..='Z'
                    | 'a'..='z'
                    | '0'..='9'
                    | '-'
                    | '.'
                    | '~'
                    | '!'
                    | '#'
                    | '$'
                    | '&'
                    | '\''
                    | '('
                    | ')'
                    | '*'
                    | '+'
                    | ','
                    | ';'
                    | '='
                    | '?' => self.index += 1,
                    _ => return Err(InvalidToken {}),
                },
                Tag::Port => match buffer_as_chars[self.index] {
                    // Ports are only valid unsigned ints.
                    '0'..='9' => self.index += 1,
                    // Pre-delimiter. '/' begins the path portion. set this
                    // token as port and prepare for path.
                    '/' => {
                        result.set_end(self.index);
                        local_state = LocalState::EndOfToken;
                        self.processing_tag = Tag::Path;
                        self.index += 1;
                    }
                    _ => {
                        return Err(InvalidToken {});
                    }
                },
                Tag::Path => match buffer_as_chars[self.index] {
                    // Pre-delimiter. send path and prepare for query.
                    '?' => {
                        result.set_end(self.index);
                        local_state = LocalState::EndOfToken;
                        self.processing_tag = Tag::Query;
                        self.index += 1;
                    }
                    // If no '?', the check for fragment. Pre-delimiter, send
                    // path and prep fragment.
                    '#' => {
                        result.set_end(self.index);
                        local_state = LocalState::EndOfToken;
                        self.processing_tag = Tag::Fragment;
                        self.index += 1;
                    }
                    // Everything else is valid until I feel like getting the
                    // ASCII in here.
                    _ => self.index += 1,
                },
                Tag::Query => match buffer_as_chars[self.index] {
                    // If fragment, send query, prep frag.
                    '#' => {
                        result.set_end(self.index);
                        local_state = LocalState::EndOfToken;
                        self.processing_tag = Tag::Fragment;
                        self.index += 1;
                    }
                    // Same thing; when I feel like it.
                    _ => self.index += 1,
                },

                // Again, when I have energy.
                Tag::Fragment => match buffer_as_chars[self.index] {
                    _ => self.index += 1,
                },
                Tag::UserInfo => todo!(),
                Tag::End => todo!(),
                Tag::Invalid => todo!(),
            }
        }
        Ok(result)
    }
}

impl Display for Tokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut clone: Tokenizer = self.clone();
        let mut out = String::new();
        let tokens = match clone.tokens() {
            Ok(t) => t,
            Err(_) => panic!("Cannot tokenize for Display"),
        };
        for token in tokens {
            out.push_str(
                format!(
                    "Tag: {:?}; Location: [{:?}, {:?}]\n",
                    token.tag(),
                    token.location().start(),
                    token.location().end()
                )
                .as_str(),
            );
        }
        write!(f, "{}", out)
    }
}

#[derive(PartialEq, Debug)]
pub struct InvalidToken {}

#[test]
fn test_parse_easy() {
    let test_uri = String::from("https://telemakos.io");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    let parsed_uri = Uri::parse_tokens(&mut tokenizer).ok().unwrap();
    assert_eq!(parsed_uri.scheme(), HttpSchemeEnum::HTTPS);
    assert_eq!(parsed_uri.host(), "telemakos.io");
    assert_eq!(parsed_uri.port(), 443);
    assert_eq!(parsed_uri.query(), None);
}

#[test]
fn test_parse_with_port() {
    let test_uri = String::from("https://telemakos.io:600");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    let parsed_uri = Uri::parse_tokens(&mut tokenizer).ok().unwrap();
    assert_eq!(parsed_uri.scheme(), HttpSchemeEnum::HTTPS);
    assert_eq!(parsed_uri.host(), "telemakos.io");
    assert_eq!(parsed_uri.port(), 600);
    assert_eq!(parsed_uri.query(), None);
}

#[test]
fn test_parse_with_port_and_query() {
    let test_uri = String::from("https://telemakos.io:600/?test_query");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    let parsed_uri = Uri::parse_tokens(&mut tokenizer).ok().unwrap();
    assert_eq!(parsed_uri.scheme(), HttpSchemeEnum::HTTPS);
    assert_eq!(parsed_uri.host(), "telemakos.io");
    assert_eq!(parsed_uri.port(), 600);
    assert_eq!(parsed_uri.query(), Some("test_query"));
}

#[test]
fn test_parse_with_port_and_query_and_fragment() {
    let test_uri = String::from("https://telemakos.io:600/?test_query#bruh-fragment");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    let parsed_uri = Uri::parse_tokens(&mut tokenizer).ok().unwrap();
    assert_eq!(parsed_uri.scheme(), HttpSchemeEnum::HTTPS);
    assert_eq!(parsed_uri.host(), "telemakos.io");
    assert_eq!(parsed_uri.port(), 600);
    assert_eq!(parsed_uri.query(), Some("test_query"));
}

#[test]
fn test_tokenizer() {
    let test_uri = String::from("https://telemakos.io");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    let tokens = tokenizer.tokens().ok().unwrap();
    for token in tokens {
        let token_str = &tokenizer.buffer[token.location().start()..token.location().end()];
        match token.tag() {
            Tag::Scheme => assert_eq!(token_str, "https"),
            Tag::Authority => assert_eq!(token_str, "telemakos.io"),
            _ => panic!("failed on tag {:?}", token.tag()),
        }
    }
}

#[test]
fn test_tokenizer_port() {
    let test_uri = String::from("https://telemakos.io:90");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    let tokens = tokenizer.tokens().ok().unwrap();
    for token in tokens {
        let token_str = &tokenizer.buffer[token.location().start()..token.location().end()];
        match token.tag() {
            Tag::Scheme => assert_eq!(token_str, "https"),
            Tag::Authority => assert_eq!(token_str, "telemakos.io"),
            Tag::Port => assert_eq!(token_str, "90"),
            _ => panic!("failed on tag {:?}", token.tag()),
        }
    }
}

#[test]
fn test_tokenizer_port_query() {
    let test_uri = String::from("https://telemakos.io:90/?kendric_tpabf");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    let tokens = tokenizer.tokens().ok().unwrap();
    for token in tokens {
        let token_str = &tokenizer.buffer[token.location().start()..token.location().end()];
        match token.tag() {
            Tag::Scheme => assert_eq!(token_str, "https"),
            Tag::Authority => assert_eq!(token_str, "telemakos.io"),
            Tag::Port => assert_eq!(token_str, "90"),
            Tag::Query => assert_eq!(token_str, "kendric_tpabf"),
            Tag::Path => assert_eq!(token_str, ""),
            _ => panic!("failed on tag {:?}", token.tag()),
        }
    }
}

#[test]
fn test_tokenizer_port_query_fragment() {
    let test_uri = String::from("https://telemakos.io:90/?kendric_tpabf#bruh!");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    let tokens = tokenizer.tokens().ok().unwrap();
    for token in tokens {
        let token_str = &tokenizer.buffer[token.location().start()..token.location().end()];
        match token.tag() {
            Tag::Scheme => assert_eq!(token_str, "https"),
            Tag::Authority => assert_eq!(token_str, "telemakos.io"),
            Tag::Port => assert_eq!(token_str, "90"),
            Tag::Query => assert_eq!(token_str, "kendric_tpabf"),
            Tag::Path => assert_eq!(token_str, ""),
            Tag::Fragment => assert_eq!(token_str, "bruh!"),
            _ => panic!("failed on tag {:?}", token.tag()),
        }
    }
}

#[test]
fn test_tokenizer_invalid_scheme() {
    let test_uri = String::from("htLtps://telemakos.io:90/?kendric_tpabf#bruh!");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    match tokenizer.tokens() {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, InvalidToken {}),
    };
}

#[test]
fn test_tokenizer_invalid_auth() {
    let test_uri = String::from("https://tele%makos.io:90/?kendric_tpabf#bruh!");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    match tokenizer.tokens() {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, InvalidToken {}),
    };
}

#[test]
fn test_tokenizer_invalid_port() {
    let test_uri = String::from("https://telemakos.io:90a/?kendric_tpabf#bruh!");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    match tokenizer.tokens() {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, InvalidToken {}),
    };
}

// Need a test for invalid path, query, and fragment once its implemented in the tokenizer

// #[test]
// #[should_panic]
// fn test_tokenizer_invalid() {
//     let test_uri = String::from("https://telemakos.io:90/?kendric_tpabf#bruh!");
//     let mut tokenizer = Tokenizer::new(test_uri.clone());

//     _ = tokenizer.tokens();
// }
