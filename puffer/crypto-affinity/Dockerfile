FROM ubuntu:20.04

RUN apt-get update
RUN apt-get update && apt-get install -y build-essential socat libseccomp-dev python3

ARG USER
ENV USER $USER

WORKDIR /
COPY start.sh /start.sh
RUN chmod 755 /start.sh

RUN useradd -m $USER

EXPOSE 9000

WORKDIR /home/$USER
COPY ./encrypt.py ./encrypt.py
COPY ./aes.py ./aes.py
RUN chmod +x ./encrypt.py && chown $USER:$USER ./encrypt.py

CMD ["/start.sh"]
