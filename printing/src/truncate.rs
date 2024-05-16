pub enum TruncationIndices {
    Empty,
    Truncate(usize, usize),
    NoTruncation,
}

/// Returns the positions wheere the inputs need to be culled from the middle
/// to fit into the given width.
///
/// If both elements of the output tuple are `None`, then the inputs fit into
/// the width without truncation.
///
/// If the first element is `Some`, then the input needs to be truncated at the
/// given index from the left.
///
/// If the second element is `Some`, then the input needs to be truncated at the
/// given index from the right.
pub fn truncation_indices(
    max_weight: usize,
    truncation_separator_width: usize,
    mut weights: impl DoubleEndedIterator<Item = usize>,
) -> TruncationIndices {
    use TruncationIndices::*;
    // If the separator doesn't fit, we can't fit anything.
    if truncation_separator_width >= max_weight {
        return Empty;
    }
    // Assume that the separator string is in the middle, so deduct its length
    // from the maximum weight. We will add the separator width back to the
    // max weight if we are on the last word and it doesn't fit with the
    // separator, but may fit without the separator.
    let max_weight = max_weight - truncation_separator_width;
    let mut left = 0;
    let mut right = 0;
    let mut left_weight = 0;
    let mut right_weight = 0;

    loop {
        match (weights.next(), weights.next_back()) {
            (Some(weight_l), Some(weight_r)) => {
                // Choose the side that has the smallest cumulative weight.
                if left_weight + weight_l <= right_weight + weight_r {
                    if left_weight + right_weight + weight_l < max_weight {
                        left_weight += weight_l + 1;
                        left += 1;
                    } else {
                        // We have found that the left word will not fit, and
                        // that the left word is shorter than the right word,
                        // so we will not be able to fit the right word either.
                        // Therefore, truncate at the current position.
                        return Truncate(left, right);
                    }
                    if left_weight + right_weight + weight_r < max_weight {
                        right_weight += weight_r + 1;
                        right += 1;
                    }
                    // At this point we know the right word will never fit, but
                    // more words may fit on the left side. Iterate through the
                    // rest of the words to find the last word that fits.
                    else {
                        for weight in weights.by_ref() {
                            if left_weight + right_weight + weight < max_weight
                            {
                                left_weight += weight + 1;
                                left += 1;
                            } else {
                                return Truncate(left, right);
                            }
                        }
                        return Truncate(left, right);
                    }
                } else {
                    if left_weight + right_weight + weight_r < max_weight {
                        right_weight += weight_r + 1;
                        right += 1;
                    } else {
                        // We have found that the right word will not fit, and
                        // that the right word is shorter than the left word,
                        // so we will not be able to fit the left word either.
                        // Therefore, truncate at the current position.
                        return Truncate(left, right);
                    }
                    if left_weight + right_weight + weight_l < max_weight {
                        left_weight += weight_l + 1;
                        left += 1;
                    } else {
                        // At this point we know the left word will never fit,
                        // but more words may fit on the right side. Iterate
                        // through the rest of the words to find the last word
                        // that fits.
                        for weight in weights.by_ref().rev() {
                            if left_weight + right_weight + weight < max_weight
                            {
                                right_weight += weight + 1;
                                right += 1;
                            } else {
                                return Truncate(left, right);
                            }
                        }
                        return Truncate(left, right);
                    }
                }
            }
            (Some(weight), None) | (None, Some(weight)) => {
                if left_weight + right_weight + weight + 2 <= max_weight {
                    // We reached the middle, and the last element fits, so no
                    // truncation is needed.
                    return NoTruncation;
                } else {
                    // We reached the middle, but the last element doesn't fit,
                    // so we need to truncate.
                    return Truncate(left, right);
                }
            }
            (None, None) => {
                // We reached the middle, and there are no more elements.
                return NoTruncation;
            }
        }
    }
}

/// Truncate the items so that they will fit into the given width when printed
/// together, separated by spaces.
///
/// If the combined length of the items is less than or equal to `max_width`,
/// items are chopped out of the middle until the remainder fit.
///
/// The `separator` is inserted between the items when they are truncated. There
/// is only at most one separator in the entire output.
///
/// This works by implementing a greedy algorithm that tries to fit as many items
/// as possible into the output, from either end of the input list, while trying
/// to keep the cumulative length of either end "balanced" so that the separator
/// is placed in the middle of the output if it is needed.
///
/// If the input list is empty, the output list will also be empty.
///
/// If the input list fits into `max_width` without truncation, the output list
/// will be the same as the input list.
///
/// If the left end of the input fits and the right end does not, or vice-versa,
/// the output list will contain the longest end that fits with the separator
/// on the other end.
///
/// If neither end fits, the output list will just contain the separator.
///
/// If the separator doesn't even fit, the output list will be empty.
#[allow(unused)]
pub fn truncate<'a>(
    max_width: usize,
    separator: &'a str,
    words: &[&'a str],
) -> Vec<&'a str> {
    let weights = words.iter().map(|word| word.len());
    let indices = truncation_indices(max_width, separator.len(), weights);

    match indices {
        TruncationIndices::Empty => vec![],
        TruncationIndices::NoTruncation => words.to_vec(),
        TruncationIndices::Truncate(left, right) => {
            let mut result = words[0..left].to_vec();
            result.push(separator);
            result.extend(&words[words.len() - right..]);
            result
        }
    }
}
