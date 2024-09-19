use std::{fmt::Display, ops::Index};

pub struct HttpRequest {
    method: HttpMethodEnum,
    target: Uri,
}

pub enum HttpMethodEnum {
    GET,
    HEAD,
    POST,
}

#[derive(Debug)]
pub enum HttpSchemeEnum {
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
    /// Shittiest parser know to man. Christ, just get a whiteboard and figure
    /// it out man.
    pub fn parse_tokens(tokenizer: &mut Tokenizer) -> Self {
        let mut scheme = HttpSchemeEnum::HTTP;
        let mut host = String::new();
        let mut port = 0 as u16;
        let mut query = None;

        let tokens = tokenizer.tokens();

        for token in tokens {
            match token.tag() {
                Tag::Scheme => {
                    match &tokenizer.buffer[token.location().start()..token.location().end()] {
                        "http" => HttpSchemeEnum::HTTP,
                        "https" => HttpSchemeEnum::HTTPS,
                        &_ => panic!("Invalid token passed somehow."),
                    };
                }
                Tag::Authority => {
                    host = String::from(
                        &tokenizer.buffer[token.location().start()..token.location().end()],
                    );
                }
                Tag::Port => {
                    port = String::from(
                        &tokenizer.buffer[token.location().start()..token.location().end()],
                    )
                    .parse::<u16>()
                    .ok()
                    .unwrap();
                }
                Tag::Query => {
                    query = Some(String::from(
                        &tokenizer.buffer[token.location().start()..token.location().end()],
                    ));
                }
                _ => continue,
            };
        }
        Self {
            scheme,
            host,
            port,
            query,
        }
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tag {
    Unknown,

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
    state: GlobalState,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GlobalState {
    // Only needs to handle single lines.
    // Relevant characters:
    // :// : / ?
    // URI = scheme ":" ["//" [userinfo "@"] host [":"]] path ["?" query] ["#" fragment]
    Start,
    Scheme,
    Authority,
    UserInfo,
    Port,
    Path,
    Query,
    Fragment,

    EndOfURI,
    Invalid,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LocalState {
    Valid,
    Invalid,
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
            state: GlobalState::Start,
        }
    }

    pub fn state(&self) -> GlobalState {
        self.state
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        self.index = 0;
        self.state = GlobalState::Start;

        loop {
            let next = self.next();

            tokens.push(next);
            if self.state == GlobalState::EndOfURI {
                break;
            }
        }

        tokens
    }

    pub fn next(&mut self) -> Token {
        let mut result = Token::new(Tag::Unknown, Location::new(self.index, self.index + 1));
        let mut local_state = LocalState::Valid;

        let buffer_as_chars: Vec<char> = self.buffer.char_indices().map(|(_, b)| b).collect();

        while local_state == LocalState::Valid && local_state != LocalState::EndOfToken {
            // Revisit this section ######################
            if self.index == buffer_as_chars.len() {
                result.tag = match self.state {
                    GlobalState::Start => todo!(),
                    GlobalState::Scheme => Tag::Scheme,
                    GlobalState::Authority => Tag::Authority,
                    GlobalState::UserInfo => Tag::UserInfo,
                    GlobalState::Port => Tag::Port,
                    GlobalState::Path => Tag::Path,
                    GlobalState::Query => Tag::Query,
                    GlobalState::Fragment => Tag::Fragment,
                    GlobalState::Invalid => Tag::Invalid,
                    GlobalState::EndOfURI => continue,
                };
                result.location.end_idx = self.index;
                self.state = GlobalState::EndOfURI;

                return result;
            }
            // ###########################################
            // println!("Checking state {:?}. . .", self.state);

            match self.state {
                GlobalState::Start => match buffer_as_chars[self.index] {
                    'h' => {
                        result.location.start_idx = self.index;
                        self.state = GlobalState::Scheme;
                        self.index += 1;
                    }
                    _ => {
                        result.tag = Tag::Invalid;
                        result.location.end_idx = self.index;
                        local_state = LocalState::Invalid;
                        self.state = GlobalState::Invalid;
                        // self.index += 1;
                    }
                },
                GlobalState::Scheme => match buffer_as_chars[self.index] {
                    // Valid characters; continue
                    't' | 'p' | 's' => self.index += 1,
                    // Post-delimiter; set result tag to current section,
                    // result end location to current index, local state
                    // to EndOfToken, GlobalState to the next section
                    // (Authority)
                    ':' => {
                        result.tag = Tag::Scheme;
                        result.location.end_idx = self.index;
                        local_state = LocalState::EndOfToken;
                        self.state = GlobalState::Authority;
                        self.index += 3; // skip ://
                    }
                    // Non valid characters; set token to invalid, token
                    // location, local state to invalid, and global state to
                    // invalid.
                    _ => {
                        result.tag = Tag::Invalid;
                        result.location.end_idx = self.index;
                        local_state = LocalState::Invalid;
                        self.state = GlobalState::Invalid;
                    }
                },
                GlobalState::Authority => match buffer_as_chars[self.index] {
                    // Post-delimiter; set tag to previous section, set
                    // location, set state to end of token. global state is
                    // already in the correct state. increment index.
                    '@' => {
                        result.tag = Tag::UserInfo;
                        result.location.end_idx = self.index;
                        local_state = LocalState::EndOfToken;
                        // self.state = GlobalState::Authority; // alread in this state
                        self.index += 1
                    }
                    ':' => {
                        result.tag = Tag::Authority;
                        result.location.end_idx = self.index;
                        local_state = LocalState::EndOfToken;
                        self.state = GlobalState::Port;
                        self.index += 1;
                    }
                    '/' => {
                        result.tag = Tag::Authority;
                        result.location.end_idx = self.index;
                        local_state = LocalState::EndOfToken;
                        self.state = GlobalState::Path;
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
                    _ => {
                        result.tag = Tag::Invalid;
                        result.location.end_idx = self.index;
                        local_state = LocalState::Invalid;
                        self.state = GlobalState::Invalid;
                    }
                },
                GlobalState::Port => match buffer_as_chars[self.index] {
                    // Ports are only valid unsigned ints.
                    '0'..='9' => self.index += 1,
                    // Pre-delimiter. '/' begins the path portion. set this
                    // token as port and prepare for path.
                    '/' => {
                        result.tag = Tag::Port;
                        result.location.end_idx = self.index;
                        local_state = LocalState::EndOfToken;
                        self.state = GlobalState::Path;
                        self.index += 1;
                    }
                    _ => {
                        result.tag = Tag::Invalid;
                        result.location.end_idx = self.index;
                        local_state = LocalState::Invalid;
                        self.state = GlobalState::Invalid;
                    }
                },
                GlobalState::Path => match buffer_as_chars[self.index] {
                    // Pre-delimiter. send path and prepare for query.
                    '?' => {
                        result.tag = Tag::Path;
                        result.location.end_idx = self.index;
                        local_state = LocalState::EndOfToken;
                        self.state = GlobalState::Query;
                        self.index += 1;
                    }
                    // If no '?', the check for fragment. Pre-delimiter, send
                    // path and prep fragment.
                    '#' => {
                        result.tag = Tag::Path;
                        result.location.end_idx = self.index;
                        local_state = LocalState::EndOfToken;
                        self.state = GlobalState::Fragment;
                        self.index += 1;
                    }
                    // Everything else is valid until I feel like getting the
                    // ASCII in here.
                    _ => self.index += 1,
                },
                GlobalState::Query => match buffer_as_chars[self.index] {
                    // If fragment, send query, prep frag.
                    '#' => {
                        result.tag = Tag::Query;
                        result.location.end_idx = self.index;
                        local_state = LocalState::EndOfToken;
                        self.state = GlobalState::Fragment;
                        self.index += 1;
                    }
                    // Same thing; when I feel like it.
                    _ => self.index += 1,
                },

                // Again, when I have energy.
                GlobalState::Fragment => match buffer_as_chars[self.index] {
                    _ => self.index += 1,
                },

                GlobalState::UserInfo => todo!(),
                GlobalState::Invalid => {
                    panic!(
                        "Could not parse {} at index {}",
                        buffer_as_chars[self.index], self.index
                    )
                }
                GlobalState::EndOfURI => todo!(),
            }
        }
        result
    }
}

impl Display for Tokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut clone: Tokenizer = self.clone();
        let mut out = String::new();
        for token in clone.tokens() {
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
