use blocks::*;

fn main() {
    let mut pfm = Polycube::new(1000, blocks::Dist::Uniform);

    pfm.shuffle(100000000);

    pfm.render();
}
