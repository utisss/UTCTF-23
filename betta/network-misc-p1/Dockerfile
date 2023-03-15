FROM alpine:latest

RUN apk add --no-cache --update \
    samba-common-tools \
    samba-client \
    samba-server \
    && rm -rf /var/cache/apk/*
    
RUN apk --no-cache add socat

RUN apk add --no-cache openssh \
    && ssh-keygen -A 

RUN echo -n 'PasswordAuthentication yes' >> /etc/ssh/sshd_config

COPY smb.conf /etc/samba/smb.conf

EXPOSE 137/udp 138/udp 139 445 8080 8722

RUN adduser wcoldwater;echo 'wcoldwater:abracadabra$' | chpasswd

RUN echo 'utctf{cust0m3d-lsts-rule!} well done!' > /etc/motd

RUN echo 'Note to self: Get better at this security stuff.' > /home/wcoldwater/note.txt

CMD socat TCP-LISTEN:8080,fork,reuseaddr EXEC:'echo Hi Wade! I am using socat to broadcat this message. Pretty nifty right? --jwalker utflag{meh-netcats-cooler}',pty,echo=0 & smbd --no-process-group --foreground --debug-stdout & /usr/sbin/sshd -D -e

