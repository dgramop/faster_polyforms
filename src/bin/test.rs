use blocks::*;
fn main() {
    let pfm = Polyform::new(3, Dist::Bernoulli ( 0.7 ));
    let before_perim = pfm.insertable_locations.len();
    let after_perim = pfm.insertable_locations.len();
    println!("Have {} insertable locations before", before_perim);
    pfm.render();
    //pfm.compute_probability(pfm.insertable_locations.len(), 
}
