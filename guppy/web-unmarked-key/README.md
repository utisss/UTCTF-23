# Unmarked Key

## Problem Details

Basically, there's supposed to be a TLS server running on the "machine", but it's down. You have SSH access (but no support for running commands), the TLS private key, and a part of the certificate, but you don't know the expected domain name.

I had to clarify this as a hint later, but there are periodic requests made over TLS to port 443 with the flag.

## Technical Details

This isn't an actual SSH server - it's a custom implementation on Node.js. Commands aren't implemented. The one thing that is implemented is remote port forwarding - if you forward something from remote port 443, it'll get pinged repeatedly with a TLS connection.

The target domain name is reserved by me (Jonathan Browne / JBYoshi) via FreeDNS, so you can't issue certificates for it.

Technically it's being hosted on the same box as everything else, so you can immediately get to the SSH server and start receiving TLS traffic, but you'd get stuck because you don't have the certificate you need. SNI is turned off, so you can't learn the right domain name from that.

## Expected Solution

The main challenge is figuring out how to reconstruct the certificate. You don't have the domain name, so you can't simply guess that. However, the chunk of the certificate you do have (if you try base64-decoding it) mentions Let's Encrypt, so it probably came from an actual CA. That means it should be in the Certificate Transparency logs somewhere.

My personal preferred site for CT searching is [crt.sh](https://crt.sh). By default it prompts you to search by domain name or fingerprint (neither of which is present in the part of the certificate you got), but the advanced search option also lets you search by a hash of the "Subject Public Key Info" - which is just the DER-encoded version of the public key. The OpenSSL command line tools let you grab this:

```sh
$ openssl pkey -in privkey.pem -outform der -pubout | sha256sum
783fb36c559f93d0e8e57af8825506294e8b23a8fc31f13f7fed061c63a22da2
```

There are two entries listed there. One is the [Certificate Transparency precertificate](https://crt.sh/?id=8664428056), which is used for Certificate Transparency signing purposes but is [not recognized by TLS client libraries](https://github.com/google/certificate-transparency/blob/master/docs/SCTValidation.md#precertificates). If you grab that, it's possible to fuse the data from it with the chunk of the certificate we have, though you also need to change a few size fields (otherwise you'll get an error like "CRYPTO_internal:too long").

Fusing the two pieces was my intended solution, but only since the next entry hadn't shown up until a few hours after I finished testing the problem. That other entry is [the actual certificate](https://crt.sh/?id=8743563548). As far as I'm aware, pretty much all competitors just used that.

(Some competitors also used [Censys](https://search.censys.io). However, [Censys only picked up the precertificate for some reason.](https://search.censys.io/certificates-legacy/20ed536cc5326c0a0d0e7820709977141a2d8ba16fac77bd169afaeaeba5d538))

One other thing to get a TLS server working: we need to specify the full certificate chain. According to crt.sh, this certificate was signed by Let's Encrypt R3, which can be downloaded from the [Let's Encrypt website](https://letsencrypt.org/certificates/). If necessary, we could also add ISRG Root X1 (which signed that), but that happens to already be in the server's root store.

From there, we need to get a TLS server connected to the SSH server. Unfortunately, there doesn't appear to be any way to run commands, but it is possible to do remote port forwarding using `ssh -R 443:localhost:<local port>`. We just need to set up a TLS server using our merged-and-fixed certificate and the private key, and it all works.

## Questions asked and/or anticipated

### The SSH server can be accessed from utctf.live, so you don't need to look up the domain to connect to it. Isn't that a cheese?

No - you still need to figure out the domain to get the certificate. (See the next question.)

### Once you get the TLS server set up, can't you get the domain from SNI?

The server is explicitly set to not use SNI, so that's not an issue.

The reason I had to do this was due to CTF-side technical limitations. For UTCTF, we prefer to put several problems on each of our VMs, and if someone happened to portscan guppy.utctf.live, they would have seen that it happened to have the problem's port open, connect, set up a TLS receiving socket, and get the SNI data. That would make it much easier to look up the certificate data than I intended.

In my original idea, the challenge would have its own IP address, and you wouldn't be given that IP in the challenge; you'd just get the port and have to do the Certificate Transparency search first to figure out what that IP is. That setup would mean that once you got the IP, you'd already know the domain name from your search process, so SNI wouldn't have to be disabled in that case.

### I found the precertificate from Certificate Transparency and combined it with the part from the problem, but it isn't working?

You can't just copy over the bit from the problem. Well, you need to copy it over, but you'll also need to tweak a few other parts of the certificate.

### The certificate is revoked?

Yes. I did that intentionally just since I am in fact sharing a private key with the world. The server doesn't use OCSP, so that shouldn't be a problem. (If there is a problem, it might be caused by something else - see below.)

In retrospect though, this just served to confuse a few people who had an error with their setup and blamed it on the revocation.

### What's the "UNABLE_TO_VERIFY_LEAF_SIGNATURE" error?

There are two common reasons for this:

* You used a self-signed certificate. The server doesn't allow those.
* You used the right certificate but didn't include the full chain in your TLS configuration. The server only knows about root certificates, not intermediate certificates. You'll need to configure your server to also send the intermediate certificates in the chain.

### I have a certificate but it's invalid

For convenience of officers looking at this, I'll give a few hints to visually identify common problems based solely on the base64-encoded representation.

* Starts with `MIIEbTCC`, ends with `fFA97DzQ`: That's not a valid certificate. That's called a precertificate, and it's specifically designed to only be valid for Certificate Transparency logs. You'll need to find a way to recreate the full certificate.
* Starts with `MIIEbTCC`, ends with `xD1xnRQ=`: You still need to tweak a few fields in the certificate to make it valid. (Officer: see the third-last paragraph of the explanation. Simply changing these first couple bytes isn't quite enough.)
* Starts with `MIIFYjCC`, ends with `xD1xnRQ=`: This is probably the correct certificate. (Officer: see live/cert.pem for the correct certificate.)
* Starts with `MIIFYTCC`, ends with `q82vEg==` OR starts with `MIIEbTCC`, ends with `wracdIYn`: I accidentally generated another certificate for the same site. That's my fault, not theirs - give them full info here. Tell them to use the certificate that was generated earlier. (This is also the situation that applies if people mention they have both a revoked certificate that matches the private key and a valid certificate that doesn't match the private key. The revoked certificate is the correct one.)

Also note that self-signed certificates are not accepted. If the competitor mentions they've generated a certificate signing request (CSR), that's wrong.

## Notes for any ISSS officers who want to recycle this problem

* Reserve a fresh temporary domain (I used afraid.org, which provides free subdomain access). Pick something that you can't easily search for using your CTF name, ISSS, or problem name, but can still be identified as a target once you see it. Put that domain in the server script.
* Get a fresh certificate on the domain. If you reserve the domain early enough, it turns out that you don't actually need to publish the bit of the public certificate; the CT logs should pick up the real certificate in addition to the precertificate after a day or two.
* Even though CA policies say you should revoke a certificate if its public key gets exposed, don't actually do that (at least not until the CTF is over). It really just serves to confuse competitors.
* Generate a new a host key for the server (I removed the UTCTF host key before the challenge source was made public; check the private repository's commit history to find it).
* You should probably update the problem description to make it clearer that the flag is being sent to the server's (fake) port 443. Lots of competitors got stuck on that before I put up a hint for it later on.
