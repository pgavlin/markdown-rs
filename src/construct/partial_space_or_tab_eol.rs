//! Several helpers to parse whitespace (`space_or_tab`, `space_or_tab_eol`).
//!
//! ## References
//!
//! *   [`micromark-factory-space/index.js` in `micromark`](https://github.com/micromark/micromark/blob/main/packages/micromark-factory-space/dev/index.js)

use crate::construct::partial_space_or_tab::{
    space_or_tab_with_options, Options as SpaceOrTabOptions,
};
use crate::event::{Content, Name};
use crate::state::{Name as StateName, State};
use crate::subtokenize::link;
use crate::tokenizer::Tokenizer;

/// Options to parse `space_or_tab` and one optional eol, but no blank line.
#[derive(Debug)]
pub struct Options {
    /// Connect this whitespace to the previous.
    pub connect: bool,
    /// Embedded content type to use.
    pub content_type: Option<Content>,
}

/// `space_or_tab`, or optionally `space_or_tab`, one `eol`, and
/// optionally `space_or_tab`.
///
/// ```bnf
/// space_or_tab_eol ::= 1*( ' ' '\t' ) | 0*( ' ' '\t' ) eol 0*( ' ' '\t' )
/// ```
pub fn space_or_tab_eol(tokenizer: &mut Tokenizer) -> StateName {
    space_or_tab_eol_with_options(
        tokenizer,
        Options {
            content_type: None,
            connect: false,
        },
    )
}

/// `space_or_tab_eol`, with the given options.
pub fn space_or_tab_eol_with_options(tokenizer: &mut Tokenizer, options: Options) -> StateName {
    tokenizer.tokenize_state.space_or_tab_eol_content_type = options.content_type;
    tokenizer.tokenize_state.space_or_tab_eol_connect = options.connect;
    StateName::SpaceOrTabEolStart
}

pub fn eol_start(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::SpaceOrTabEolAfterFirst),
        State::Next(StateName::SpaceOrTabEolAtEol),
    );

    State::Retry(space_or_tab_with_options(
        tokenizer,
        SpaceOrTabOptions {
            kind: Name::SpaceOrTab,
            min: 1,
            max: usize::MAX,
            content_type: tokenizer
                .tokenize_state
                .space_or_tab_eol_content_type
                .clone(),
            connect: tokenizer.tokenize_state.space_or_tab_eol_connect,
        },
    ))
}

pub fn eol_after_first(tokenizer: &mut Tokenizer) -> State {
    tokenizer.tokenize_state.space_or_tab_eol_ok = true;

    if tokenizer
        .tokenize_state
        .space_or_tab_eol_content_type
        .is_some()
    {
        tokenizer.tokenize_state.space_or_tab_eol_connect = true;
    }

    State::Retry(StateName::SpaceOrTabEolAtEol)
}

/// `space_or_tab_eol`: after optionally first `space_or_tab`.
///
/// ```markdown
/// > | a
///      ^
///   | b
/// ```
pub fn eol_at_eol(tokenizer: &mut Tokenizer) -> State {
    if let Some(b'\n') = tokenizer.current {
        tokenizer.enter_with_content(
            Name::LineEnding,
            tokenizer
                .tokenize_state
                .space_or_tab_eol_content_type
                .clone(),
        );

        if tokenizer.tokenize_state.space_or_tab_eol_connect {
            let index = tokenizer.events.len() - 1;
            link(&mut tokenizer.events, index);
        } else if tokenizer
            .tokenize_state
            .space_or_tab_eol_content_type
            .is_some()
        {
            tokenizer.tokenize_state.space_or_tab_eol_connect = true;
        }

        tokenizer.consume();
        tokenizer.exit(Name::LineEnding);
        State::Next(StateName::SpaceOrTabEolAfterEol)
    } else {
        let ok = tokenizer.tokenize_state.space_or_tab_eol_ok;
        tokenizer.tokenize_state.space_or_tab_eol_content_type = None;
        tokenizer.tokenize_state.space_or_tab_eol_connect = false;
        tokenizer.tokenize_state.space_or_tab_eol_ok = false;
        if ok {
            State::Ok
        } else {
            State::Nok
        }
    }
}

/// `space_or_tab_eol`: after eol.
///
/// ```markdown
///   | a
/// > | b
///     ^
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn eol_after_eol(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::SpaceOrTabEolAfterMore),
        State::Next(StateName::SpaceOrTabEolAfterMore),
    );
    State::Retry(space_or_tab_with_options(
        tokenizer,
        SpaceOrTabOptions {
            kind: Name::SpaceOrTab,
            min: 1,
            max: usize::MAX,
            content_type: tokenizer
                .tokenize_state
                .space_or_tab_eol_content_type
                .clone(),
            connect: tokenizer.tokenize_state.space_or_tab_eol_connect,
        },
    ))
}

/// `space_or_tab_eol`: after more (optional) `space_or_tab`.
///
/// ```markdown
///   | a
/// > | b
///     ^
/// ```
pub fn eol_after_more(tokenizer: &mut Tokenizer) -> State {
    tokenizer.tokenize_state.space_or_tab_eol_content_type = None;
    tokenizer.tokenize_state.space_or_tab_eol_connect = false;
    tokenizer.tokenize_state.space_or_tab_eol_ok = false;

    // Blank line not allowed.
    if matches!(tokenizer.current, None | Some(b'\n')) {
        State::Nok
    } else {
        State::Ok
    }
}
