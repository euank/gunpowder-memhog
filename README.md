# Gunpowder Memhog

This is a memory hog program. It differs from other memhog programs in that it
allows increase of memory over time in discrete steps.

It's also far more safe because it's written in Rust.

# Usage

```
Usage:
    ./target/release/gunpowder-memhog [OPTIONS] [FINAL MEMORY]

Take up memory

positional arguments:
  final memory          Memory to reach after timeout

optional arguments:
  -h,--help             show this help message and exit
  -m,--start-memory START_MEMORY
                        Starting memory
  -e,--exit-timeout EXIT_TIMEOUT
                        How long to run until exiting in seconds (0 for
                        forever)
  -t,--step-timeout STEP_TIMEOUT
                        How long to take to increase memory for from initial ->
                        final in seconds
```

# FAQ

### Do you really think it's more safe because it's in rust?

No

### Why not use firefox?

I needed a minimum memory usage of under 150M

### You know you don't need unsafe blocks for this, right?

Yeah. See the included license for why I felt they were necessary.

### You lied, it takes up more memory than you said!

Not a question, but the program does take around 12M fairly consistently for me
apart from the intended resource wasting. Maybe that'll go down over time as
the rust runtime, insofar as it has a runtime, thins down.

### I have a contribution; will you take it?

If it follows the license, probably!

### I see you have a Dockerfile

See [euank/gunpowder-memhog](https://registry.hub.docker.com/u/euank/gunpowder-memhog/).
