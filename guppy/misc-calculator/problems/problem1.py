import random
password = open("password.txt").read()
solution = random.getrandbits(32)

answer = input()
# No locals allowed!
result = eval(answer, {"open": None})

if result == solution:
    print(f"{result}, correct!  The password is '{password}'.")
else:
    print(f"Result: {result}.  The correct answer was {solution}.")
