import random
password = open("password.txt").read()
solution = random.getrandbits(32)

answer = input()
result = eval(answer)

if result == solution:
    print(f"{result}, correct!  The password is '{password}'.")
else:
    print(f"Result: {result}.  The correct answer was {solution}.")
