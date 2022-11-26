extern crate kiss3d;
extern crate rand;

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::mem;
use std::cmp;

use rand::Rng;
use rand::prelude::*;
use rand::distributions::{Bernoulli, Distribution};


// for rendering
use kiss3d::nalgebra::{Vector3, UnitQuaternion, Translation, Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use kiss3d::camera::FirstPerson;
use kiss3d::nalgebra::Point3;
use kiss3d::scene::SceneNode;
use kiss3d::window::State;


// wasm
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub enum Dist {
    Bernoulli(f64),
    Uniform
}

/// Represents a 3D Polyform
pub struct Polyform {
    
    // The actual polyform
    complex: HashSet<(i32, i32, i32)>,

    // Bookkeeping information to speed up operations on the polyform

    // Bounding box
    min_x: i32,
    max_x: i32,

    min_y: i32,
    max_y: i32,

    min_z: i32,
    max_z: i32,

    // keeps track of all empty locations strongly connected to a piece
    // beacuse this includes holes, we can't use this as a "tighter bounding box". There may be
    // other ways to use this information to speed up check validitiy, but for now
    insertable_locations: HashSet<(i32, i32, i32)>,

    // distribution
    dist: Dist,
}


// O(1)
fn get_neighbors(set: &HashSet<(i32, i32, i32)>, block: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut neighbors = Vec::<(i32, i32, i32)>::new();

    if set.contains(&(block.0, block.1, block.2 + 1)) {
        neighbors.push((block.0, block.1, block.2 + 1));
    }
    if set.contains(&(block.0, block.1, block.2 - 1)) {
        neighbors.push((block.0, block.1, block.2 - 1));
    }
    if set.contains(&(block.0, block.1 + 1, block.2)) {
        neighbors.push((block.0, block.1 + 1, block.2));
    }
    if set.contains(&(block.0, block.1 - 1, block.2)) {
        neighbors.push((block.0, block.1 - 1, block.2));
    }
    if set.contains(&(block.0 + 1, block.1, block.2)) {
        neighbors.push((block.0 + 1, block.1, block.2));
    }
    if set.contains(&(block.0 - 1, block.1, block.2)) {
        neighbors.push((block.0 - 1, block.1, block.2));
    }

    neighbors
}

// O(1)
fn get_vacant_neighbors(set: &HashSet<(i32, i32, i32)>, block: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut neighbors = Vec::<(i32, i32, i32)>::new();

    if !set.contains(&(block.0, block.1, block.2 + 1)) {
        neighbors.push((block.0, block.1, block.2 + 1));
    }
    if !set.contains(&(block.0, block.1, block.2 - 1)) {
        neighbors.push((block.0, block.1, block.2 - 1));
    }
    if !set.contains(&(block.0, block.1 + 1, block.2)) {
        neighbors.push((block.0, block.1 + 1, block.2));
    }
    if !set.contains(&(block.0, block.1 - 1, block.2)) {
        neighbors.push((block.0, block.1 - 1, block.2));
    }
    if !set.contains(&(block.0 + 1, block.1, block.2)) {
        neighbors.push((block.0 + 1, block.1, block.2));
    }
    if !set.contains(&(block.0 - 1, block.1, block.2)) {
        neighbors.push((block.0 - 1, block.1, block.2));
    }

    neighbors
}

// O(1)
fn has_neighbor(set: &HashSet<(i32, i32, i32)>, piece: &(i32, i32, i32)) -> bool {
        set.contains(&(piece.0, piece.1, piece.2+1))
                || set.contains(&(piece.0, piece.1, piece.2-1)) 
                || set.contains(&(piece.0, piece.1+1, piece.2)) 
                || set.contains(&(piece.0, piece.1-1, piece.2)) 
                || set.contains(&(piece.0+1, piece.1, piece.2)) 
                || set.contains(&(piece.0-1, piece.1, piece.2))
}

fn get_random(set: &HashSet<(i32, i32, i32)>) -> (i32, i32, i32) {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..set.len());

    let mut cur = 0;
    for i in set {
        if cur == index {
            return i.clone();
        }
        cur = cur + 1;
    }

    // unreachable
    panic!()
}

// O(1)
/// Used in DCUT algorithm, since we're looking for elements of ~P that preclude a strong
/// connection between two elements of P, ~P does not have to be strongly connected for P to not be
/// strongly connected. 
// Proof: Suppose two pieces are diagonal to each other after a corner piece is moved elsewhere. This piece should be cuttable,
// but P's compliment is not strongly connected through the corner, in fact P's compliment is
// diagonal. Therefore P is not strongly connected doesn't imply ~P is strongly connected
fn get_vacant_neighbors_dcut(set: &HashSet<(i32, i32, i32)>, block: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    // reminder to clean up the dcut algorithm main function
    todo!()
}

impl Polyform {

    // O(n^2) 
    // worst case:
    // T(n) = n + T(n-1)
    // best case:
    // T(n) = n + T(n-3)
    // early termination is possible but not considered for these "hand-wavy" computations
    /// Naive known-correct approach (trivial to prove correctness for yourself) for checking validity. Basically a BFS
    fn naive(&self) -> bool {
        let mut strongly_connected = HashSet::<(i32, i32, i32)>::new();
        let mut working_poly = self.complex.clone();


        while working_poly.len() > 0 {
            // things to remove from the working_poly after each pass
            let mut removals = Vec::<(i32, i32, i32)>::new();

            for piece in &working_poly {
                // if a neighbor is in strongly connected, append self
                if has_neighbor(&strongly_connected, &piece) {
                    removals.push(piece.clone());
                    strongly_connected.insert(piece.clone());
                }
                // if strongly_connected is empty append self
                else if strongly_connected.len() == 0 {
                    removals.push(piece.clone());
                    strongly_connected.insert(piece.clone());
                }
            }

            if removals.len() == 0 {
                return false;
            }

            for removal in &removals {
                working_poly.remove(&removal);
            }

            // check if we made progress. if we did a whole pass and didn't make progress, then we
            // are stuck
            // can do early termination if all removed's neighbors are added in the set
        }

        if strongly_connected.len() == self.complex.len() {
            return true;
        }

        return false;
    }

    /// naive with quick exit functions, relies on the fact that the previous polyform was valid
    fn semi_naive(&self, removed: &(i32, i32, i32)) -> bool {

        // if the piece never actually moved anywhere, i.e. if it was reinserted in the same spot,
        // the polyform is certainly still valid
        if self.complex.contains(&removed) {
            return true;
        }


        // if the removed polyform was an end piece, i.e. moving/removing it cannot disconnect two
        // parts of the polyform, i.e. a removing a leaf in a graph will not break the graph
        if get_neighbors(&self.complex, removed).len() == 1 {
            return true;
        }

        self.naive()
    }

    /// Stack + DFS
    fn dfs(&self) -> bool {
        let mut needs_neighbors_added = Vec::<(i32, i32, i32)>::new();
        let mut visited = HashSet::<(i32, i32, i32)>::new();

        let first = match self.complex.iter().next() {
            Some(first) => first,
            None => {
                // an empty polyform is a strongly connected polyform
                return true;
            }
        };

        needs_neighbors_added.push(first.clone());
        visited.insert(first.clone());

        while let Some(center) = needs_neighbors_added.pop() {
            // add all neighbors to the set and, if they haven't already been visited, to the stack
            let neighbors = get_neighbors(&self.complex, &center);
            for neighbor in neighbors {
                if visited.insert(neighbor) {
                    needs_neighbors_added.push(neighbor);
                }
            }
        }

        visited.len() == self.complex.len()
    }

    /// dcut algorithm I proposed earlier for check validity
    fn dcut(&self, through: &(i32, i32, i32)) -> bool {
        // terminates much quicker for known valid pieces and invalidity when P's complement isn't
        // super sparse
        //

        let mut last_ring = BTreeSet::<(i32, i32, i32)>::new();

        // while it is possible for us to make an entire ring that connects to itself and to the
        // previous ring, like an onion
        // and we aren't yet at the bounding box (in which case P's complement extends to infinity
        // and we can trivially show it's contiguous)
        loop {
            let next_ring = BTreeSet::<(i32, i32, i32)>::new();

            for ring_element in &last_ring {
                // get vacant moves (neighbor algorithm is different when we're cutting, since we
                // have to show the location of element in ~P precludes a strong connection between
                // parts of P)

                // wish to move outward towards the bounding box
                

            }

            last_ring = next_ring;
        }

        // last ring contains the outermost ring of the cut algorithm. Goal is to seek outward from
        // the innermost ring

        // get a stack
        // append the through piece
        //
        // for each item in the stack, pop and append ALL empty neighbors

        // check if all the pieces 
    }

    // each algorithm can run in its own thread, and perhaps can have multiple threads if we can
    // make it less serial

    // O(1)
    fn insert(&mut self, block: (i32, i32, i32)) -> bool {
        if block.0 < self.min_x {
            self.min_x = block.0;
        }
        if block.0 > self.max_x {
            self.max_x = block.0;
        }
        if block.1 < self.min_y {
            self.min_y = block.1;
        }
        if block.1 > self.max_y {
            self.max_y = block.1;
        }
        if block.2 < self.min_z {
            self.min_z = block.2;
        }
        if block.2 > self.max_z {
            self.max_z = block.2;
        }

        // if this piece was inserted at a border piece, remove that location from the border
        // because it's now occupied
        self.insertable_locations.remove(&block);

        // update the insertable_locations by inserting all empty locations strongly connected to
        // this inserted piece
        for neighbor in get_vacant_neighbors(&self.complex, &block) {
            self.insertable_locations.insert(neighbor);
        }

        self.complex.insert(block)
    }


    // a remove impl that maintains min_x max_x would run in at least O(n). If we don't force the
    // bounding box to be tight (which permits to not update the bounding box when an element is removed)
    //
    // A looser bounding box can have some small implications on the performance of checkvalidity (i.e. it must check cut through the piece and hit larger bounds), but
    // this increase in runtime would pale in comparison to linear time removals
    //
    // To preserve performance of the frequeny move operations
    // O(1)
    fn remove(&mut self, piece: &(i32, i32, i32)) -> bool {
        // in all cases this will be true, but keeping it here to keep the algorithm mostly
        // correct. Later on, we can pass an argument to the function that pre-empts the need for
        // this check, but this may only provide a negligble performance increase
        //
        // If the piece about to be removed is strongly connected, add the piece to the
        // insertable_locations
        if has_neighbor(&self.complex, &piece)
        { 
            self.insertable_locations.insert(piece.clone());
        }

        let removal = self.complex.remove(&piece);
        
        // remove all strongly connected pieces of insertable_locations to the piece about to be
        // removed if those pieces. max|get_neighbors| = 6 so this loop runs in constant time 
        for border_neighbor in get_neighbors(&self.insertable_locations, &piece) {
            // if the empty piece that used to be strongly connected to this removed piece doesn't
            // have another part of the polyform it's strongly connected to, it's no longer an
            // insertable location
            if !has_neighbor(&self.complex, &border_neighbor) {
                self.insertable_locations.remove(&border_neighbor);
            }
        }

        removal
    }

    

    // O(n)
    /// Selects a random piece in the complex
    fn get_random(&self) -> (i32, i32, i32) {
        // once we can demonstrate that picking the first element of the set or randomly picking
        // spots in the table gets the sampling we want, we can make this function less silly.
        // this depends on how the underlying elements are hashsed. this naive impl however lets us
        // use BTreeMaps easily
        get_random(&self.complex)

    }

    // places a single polyomino on one of the border elements with equal probability
    fn insert_random(&mut self) -> (i32, i32, i32) {
        let r = get_random(&self.insertable_locations);
        self.insert(r);
        r
    }

    fn remove_random(&mut self) -> (i32, i32, i32) {
        let r = self.get_random();
        self.remove(&r);
        return r.clone();
    }

    // O(1)
    fn compute_probability(&mut self, x_perimeter: usize, y_perimeter: usize, p: f64) {
            let mut perimeter = y_perimeter - x_perimeter;
            let mut probability = ((1.0 as f64)-p).powf(perimeter as f64); 
            probability = if p > 1.0 {1.0} else {p};
            self.dist = Dist::Bernoulli(p);
        }


    // O(n)
    pub fn new(len: usize, dist: Dist) -> Polyform {
        let mut polyform = Polyform {
            complex: HashSet::new(),
            insertable_locations: HashSet::new(), // we could initialize this to be to origin but it doesn't matter
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
            dist,
        };

        for i in 0..len {
               polyform.insert((0, 0, i as i32));
        }

        polyform
    }

    // computes a tight bounding box in O(n)
    fn recompute_bounding_box(&mut self) {
        self.min_x = i32::MAX;
        self.min_y = i32::MAX;
        self.min_z = i32::MAX;
        self.max_x = i32::MIN;
        self.max_y = i32::MIN;
        self.max_z = i32::MIN;
        for piece in &self.complex {
            if piece.0 < self.min_x {
                self.min_x = piece.0;
            }
            if piece.0 > self.max_x {
                self.max_x = piece.0;
            }
            if piece.1 < self.min_y {
                self.min_y = piece.1;
            }
            if piece.1 > self.max_y {
                self.max_y = piece.1;
            }
            if piece.2 < self.min_z {
                self.min_z = piece.2;
            }
            if piece.2 > self.max_z {
                self.max_z = piece.2;
            }
        }
    }

    // this function is strongly based on the eaxmple in kiss3d's readme. Deprecated, use
    pub fn render(self) {
        self.render_shuffle(0, Some(0))
    }

    // this function is strongly based on the eaxmple in kiss3d's readme
    pub fn render_shuffle(self, shuffles_per_render: usize, stop_after: Option<usize>)  {
        let mut window = Window::new("Polyform");

        window.set_light(Light::StickToCamera);

        let mut max_dist = self.max_x - self.min_x;
        if self.max_y - self.min_y > max_dist {
            max_dist = self.max_y - self.min_y;
        }
        if self.max_z - self.min_z > max_dist {
            max_dist = self.max_z - self.min_z;
        }
        
        let eye = Point3::new((max_dist as f32)/2.0, (max_dist as f32)/2.0, (max_dist as f32)/2.0);
        let at = Point3::origin();
        let arcball = ArcBall::new(eye, at);

        //let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());


        let rs = RenderState {
            shuffles_per_render,
            stop_after,
            pfm: self,
            group: None,
            camera: arcball,
            total_shuffles: 0,
            exported: false
        };

        window.render_loop(rs)
    }
    

    pub fn shuffle(&mut self, times: usize) -> Option<((i32, i32, i32), (i32, i32, i32))> {
        let LEN: usize = self.complex.len();
        let mut LEN_X = self.insertable_locations.len();

        let mut last_shuffled = None;
        for _ in 0..times {
            let removed = self.remove_random();
            //println!("removed {:?}", removed);
            let inserted = self.insert_random();
            //println!("inserted {:?}", inserted);
            if self.complex.len() != LEN {
                println!("detected decrease in polyform size");
            }

            let mut distCheck = true;
            match self.dist{
                Dist::Bernoulli(probability) => {
                    
                    // bernoulli coin flip
                    self.compute_probability(LEN_X, self.insertable_locations.len(), probability);
                    let dist = Bernoulli::new(probability).unwrap();
                    let sample = dist.sample(&mut rand::thread_rng());

                    if !sample {
                        println!("Reversing operation");
                        self.insert(removed);
                        self.remove(&inserted);
                        distCheck = false
                    } else {
                        println!("Maintainig operation and updating p");
                        self.dist = Dist::Bernoulli(probability);
                        last_shuffled = Some((inserted, removed));
                    }
                }
                Dist::Uniform => ()
            }

            if distCheck {
                if !self.dfs() { //can use self.naive or self.semi_naive here instead
                    //println!("Reversing operation");
                    self.insert(removed);
                    self.remove(&inserted);
                } 
            }
        }

        last_shuffled
    }

    pub fn export_scad(&mut self) -> String {
        let mut scad = String::new();
        self.recompute_bounding_box();


        for piece in &self.complex {
            let centered = self.center(piece);
            scad.push_str(&format!("translate([{}, {}, {}]) cube([1.01, 1.01, 1.01]);\n", centered.0, centered.1, centered.2));
        }

        scad
    }

    pub fn export(&self) -> String {
        let mut export = String::from("[");

        // don't center in order to prevent floating point problems
        for piece in &self.complex {
            export.push_str(&format!("({}, {}, {}), ", piece.0, piece.1, piece.2));
        }

        export.push_str("]");

        export
    }

    pub fn export_analysis(&self) -> String {
        let mut export = String::from("3");

        // don't center in order to prevent floating point problems
        for piece in &self.complex {
            export.push_str(&format!("\n{} {} {} 1", piece.0, piece.1, piece.2));
        }

        export
    }

    pub fn center(&self, piece: &(i32, i32, i32)) -> (f32, f32, f32) {
        (piece.0 as f32 - (self.max_x as f32 - self.min_x as f32)/2.0 - self.min_x as f32 , piece.1 as f32 - (self.max_y as f32 - self.min_y as f32)/2.0 as f32 - self.min_y as f32, piece.2 as f32 - (self.max_z as f32 - self.min_z as f32)/2.0 - self.min_z as f32)

    }
}

struct RenderState {
    shuffles_per_render: usize,
    stop_after: Option<usize>,
    pfm: Polyform,
    group: Option<SceneNode>,
    camera: ArcBall,
    total_shuffles: usize,
    exported: bool
}

impl State for RenderState {
    fn cameras_and_effect_and_renderer(
            &mut self,
        ) -> (
            Option<&mut dyn kiss3d::camera::Camera>,
            Option<&mut dyn kiss3d::planar_camera::PlanarCamera>,
            Option<&mut dyn kiss3d::renderer::Renderer>,
            Option<&mut dyn kiss3d::post_processing::PostProcessingEffect>,
        ) {

        return (Some(&mut self.camera), None, None, None);
    }

    fn step(&mut self, window: &mut Window) {
        
        match self.stop_after {
            Some(stop_after) if stop_after <= self.total_shuffles => {
                if !self.exported {
                    println!("{}", self.pfm.export());
                    self.exported = true;
                }
                //window.close();
                return;
            },
            _ => ()
        }

        let last_shuffled = self.pfm.shuffle(self.shuffles_per_render);

        self.total_shuffles = self.total_shuffles + self.shuffles_per_render;
        eprintln!("Completed {} shuffles live", self.total_shuffles);

        let mut oldgroup = None;
        mem::swap(&mut oldgroup, &mut self.group);
        if let Some(mut oldgroup) = oldgroup {
            window.remove_node(&mut oldgroup)
        }

        let mut group = window.add_group();
        self.pfm.recompute_bounding_box();

        // in the future we can combine neighboring pieces for faster rendering
        for piece in &self.pfm.complex {
            let mut c = group.add_cube(1.0, 1.0, 1.0);
            c.set_color(0.2, 0.2, 0.6);

            if let Some(last_shuffled) = last_shuffled {
                if last_shuffled.0.0 == piece.0 && last_shuffled.0.1 ==piece.1 && last_shuffled.0.2 == piece.2 {
                    c.set_color(0.0, 1.0, 0.0);
                }
            }

            let centered = self.pfm.center(piece);

            // because we don't maintain strict bounds, this isn't a perfect translation. We could
            // recompute strict bounds
            c.append_translation(&Translation3::new(centered.0, centered.1, centered.2));
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(last_shuffled) = last_shuffled {
            let mut removed = group.add_cube(1.0, 1.0, 1.0);

            removed.set_lines_width(1.0);
            removed.set_surface_rendering_activation(false);

            removed.set_color(1.0, 0.0, 0.0);
            let centered_removed = self.pfm.center(&last_shuffled.1);
            removed.append_translation(&Translation3::new(centered_removed.0, centered_removed.1, centered_removed.2));
        }

        self.group = Some(group);

        
    }
}

#[wasm_bindgen(start)]
pub fn our_main() -> Result<(), JsValue> {
    let mut pfm = Polyform::new(100, Dist::Uniform);
    pfm.render_shuffle(10, None);
    Ok(())
}
