N.B. Do NOT use this as an example of good code. (Probably full of bad habits!)
# Harnessing the power of Cunningham's Law I thought that I would publish:

```bash
"How to save state in rust"
    ^
   not
```

This example (tries to) uses https://doc.rust-lang.org/std/ops/trait.Drop.html
to only save the data structure when it is going to be dropped.

## Question
* Is this going to be an issue because we have a nested reflexive strust that is mostly comprised of versions of itself?
