use genawaiter::sync::{Gen, GenBoxed};

/// Binary Reflexed Gray Code Generator
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

