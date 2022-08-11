//! Label end is a construct that occurs in the [text][] conten&t type.
//!
//! It forms with the following BNF:
//!
//! ```bnf
//! label_end ::= ']' [ resource | reference_full | reference_collapsed ]
//!
//! resource ::= '(' [ whitespace ] destination [ whitespace title ] [ whitespace ] ')'
//! reference_full ::= '[' label ']'
//! reference_collapsed ::= '[' ']'
//!
//! ; See the `destination`, `title`, and `label` constructs for the BNF of
//! ; those parts.
//! ```
//!
//! See [`destination`][destination], [`label`][label], and [`title`][title]
//! for grammar, notes, and recommendations.
//!
//! Label end does not, on its own, relate to anything in HTML.
//! When matched with a [label start (link)][label_start_link], they together
//! relate to the `<a>` element in HTML.
//! See [*§ 4.5.1 The `a` element*][html-a] in the HTML spec for more info.
//! It can also match with [label start (image)][label_start_image], in which
//! case they form an `<img>` element.
//! See [*§ 4.8.3 The `img` element*][html-img] in the HTML spec for more info.
//!
//! In the case of a resource, the destination and title are given directly
//! with the label end.
//! In the case of a reference, this information is provided by a matched
//! [definition][].
//! Full references (`[x][y]`) match to definitions through their explicit,
//! second, label (`y`).
//! Collapsed references (`[x][]`) and shortcut references (`[x]`) match by
//! interpreting the text provided between the first, implicit, label (`x`).
//! To match, the effective label of the reference must be equal to the label
//! of the definition after normalizing with
//! [`normalize_identifier`][normalize_identifier].
//!
//! Importantly, while the label of a full reference *can* include [string][]
//! content, and in case of collapsed and shortcut references even [text][]
//! content, that content is not considered when matching.
//! To illustrate, neither label matches the definition:
//!
//! ```markdown
//! [a&b]: https://example.com
//!
//! [x][a&amp;b], [a\&b][]
//! ```
//!
//! When the resource or reference matches, the destination forms the `href`
//! attribute in case of a [label start (link)][label_start_link], and an
//! `src` attribute otherwise.
//! The title is formed, optionally, on either `<a>` or `<img>`.
//!
//! For info on how to encode characters in URLs, see
//! [`destination`][destination].
//! For info on how characters are encoded as `href` on `<a>` or `src` on
//! `<img>` when compiling, see
//! [`sanitize_uri`][sanitize_uri].
//!
//! In case of a matched [label start (link)][label_start_link], the interpreted
//! content between it and the label end, is placed between the opening and
//! closing tags.
//! Otherwise, the text is also interpreted, but used *without* the resulting
//! tags:
//!
//! ```markdown
//! [a *b* c](#)
//!
//! ![a *b* c](#)
//! ```
//!
//! Yields:
//!
//! ```html
//! <p><a href="#">a <em>b</em> c</a></p>
//! <p><img src="#" alt="a b c" /></p>
//! ```
//!
//! It is possible to use images in links.
//! It’s somewhat possible to have links in images (the text will be used, not
//! the HTML, see above).
//! But it’s not possible to use links in links.
//! The “deepest” link wins.
//! To illustrate:
//!
//! ```markdown
//! a [b [c](#) d](#) e
//! ```
//!
//! Yields:
//!
//! ```html
//! <p>a [b <a href="#">c</a> d](#) e</p>
//! ```
//!
//! This limiation is imposed because links in links is invalid according to
//! HTML.
//! Technically though, in markdown it is still possible to construct them by
//! using an [autolink][] in a link.
//! You definitely should not do that.
//!
//! ## Tokens
//!
//! *   [`Data`][Name::Data]
//! *   [`Image`][Name::Image]
//! *   [`Label`][Name::Label]
//! *   [`LabelEnd`][Name::LabelEnd]
//! *   [`LabelMarker`][Name::LabelMarker]
//! *   [`LabelText`][Name::LabelText]
//! *   [`LineEnding`][Name::LineEnding]
//! *   [`Link`][Name::Link]
//! *   [`Reference`][Name::Reference]
//! *   [`ReferenceMarker`][Name::ReferenceMarker]
//! *   [`ReferenceString`][Name::ReferenceString]
//! *   [`Resource`][Name::Resource]
//! *   [`ResourceDestination`][Name::ResourceDestination]
//! *   [`ResourceDestinationLiteral`][Name::ResourceDestinationLiteral]
//! *   [`ResourceDestinationLiteralMarker`][Name::ResourceDestinationLiteralMarker]
//! *   [`ResourceDestinationRaw`][Name::ResourceDestinationRaw]
//! *   [`ResourceDestinationString`][Name::ResourceDestinationString]
//! *   [`ResourceMarker`][Name::ResourceMarker]
//! *   [`ResourceTitle`][Name::ResourceTitle]
//! *   [`ResourceTitleMarker`][Name::ResourceTitleMarker]
//! *   [`ResourceTitleString`][Name::ResourceTitleString]
//! *   [`SpaceOrTab`][Name::SpaceOrTab]
//!
//! ## References
//!
//! *   [`label-end.js` in `micromark`](https://github.com/micromark/micromark/blob/main/packages/micromark-core-commonmark/dev/lib/label-end.js)
//! *   [*§ 4.7 Link reference definitions* in `CommonMark`](https://spec.commonmark.org/0.30/#link-reference-definitions)
//! *   [*§ 6.3 Links* in `CommonMark`](https://spec.commonmark.org/0.30/#links)
//! *   [*§ 6.4 Images* in `CommonMark`](https://spec.commonmark.org/0.30/#images)
//!
//! [string]: crate::content::string
//! [text]: crate::content::text
//! [destination]: crate::construct::partial_destination
//! [title]: crate::construct::partial_title
//! [label]: crate::construct::partial_label
//! [label_start_image]: crate::construct::label_start_image
//! [label_start_link]: crate::construct::label_start_link
//! [definition]: crate::construct::definition
//! [autolink]: crate::construct::autolink
//! [sanitize_uri]: crate::util::sanitize_uri::sanitize_uri
//! [normalize_identifier]: crate::util::normalize_identifier::normalize_identifier
//! [html-a]: https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element
//! [html-img]: https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element

use crate::constant::RESOURCE_DESTINATION_BALANCE_MAX;
use crate::construct::partial_space_or_tab_eol::space_or_tab_eol;
use crate::event::{Event, Kind, Name};
use crate::resolve::Name as ResolveName;
use crate::state::{Name as StateName, State};
use crate::tokenizer::{Media, Tokenizer};
use crate::util::{
    normalize_identifier::normalize_identifier,
    skip,
    slice::{Position, Slice},
};

/// Start of label end.
///
/// ```markdown
/// > | [a](b) c
///       ^
/// > | [a][b] c
///       ^
/// > | [a][] b
///       ^
/// > | [a] b
/// ```
pub fn start(tokenizer: &mut Tokenizer) -> State {
    if Some(b']') == tokenizer.current && tokenizer.parse_state.constructs.label_end {
        // If there is an okay opening:
        if !tokenizer.tokenize_state.label_start_stack.is_empty() {
            let label_start = tokenizer.tokenize_state.label_start_stack.last().unwrap();

            tokenizer.tokenize_state.end = tokenizer.events.len();

            // Mark as balanced if the info is inactive.
            if label_start.inactive {
                return State::Retry(StateName::LabelEndNok);
            }

            tokenizer.enter(Name::LabelEnd);
            tokenizer.enter(Name::LabelMarker);
            tokenizer.consume();
            tokenizer.exit(Name::LabelMarker);
            tokenizer.exit(Name::LabelEnd);
            return State::Next(StateName::LabelEndAfter);
        }
    }

    State::Nok
}

/// After `]`.
///
/// ```markdown
/// > | [a](b) c
///       ^
/// > | [a][b] c
///       ^
/// > | [a][] b
///       ^
/// > | [a] b
///       ^
/// ```
pub fn after(tokenizer: &mut Tokenizer) -> State {
    let start = tokenizer.tokenize_state.label_start_stack.last().unwrap();
    let defined = tokenizer
        .parse_state
        .definitions
        .contains(&normalize_identifier(
            // We don’t care about virtual spaces, so `indices` and `as_str` are fine.
            Slice::from_indices(
                tokenizer.parse_state.bytes,
                tokenizer.events[start.start.1].point.index,
                tokenizer.events[tokenizer.tokenize_state.end].point.index,
            )
            .as_str(),
        ));

    match tokenizer.current {
        // Resource (`[asd](fgh)`)?
        Some(b'(') => {
            tokenizer.attempt(
                State::Next(StateName::LabelEndOk),
                State::Next(if defined {
                    StateName::LabelEndOk
                } else {
                    StateName::LabelEndNok
                }),
            );
            State::Retry(StateName::LabelEndResourceStart)
        }
        // Full (`[asd][fgh]`) or collapsed (`[asd][]`) reference?
        Some(b'[') => {
            tokenizer.attempt(
                State::Next(StateName::LabelEndOk),
                State::Next(if defined {
                    StateName::LabelEndReferenceNotFull
                } else {
                    StateName::LabelEndNok
                }),
            );
            State::Retry(StateName::LabelEndReferenceFull)
        }
        // Shortcut (`[asd]`) reference?
        _ => State::Retry(if defined {
            StateName::LabelEndOk
        } else {
            StateName::LabelEndNok
        }),
    }
}

/// After `]`, at `[`, but not at a full reference.
///
/// > 👉 **Note**: we only get here if the label is defined.
///
/// ```markdown
/// > | [a][] b
///        ^
/// > | [a] b
///        ^
/// ```
pub fn reference_not_full(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::LabelEndOk),
        State::Next(StateName::LabelEndNok),
    );
    State::Retry(StateName::LabelEndReferenceCollapsed)
}

/// Done, we found something.
///
/// ```markdown
/// > | [a](b) c
///           ^
/// > | [a][b] c
///           ^
/// > | [a][] b
///          ^
/// > | [a] b
///        ^
/// ```
pub fn ok(tokenizer: &mut Tokenizer) -> State {
    // Remove the start.
    let label_start = tokenizer.tokenize_state.label_start_stack.pop().unwrap();

    let is_link = tokenizer.events[label_start.start.0].name == Name::LabelLink;

    if is_link {
        let mut index = 0;
        while index < tokenizer.tokenize_state.label_start_stack.len() {
            let label_start = &mut tokenizer.tokenize_state.label_start_stack[index];
            if tokenizer.events[label_start.start.0].name == Name::LabelLink {
                label_start.inactive = true;
            }
            index += 1;
        }
    }

    tokenizer.tokenize_state.media_list.push(Media {
        start: label_start.start,
        end: (tokenizer.tokenize_state.end, tokenizer.events.len() - 1),
    });
    tokenizer.tokenize_state.end = 0;
    tokenizer.register_resolver_before(ResolveName::Label);
    State::Ok
}

/// Done, it’s nothing.
///
/// There was an okay opening, but we didn’t match anything.
///
/// ```markdown
/// > | [a](b c
///        ^
/// > | [a][b c
///        ^
/// > | [a] b
///        ^
/// ```
pub fn nok(tokenizer: &mut Tokenizer) -> State {
    let start = tokenizer.tokenize_state.label_start_stack.pop().unwrap();

    tokenizer.tokenize_state.label_start_list_loose.push(start);

    tokenizer.tokenize_state.end = 0;
    State::Nok
}

/// Before a resource, at `(`.
///
/// ```markdown
/// > | [a](b) c
///        ^
/// ```
pub fn resource_start(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'(') => {
            tokenizer.enter(Name::Resource);
            tokenizer.enter(Name::ResourceMarker);
            tokenizer.consume();
            tokenizer.exit(Name::ResourceMarker);
            State::Next(StateName::LabelEndResourceBefore)
        }
        _ => unreachable!("expected `(`"),
    }
}

/// At the start of a resource, after `(`, before a destination.
///
/// ```markdown
/// > | [a](b) c
///         ^
/// ```
pub fn resource_before(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::LabelEndResourceOpen),
        State::Next(StateName::LabelEndResourceOpen),
    );
    State::Retry(space_or_tab_eol(tokenizer))
}

/// At the start of a resource, after optional whitespace.
///
/// ```markdown
/// > | [a](b) c
///         ^
/// ```
pub fn resource_open(tokenizer: &mut Tokenizer) -> State {
    if let Some(b')') = tokenizer.current {
        State::Retry(StateName::LabelEndResourceEnd)
    } else {
        tokenizer.tokenize_state.token_1 = Name::ResourceDestination;
        tokenizer.tokenize_state.token_2 = Name::ResourceDestinationLiteral;
        tokenizer.tokenize_state.token_3 = Name::ResourceDestinationLiteralMarker;
        tokenizer.tokenize_state.token_4 = Name::ResourceDestinationRaw;
        tokenizer.tokenize_state.token_5 = Name::ResourceDestinationString;
        tokenizer.tokenize_state.size_b = RESOURCE_DESTINATION_BALANCE_MAX;

        tokenizer.attempt(
            State::Next(StateName::LabelEndResourceDestinationAfter),
            State::Next(StateName::LabelEndResourceDestinationMissing),
        );
        State::Retry(StateName::DestinationStart)
    }
}

/// In a resource, after a destination, before optional whitespace.
///
/// ```markdown
/// > | [a](b) c
///          ^
/// ```
pub fn resource_destination_after(tokenizer: &mut Tokenizer) -> State {
    tokenizer.tokenize_state.token_1 = Name::Data;
    tokenizer.tokenize_state.token_2 = Name::Data;
    tokenizer.tokenize_state.token_3 = Name::Data;
    tokenizer.tokenize_state.token_4 = Name::Data;
    tokenizer.tokenize_state.token_5 = Name::Data;
    tokenizer.tokenize_state.size_b = 0;
    tokenizer.attempt(
        State::Next(StateName::LabelEndResourceBetween),
        State::Next(StateName::LabelEndResourceEnd),
    );
    State::Retry(space_or_tab_eol(tokenizer))
}

/// Without destination.
pub fn resource_destination_missing(tokenizer: &mut Tokenizer) -> State {
    tokenizer.tokenize_state.token_1 = Name::Data;
    tokenizer.tokenize_state.token_2 = Name::Data;
    tokenizer.tokenize_state.token_3 = Name::Data;
    tokenizer.tokenize_state.token_4 = Name::Data;
    tokenizer.tokenize_state.token_5 = Name::Data;
    tokenizer.tokenize_state.size_b = 0;
    State::Nok
}

/// In a resource, after a destination, after whitespace.
///
/// ```markdown
/// > | [a](b ) c
///           ^
/// ```
pub fn resource_between(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'"' | b'\'' | b'(') => {
            tokenizer.tokenize_state.token_1 = Name::ResourceTitle;
            tokenizer.tokenize_state.token_2 = Name::ResourceTitleMarker;
            tokenizer.tokenize_state.token_3 = Name::ResourceTitleString;
            tokenizer.attempt(
                State::Next(StateName::LabelEndResourceTitleAfter),
                State::Nok,
            );
            State::Retry(StateName::TitleStart)
        }
        _ => State::Retry(StateName::LabelEndResourceEnd),
    }
}

/// In a resource, after a title.
///
/// ```markdown
/// > | [a](b "c") d
///              ^
/// ```
pub fn resource_title_after(tokenizer: &mut Tokenizer) -> State {
    tokenizer.tokenize_state.token_1 = Name::Data;
    tokenizer.tokenize_state.token_2 = Name::Data;
    tokenizer.tokenize_state.token_3 = Name::Data;
    tokenizer.attempt(
        State::Next(StateName::LabelEndResourceEnd),
        State::Next(StateName::LabelEndResourceEnd),
    );
    State::Retry(space_or_tab_eol(tokenizer))
}

/// In a resource, at the `)`.
///
/// ```markdown
/// > | [a](b) d
///          ^
/// ```
pub fn resource_end(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b')') => {
            tokenizer.enter(Name::ResourceMarker);
            tokenizer.consume();
            tokenizer.exit(Name::ResourceMarker);
            tokenizer.exit(Name::Resource);
            State::Ok
        }
        _ => State::Nok,
    }
}

/// In a reference (full), at the `[`.
///
/// ```markdown
/// > | [a][b] d
///        ^
/// ```
pub fn reference_full(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'[') => {
            tokenizer.tokenize_state.token_1 = Name::Reference;
            tokenizer.tokenize_state.token_2 = Name::ReferenceMarker;
            tokenizer.tokenize_state.token_3 = Name::ReferenceString;
            tokenizer.attempt(
                State::Next(StateName::LabelEndReferenceFullAfter),
                State::Nok,
            );
            State::Retry(StateName::LabelStart)
        }
        _ => unreachable!("expected `[`"),
    }
}

/// In a reference (full), after `]`.
///
/// ```markdown
/// > | [a][b] d
///          ^
/// ```
pub fn reference_full_after(tokenizer: &mut Tokenizer) -> State {
    tokenizer.tokenize_state.token_1 = Name::Data;
    tokenizer.tokenize_state.token_2 = Name::Data;
    tokenizer.tokenize_state.token_3 = Name::Data;

    if tokenizer
        .parse_state
        .definitions
        // We don’t care about virtual spaces, so `as_str` is fine.
        .contains(&normalize_identifier(
            Slice::from_position(
                tokenizer.parse_state.bytes,
                &Position::from_exit_event(
                    &tokenizer.events,
                    skip::to_back(
                        &tokenizer.events,
                        tokenizer.events.len() - 1,
                        &[Name::ReferenceString],
                    ),
                ),
            )
            .as_str(),
        ))
    {
        State::Ok
    } else {
        State::Nok
    }
}

/// In a reference (collapsed), at the `[`.
///
/// > 👉 **Note**: we only get here if the label is defined.
///
/// ```markdown
/// > | [a][] d
///        ^
/// ```
pub fn reference_collapsed(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'[') => {
            tokenizer.enter(Name::Reference);
            tokenizer.enter(Name::ReferenceMarker);
            tokenizer.consume();
            tokenizer.exit(Name::ReferenceMarker);
            State::Next(StateName::LabelEndReferenceCollapsedOpen)
        }
        _ => State::Nok,
    }
}

/// In a reference (collapsed), at the `]`.
///
/// > 👉 **Note**: we only get here if the label is defined.
///
/// ```markdown
/// > | [a][] d
///         ^
/// ```
pub fn reference_collapsed_open(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b']') => {
            tokenizer.enter(Name::ReferenceMarker);
            tokenizer.consume();
            tokenizer.exit(Name::ReferenceMarker);
            tokenizer.exit(Name::Reference);
            State::Ok
        }
        _ => State::Nok,
    }
}

/// Resolve media.
///
/// This turns correct label start (image, link) and label end into links and
/// images, or turns them back into data.
#[allow(clippy::too_many_lines)]
pub fn resolve(tokenizer: &mut Tokenizer) {
    let mut left = tokenizer.tokenize_state.label_start_list_loose.split_off(0);
    let mut left_2 = tokenizer.tokenize_state.label_start_stack.split_off(0);
    let media = tokenizer.tokenize_state.media_list.split_off(0);
    left.append(&mut left_2);

    let events = &tokenizer.events;

    // Remove loose label starts.
    let mut index = 0;
    while index < left.len() {
        let label_start = &left[index];
        let data_enter_index = label_start.start.0;
        let data_exit_index = label_start.start.1;

        tokenizer.map.add(
            data_enter_index,
            data_exit_index - data_enter_index + 1,
            vec![
                Event {
                    kind: Kind::Enter,
                    name: Name::Data,
                    point: events[data_enter_index].point.clone(),
                    link: None,
                },
                Event {
                    kind: Kind::Exit,
                    name: Name::Data,
                    point: events[data_exit_index].point.clone(),
                    link: None,
                },
            ],
        );

        index += 1;
    }

    // Add grouping events.
    let mut index = 0;
    while index < media.len() {
        let media = &media[index];
        // LabelLink:Enter or LabelImage:Enter.
        let group_enter_index = media.start.0;
        let group_enter_event = &events[group_enter_index];
        // LabelLink:Exit or LabelImage:Exit.
        let text_enter_index = media.start.0
            + (if group_enter_event.name == Name::LabelLink {
                4
            } else {
                6
            });
        // LabelEnd:Enter.
        let text_exit_index = media.end.0;
        // LabelEnd:Exit.
        let label_exit_index = media.end.0 + 3;
        // Resource:Exit, etc.
        let group_end_index = media.end.1;

        // Insert a group enter and label enter.
        tokenizer.map.add(
            group_enter_index,
            0,
            vec![
                Event {
                    kind: Kind::Enter,
                    name: if group_enter_event.name == Name::LabelLink {
                        Name::Link
                    } else {
                        Name::Image
                    },
                    point: group_enter_event.point.clone(),
                    link: None,
                },
                Event {
                    kind: Kind::Enter,
                    name: Name::Label,
                    point: group_enter_event.point.clone(),
                    link: None,
                },
            ],
        );

        // Empty events not allowed.
        if text_enter_index != text_exit_index {
            // Insert a text enter.
            tokenizer.map.add(
                text_enter_index,
                0,
                vec![Event {
                    kind: Kind::Enter,
                    name: Name::LabelText,
                    point: events[text_enter_index].point.clone(),
                    link: None,
                }],
            );

            // Insert a text exit.
            tokenizer.map.add(
                text_exit_index,
                0,
                vec![Event {
                    kind: Kind::Exit,
                    name: Name::LabelText,
                    point: events[text_exit_index].point.clone(),
                    link: None,
                }],
            );
        }

        // Insert a label exit.
        tokenizer.map.add(
            label_exit_index + 1,
            0,
            vec![Event {
                kind: Kind::Exit,
                name: Name::Label,
                point: events[label_exit_index].point.clone(),
                link: None,
            }],
        );

        // Insert a group exit.
        tokenizer.map.add(
            group_end_index + 1,
            0,
            vec![Event {
                kind: Kind::Exit,
                name: if group_enter_event.name == Name::LabelLink {
                    Name::Link
                } else {
                    Name::Image
                },
                point: events[group_end_index].point.clone(),
                link: None,
            }],
        );

        index += 1;
    }

    tokenizer.map.consume(&mut tokenizer.events);
}
