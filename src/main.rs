use genawaiter::stack::{let_gen_using, Co};

async fn producer(co: Co<'_, i32>) {
    let mut n = 1;
    while n < 10 {
        co.yield_(n).await;
        n += 2;
    }
}

fn main() {
    let_gen_using!(odds_under_ten, producer);

    for num in odds_under_ten {
        println!("{}", num);
    }
}
