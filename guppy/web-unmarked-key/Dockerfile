FROM node

ARG FLAG
ARG USER
ENV FLAG $FLAG
ENV USER $USER

EXPOSE 2222

WORKDIR /work
COPY ssh-server /work

RUN npm install
RUN useradd -m $USER
RUN chmod -R 755 /work
USER $USER


CMD [ "node", "." ]
