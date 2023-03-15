# Hard Volatility

This challenge is very hard. It should take an experienced person several hours or more.

Intended solution:
 - Download the debug symbols for the kernel from the Fedora packages repository
 - Make your own profile
 - Write a volatility plugin to dump core of a specific process
 - Download the debug symbols for Filezilla from the packages repository 
 - Open the coredump in gdb using the debug symbols
 - Inspect the value of the password field

## FAQ

Please do not give any hints not listed here on this problem. If in doubt, tell them to ask Daniel.

### I got an answer that isn't being marked as correct.
 - Ask them for their answer
 - Compare it to the correct one
 - If it's close, tell them it's close
 - If not, tell them that they must not be looking at the correct string, and send a message to Daniel with what they thought it was

### I ran out of attempts!
Why did that happen? If they just didn't realize there was a limit, give them one more.

Otherwise, have Daniel look at. If it looks like they were guessing strings from memory, he will not give them more attempts.
