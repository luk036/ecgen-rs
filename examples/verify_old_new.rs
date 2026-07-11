use ecgen::combin::emk_comb_gen;
use ecgen::combin_old::emk_gen;
use ecgen::set_partition::set_partition_gen;
use ecgen::set_partition_old::set_partition_gen_old;

fn main() {
    // EMK
    let new_emk: Vec<_> = emk_comb_gen(5, 2).into_iter().collect();
    let old_emk: Vec<_> = emk_gen(5, 2).into_iter().collect();
    println!("EMK(5,2) match: {} ({} vs {} items)", new_emk == old_emk, new_emk.len(), old_emk.len());

    let new_emk2: Vec<_> = emk_comb_gen(5, 3).into_iter().collect();
    let old_emk2: Vec<_> = emk_gen(5, 3).into_iter().collect();
    println!("EMK(5,3) match: {} ({} vs {} items)", new_emk2 == old_emk2, new_emk2.len(), old_emk2.len());

    let new_emk3: Vec<_> = emk_comb_gen(16, 5).into_iter().collect();
    let old_emk3: Vec<_> = emk_gen(16, 5).into_iter().collect();
    println!("EMK(16,5) match: {} ({} vs {} items)", new_emk3 == old_emk3, new_emk3.len(), old_emk3.len());

    // Set partition
    let new_sp: Vec<_> = set_partition_gen(5, 2).into_iter().collect();
    let old_sp: Vec<_> = set_partition_gen_old(5, 2).into_iter().collect();
    println!("SetPartition(5,2) match: {} ({} vs {} items)", new_sp == old_sp, new_sp.len(), old_sp.len());

    let new_sp2: Vec<_> = set_partition_gen(11, 5).into_iter().collect();
    let old_sp2: Vec<_> = set_partition_gen_old(11, 5).into_iter().collect();
    println!("SetPartition(11,5) match: {} ({} vs {} items)", new_sp2 == old_sp2, new_sp2.len(), old_sp2.len());
}
