use genawaiter::sync::{Gen, GenBoxed};

/// The `brgc_gen` function generates a binary reflexed gray code sequence of length `n`.
///
/// Arguments:
///
/// * `n`: The parameter `n` represents the number of bits in the binary reflexed gray code sequence.
///
/// Returns:
///
/// The function `brgc_gen` returns a `GenBoxed<usize>`, which is a boxed generator that yields values
/// of type `usize`.
///
/// # Examples
///
/// ```
/// use ecgen::brgc_gen;
///  
/// let mut lst = ["⬜"; 3];
/// println!("{}", lst.concat());
/// let mut cnt = 1;
/// for n in brgc_gen(lst.len()) {
///     lst[n] = if lst[n] == "⬜" { "⬛" } else { "⬜" };
///     println!("{}", lst.concat());
///     cnt += 1;
/// }
///
/// assert_eq!(cnt, 8);
/// ```
pub fn brgc_gen(n: usize) -> GenBoxed<usize> {
    Gen::new_boxed(|co| async move {
        if n < 1 {
            return;
        }
        for i in brgc_gen(n - 1) {
            co.yield_(i).await;
        }
        co.yield_(n - 1).await;
        for i in brgc_gen(n - 1) {
            co.yield_(i).await;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brgc() {
        let mut cnt = 1;
        for _n in brgc_gen(3) {
            cnt += 1;
        }
        assert_eq!(cnt, 8);
    }
}
