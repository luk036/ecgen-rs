use ecgen::combin::emk_comb_gen;
use ecgen::combin_old::emk_gen;

fn main() {
    let new_emk: Vec<_> = emk_comb_gen(5, 2).into_iter().collect();
    let old_emk: Vec<_> = emk_gen(5, 2).into_iter().collect();

    println!("new (5,2): {:?}", new_emk);
    println!("old (5,2): {:?}", old_emk);

    let new_emk3: Vec<_> = emk_comb_gen(5, 0).into_iter().collect();
    let old_emk3: Vec<_> = emk_gen(5, 0).into_iter().collect();
    println!("new (5,0): {:?}", new_emk3);
    println!("old (5,0): {:?}", old_emk3);

    let new_emk4: Vec<_> = emk_comb_gen(5, 1).into_iter().collect();
    let old_emk4: Vec<_> = emk_gen(5, 1).into_iter().collect();
    println!("new (5,1): {:?}", new_emk4);
    println!("old (5,1): {:?}", old_emk4);

    let new_emk5: Vec<_> = emk_comb_gen(5, 5).into_iter().collect();
    let old_emk5: Vec<_> = emk_gen(5, 5).into_iter().collect();
    println!("new (5,5): {:?}", new_emk5);
    println!("old (5,5): {:?}", old_emk5);
}
