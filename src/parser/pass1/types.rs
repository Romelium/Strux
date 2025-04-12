//! Type aliases for Pass 1 parser results.

use crate::core_types::Action;
use crate::errors::ParseError;

/// Represents the byte range (start, end) of a code block relative to the parsed content slice.
pub type BlockRange = (usize, usize);

/// Information about a successfully parsed action from a standard code block.
/// Contains the Action, its header's starting position (relative), and the source ("external", "internal", etc.).
pub type BlockActionInfo = (Action, usize, &'static str);

/// Result type for functions determining the action associated with a standard code block.
// This replaces the old DeterminedAction - it directly gives the result of the determination.
pub type DeterminationResult = Result<Option<BlockActionInfo>, ParseError>;

/// Information about a successfully parsed action from a wrapped header (` ```markdown ` block).
/// Contains the Action, its header's starting position (relative), and the range of the *next* code block it applies to.
/// For wrapped Delete actions, the BlockRange will be (0, 0).
pub type WrappedActionInfo = (Action, usize, BlockRange);

/// Result type for functions determining the action associated with a wrapped header block.
pub type WrappedActionResult = Result<Option<WrappedActionInfo>, ParseError>;

// REMOVED DeterminedAction as it's replaced by DeterminationResult
// /// Result type combining both possibilities for the main block action determination function.
// pub type DeterminedAction =
//     Result<(Option<(Action, usize, &'static str)>, &'static str), ParseError>;
