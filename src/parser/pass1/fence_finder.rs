//! Utility functions for finding matching fences.

use regex::{Match, RegexBuilder};

// Enum to represent the type of fence event
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum EventType {
    TargetClose, // Highest priority for tie-breaking
    OtherOpen,
    TargetOpen, // Lowest priority if at same location as TargetClose
}

// Struct to hold event details for sorting
#[derive(Debug)]
struct Event<'a> {
    pos: usize,
    priority_order: EventType, // Lower value = higher processing priority
    match_obj: Match<'a>,
    actual_event_type: EventType, // To know what action to take
}

/// Finds the matching closing fence for a given opening fence, handling nesting.
/// This function now also correctly skips over nested blocks that use a *different*
/// type of fence (e.g., skips a ````...```` block when searching for ```` ``` ```` closers).
pub(crate) fn find_closing_fence<'a>(
    content: &'a str,
    target_fence_chars: &str, // The type of fence we are trying to close (e.g., "```")
    search_start_pos: usize,
) -> Option<Match<'a>> {
    println!(
        "[find_closing_fence] START: target_fence_chars='{}', search_start_pos={}",
        target_fence_chars, search_start_pos
    );

    let escaped_target_fence = regex::escape(target_fence_chars);
    let target_opening_pattern = format!(r"(?m)^\s*{}([^`\n\r]*)(\r?\n)", escaped_target_fence);
    let target_closing_pattern = format!(r"(?m)^[ \t]*{}[ \t]*$", escaped_target_fence);

    let other_fence_chars = if target_fence_chars == "```" {
        "````"
    } else {
        "```"
    };
    let escaped_other_fence = regex::escape(other_fence_chars);
    let other_opening_pattern = format!(r"(?m)^\s*{}([^`\n\r]*)(\r?\n)", escaped_other_fence);

    let target_opening_re = RegexBuilder::new(&target_opening_pattern)
        .crlf(true)
        .build()
        .unwrap();
    let target_closing_re = RegexBuilder::new(&target_closing_pattern)
        .crlf(true)
        .build()
        .unwrap();
    let other_opening_re = RegexBuilder::new(&other_opening_pattern)
        .crlf(true)
        .build()
        .unwrap();

    let mut level = 1;
    let mut current_pos = search_start_pos;

    loop {
        println!(
            "[find_closing_fence] LOOP: current_pos={}, level={}",
            current_pos, level
        );

        let mut candidates: Vec<Event> = Vec::new();

        if let Some(m) = target_closing_re.find_at(content, current_pos) {
            candidates.push(Event {
                pos: m.start(),
                priority_order: EventType::TargetClose,
                match_obj: m,
                actual_event_type: EventType::TargetClose,
            });
        }
        if let Some(m) = other_opening_re.find_at(content, current_pos) {
            candidates.push(Event {
                pos: m.start(),
                priority_order: EventType::OtherOpen,
                match_obj: m,
                actual_event_type: EventType::OtherOpen,
            });
        }
        if let Some(m) = target_opening_re.find_at(content, current_pos) {
            candidates.push(Event {
                pos: m.start(),
                priority_order: EventType::TargetOpen,
                match_obj: m,
                actual_event_type: EventType::TargetOpen,
            });
        }

        if candidates.is_empty() {
            println!(
                "[find_closing_fence] NO MORE EVENTS: No candidates found. level={}",
                level
            );
            return None;
        }

        // Sort by position, then by priority_order (lower enum discriminant means higher priority)
        candidates.sort_by(|a, b| {
            a.pos
                .cmp(&b.pos)
                .then_with(|| a.priority_order.cmp(&b.priority_order))
        });

        let earliest_event = &candidates[0];
        let current_event_match = earliest_event.match_obj;

        println!(
            "[find_closing_fence]   Selected Event: {:?}, type: {:?}, match: {:?}",
            earliest_event.priority_order,
            earliest_event.actual_event_type,
            (
                current_event_match.start(),
                current_event_match.end(),
                current_event_match.as_str()
            )
        );

        match earliest_event.actual_event_type {
            EventType::TargetClose => {
                println!(
                    "[find_closing_fence]   Event Action: Target Close at {}-{}",
                    current_event_match.start(),
                    current_event_match.end()
                );
                level -= 1;
                if level == 0 {
                    println!(
                        "[find_closing_fence] MATCH FOUND: level=0, match_pos={}, match_str='{:?}'",
                        current_event_match.start(),
                        current_event_match.as_str()
                    );
                    return Some(current_event_match);
                }
                current_pos = current_event_match.end();
                if level < 0 {
                    println!("[find_closing_fence] ERROR: Level < 0 (too many target closers). Returning None.");
                    return None;
                }
            }
            EventType::TargetOpen => {
                println!(
                    "[find_closing_fence]   Event Action: Target Open at {}-{}",
                    current_event_match.start(),
                    current_event_match.end()
                );
                level += 1;
                current_pos = current_event_match.end();
            }
            EventType::OtherOpen => {
                println!(
                    "[find_closing_fence]   Event Action: Other Open ('{}') at {}-{}",
                    other_fence_chars,
                    current_event_match.start(),
                    current_event_match.end()
                );
                if let Some(other_close_match) =
                    find_closing_fence(content, other_fence_chars, current_event_match.end())
                {
                    println!(
                        "[find_closing_fence]     Skipped other block from {} to {}",
                        current_event_match.start(),
                        other_close_match.end()
                    );
                    current_pos = other_close_match.end();
                } else {
                    println!("[find_closing_fence]     Malformed/unclosed 'other' block starting at {}. Treating as text and continuing search for target '{}'.", current_event_match.start(), target_fence_chars);
                    current_pos = current_event_match.end();
                }
            }
        }
    }
}
