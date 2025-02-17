pub fn solve_reverse_words_151(s: &str) -> String {
    let words = s.trim().split_whitespace().rev();
    let words = words.flat_map(|word| [" ", word]);
    words.skip(1).collect()
}

pub fn solve_product_array_except_self_238(values: &[i32]) -> Vec<i32> {
    let mut post = Vec::new();
    let mut accum = 1;
    for value in values.iter().rev().copied() {
        accum *= value;
        post.push(accum);
    }

    let mut prefix = 1;
    let mut ret = Vec::new();
    let mut postfix = post.into_iter().rev().skip(1);
    for value in values.iter() {
        ret.push(prefix * postfix.next().unwrap_or(1));
        prefix *= value;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words_151() {
        assert_eq!(
            "blue is sky the",
            solve_reverse_words_151("the sky is blue").as_str()
        );
        assert_eq!(
            "example good a",
            solve_reverse_words_151("a good   example").as_str()
        );
    }

    #[test]
    fn test_product_array_except_self_238() {
        assert_eq!(
            vec![24, 12, 8, 6],
            solve_product_array_except_self_238(&[1, 2, 3, 4])
        );
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            solve_product_array_except_self_238(&[-1, 1, 0, -3, 3])
        );
    }
}
