![Screen Shot 2022-10-08 at 7 24 45 PM](https://user-images.githubusercontent.com/10949560/194731058-21ec39ac-7e6d-41ae-a192-9a894b1ceff1.png)
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

In action with the slower "render" mode:

![ezgif com-gif-maker](https://user-images.githubusercontent.com/10949560/194731222-e064426f-8062-4b7e-adff-b05912c98736.gif)
