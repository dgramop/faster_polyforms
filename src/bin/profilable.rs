use blocks::*;

fn main() {
    let mut pfm = Polyform::new(1000, blocks::Dist::Uniform);

    pfm.shuffle(100000000);

    pfm.render();
}
