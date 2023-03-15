# Medium Volatility
This challenge is a bit harder. It should take around an hour and a half for an experienced person who hasn't done volatility internals to solve.

Intended solution:
 - Get the kernel debug symbols from the Debian package repository
 - Build a profile yourself
 - Write a volatility plugin to read the uptime

The final step consists mostly of preparation and reading source code. The actual code size necessary is short, only about 4ish non-boilerplate executable lines.

## FAQ
For noobish questions, see the FAQ for the easy problem.

### There's no profile!
Get yer own 😛

### There's no plugin to get the uptime!
Indeed, there is not 😉

### The bash command doesn't work.
That's because there are no interactive instances of bash running lol

### Rounding???
Try it! If you're within ±2µs, it will accept it.

### I got a value that seems about right, but it's graded as wrong.
- Ask them for the number they got.
- Convert it from microseconds to seconds.
- If it's about an hour, then tell them that they might have found an unintended path that got a different answer, and send it and their username to Daniel so he can try to figure out what happened. Tell them to move on for now, and they'll hear back from him.
- If it isn't about an hour, then tell them that they must not have computed it correctly.
