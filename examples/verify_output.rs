use ecgen::{brgc_gen, ehr_gen, emk_comb_gen, set_bipart_gen, set_partition_gen, sjt_gen};

fn main() {
    println!("=== EMK combinations ===");
    let v: Vec<_> = emk_comb_gen(5, 2).into_iter().collect();
    println!("emk_comb_gen(5,2) ({} items): {:?}", v.len(), v);
    let v: Vec<_> = emk_comb_gen(5, 3).into_iter().collect();
    println!("emk_comb_gen(5,3) ({} items): {:?}", v.len(), v);

    println!("\n=== SJT permutations ===");
    let v: Vec<_> = sjt_gen(3).into_iter().collect();
    println!("sjt_gen(3) ({} items): {:?}", v.len(), v);
    let v: Vec<_> = sjt_gen(4).into_iter().collect();
    println!("sjt_gen(4) ({} items): {:?}", v.len(), v);

    println!("\n=== Ehrlich permutations ===");
    let v: Vec<_> = ehr_gen(3).into_iter().collect();
    println!("ehr_gen(3) ({} items): {:?}", v.len(), v);
    let v: Vec<_> = ehr_gen(4).into_iter().collect();
    println!("ehr_gen(4) ({} items): {:?}", v.len(), v);

    println!("\n=== BRGC ===");
    let v: Vec<_> = brgc_gen(3).into_iter().collect();
    println!("brgc_gen(3) ({} items): {:?}", v.len(), v);

    println!("\n=== Set partition ===");
    let v: Vec<_> = set_partition_gen(5, 2).into_iter().collect();
    println!("set_partition_gen(5,2) ({} items): {:?}", v.len(), v);

    println!("\n=== Set bipartition ===");
    let v: Vec<_> = set_bipart_gen(5).into_iter().collect();
    println!("set_bipart_gen(5) ({} items): {:?}", v.len(), v);
}
