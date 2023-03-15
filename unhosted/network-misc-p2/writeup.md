 # Problem Title A Network Problem
 * **Event:** StudioGhibliCTF
 * **Problem Type:** Network
 * **(Optional) Tools Required / Used:** netcat
 
 ## Steps

 #### Step 1
nmap network.utctf.live -p 445

 #### Step 2
 list the shares:
 smbmap -H network.utctf.live (shows us we can access WorkShares as a guest)
 
 #### Step 3 
 Grab all the file (you could search them individually, but this is fast)
 smb> recurse ON
 smb> prompt OFF
 smb> mget *
 
 #### Step 4
 List the files in tree style:
 tree shares
 
 #### Step 5
 cat /shares/IT/Itstuff/notetoIT
