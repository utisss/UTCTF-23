# What Time is It?

The key idea here is that email separates the different content types in a multipart email using boundary hashes The boundary hashes in the `phishing.eml` are listed below.

1. 00000000000093882205f60cdcdba

These boundary hashes must be unique. The timestamp at which the boundary was generated is stored in the boundary itself to make the possibility of a collision much rarer. We can extract the timestamp from this boundary hash to get insight into when this email was actully sent. Note that the timestamp in the boundary hash is to be interpreted differently on Linux and Windows systems. On Linux systems, the timestamp represents the time passed since the Linux epoch whereas on Windows systems, the timestamp represents the number of 100 nanosecond intervals since the Windows epoch. In this case, we were told that a Linux server processed this email.

Details on how to actually extract the timestamp from the boundary hash are explained well in the following article.

[https://www.metaspike.com/gmail-mime-boundary-delimiter-timestamps/](https://www.metaspike.com/gmail-mime-boundary-delimiter-timestamps/)
