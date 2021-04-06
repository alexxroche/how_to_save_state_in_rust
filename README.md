N.B. Do NOT use this as an example of good code. (Probably full of bad habits!)
# Harnessing the power of Cunningham's Law 
# I thought that I would publish:

```bash
"How to save state in rust"
    ^
   not
```

In this example we have a program that (artificially) runs for a very long time.

We want to be able to halt the program at any time and with a last_gasp function
save internal state of a struct.

At start we look for a previously saved state.
If a saved state exists it is imported to
create persistence between runs.


## Things to consider:

* A race condition between the signal being sent and the struct in memory being changed
* Could we protect against the data race with a `let sig_watch: mut i8 = 0;` that is incremented to the value of the Signal that is received, and then the main work loop watches to see if it has been incremented, ensures that ST is stable in memory before ` sig_watch += 1;` to let the Signal trap know that it can safely write the data to disk?
* A better way to be able to add last_gasp as an external crate that could be instructed which struct(s) to save
* Could this be done using async
* Should this be done using async


## Things to ignore:

* The save file could be overwritten by another program if we run multiple copies of this program at the same time
* The questionable crate layout
* The ST struct
* The "work" being done.
* Can't use a database because that would be adding a bottleneck
* Can't write to disk with every work loop because the disk IO tanks performance

## Suggestions and varations are welcome

I'll try to have a new branch for each approach.
