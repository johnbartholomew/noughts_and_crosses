# Noughts and crosses (tic-tac-toe) solver

This is my first Rust program after a day of reading ‘[The Book](https://doc.rust-lang.org/stable/book/)’. It solves noughts and crosses and outputs the result:

```
Analysed 38856 games in 1772 microseconds
Noughts and crosses is a draw with perfect play
```

I’m sharing it on GitHub because I would appreciate feedback on whether I’m doing things in the proper Rust way, or whether I’m transferring habits from JavaScript and PHP that aren’t very Rust-like. In particular:

- Is there a better way of structuring any element of the program using Rust features I haven’t learned yet?
- Am I commenting things appropriately?
- Am I testing things appropriately? (Although this isn’t Rust-specific, what is the appropriate way of testing the `solve` function, given the purpose of the program is to find what result it will give so the tests shouldn’t assume a particular result?)

Please make suggestions (which I’ll then try to implement as part of my learning process) through GitHub issues or by messaging me on Mastodon (@kate@fosstodon.org).
