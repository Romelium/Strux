//! Utility functions for finding matching fences.

use regex::{Match, RegexBuilder};

// Enum to represent the type of fence event for sorting.
// Lower discriminant = higher priority if at same position.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum EventKind {
    TargetClose,
    AnyOpen,
}

// Struct to hold event details for sorting
#[derive(Debug)]
struct Event<'a> {
    pos: usize,
    kind: EventKind,
    match_obj: Match<'a>,
    // Store the captured fence string for 'AnyOpen' events
    fence_chars: Option<&'a str>,
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
    let target_closing_pattern = format!(r"(?m)^[ \t]*{}[ \t]*$", escaped_target_fence);
    // Regex for ANY opening fence of 3+ backticks. Capture the fence itself.
    let any_opening_pattern = r"(?m)^\s*(`{3,})[^`\n\r]*(\r?\n)";

    let target_closing_re = RegexBuilder::new(&target_closing_pattern)
        .crlf(true)
        .build()
        .unwrap();
    let any_opening_re = RegexBuilder::new(any_opening_pattern)
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
                kind: EventKind::TargetClose,
                match_obj: m,
                fence_chars: None,
            });
        }
        // Find next potential opening fence of *any* type
        if let Some(caps) = any_opening_re.captures_at(content, current_pos) {
            let full_match = caps.get(0).unwrap();
            let fence_match = caps.get(1).unwrap();
            candidates.push(Event {
                pos: full_match.start(),
                kind: EventKind::AnyOpen,
                match_obj: full_match,
                fence_chars: Some(fence_match.as_str()),
            });
        }

        if candidates.is_empty() {
            println!(
                "[find_closing_fence] NO MORE EVENTS: No candidates found. level={}",
                level
            );
            return None;
        }

        // Sort by position, then by kind (TargetClose wins if at same position)
        candidates.sort_by(|a, b| a.pos.cmp(&b.pos).then_with(|| a.kind.cmp(&b.kind)));

        let earliest_event = &candidates[0];
        let current_event_match = earliest_event.match_obj;

        println!(
            "[find_closing_fence]   Selected Event: {:?}, match: {:?}",
            earliest_event.kind,
            (
                current_event_match.start(),
                current_event_match.end(),
                current_event_match.as_str()
            )
        );

        match earliest_event.kind {
            EventKind::TargetClose => {
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
            EventKind::AnyOpen => {
                let opening_fence_chars = earliest_event.fence_chars.unwrap();
                println!(
                    "[find_closing_fence]   Event Action: Any Open ('{}') at {}-{}",
                    opening_fence_chars,
                    current_event_match.start(),
                    current_event_match.end()
                );

                if opening_fence_chars == target_fence_chars {
                    // Nested block of the same type
                    level += 1;
                    current_pos = current_event_match.end();
                } else {
                    // Nested block of a different type, we need to find its end and skip over it
                    if let Some(other_close_match) =
                        find_closing_fence(content, opening_fence_chars, current_event_match.end())
                    {
                        println!(
                            "[find_closing_fence]     Skipped nested block from {} to {}",
                            current_event_match.start(),
                            other_close_match.end()
                        );
                        current_pos = other_close_match.end();
                    } else {
                        // Malformed/unclosed inner block. Instead of failing the whole search,
                        // treat this unclosed opening fence as simple text and continue the
                        // search for the original target from after this line. This handles
                        // cases where content inside a block looks like a fence but isn't.
                        println!("[find_closing_fence]     Unclosed nested block starting at {}. Treating as text and continuing search for target '{}'.", current_event_match.start(), target_fence_chars);
                        current_pos = current_event_match.end();
                    }
                }
            }
        }
    }
}
