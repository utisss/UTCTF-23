# Reading List
* **Event:** UTCTF 2023
* **Problem Type:** Reverse Engineering
* **Point Value / Difficulty:** Beginner
* **(Optional) Tools Required / Used:** `strings` or Ghidra

## Steps​
#### Step 1
The prompt mentions that the binary contains some strings. Running the program gives the user a choice of which string to print. However, none of the strings that the program prints actually contains the flag.

#### Step 2
To find the flag, you can run `strings readingList | grep utflag` to print it to the terminal. If it's already open in Ghidra to try to reverse engineer the program, there is a built in string finder tool as well.