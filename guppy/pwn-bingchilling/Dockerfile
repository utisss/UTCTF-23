FROM ubuntu:20.04

RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends tzdata
RUN apt-get update && apt-get install -y socat libseccomp-dev libglib2.0-dev

ARG FLAG
ARG USER
ENV USER $USER
ENV FLAG $FLAG

WORKDIR /
COPY start.sh /start.sh
RUN chmod 755 /start.sh

RUN useradd -m $USER

RUN echo "$FLAG" > /home/$USER/flag.txt
RUN chown root:root /home/$USER/flag.txt
RUN chmod 644 /home/$USER/flag.txt

EXPOSE 9000

WORKDIR /build
COPY build/. .

WORKDIR /

CMD ["/start.sh"]
