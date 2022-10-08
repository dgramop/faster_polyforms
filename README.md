# MEGL Polyforms: Rust Version

Here is an accelerated version of what was originally prototyped in the Colab, with a few improvements

Speed:
- Incremental computation of the set of possible locations to insert a piece such that it's strongly connected to something
- Early termination for the naive algorithm
- (soon: cut algorithm and threading)

Quality of life:
- Live rendering
- Easy CLI configuration

What if we had one thread find all contiguous blocks on the left side of center, another thread find all contiguous blocks right of center, and then see if each contiguous block to the left is contiguous with a block on the right at the end. 
