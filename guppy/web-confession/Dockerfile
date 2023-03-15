FROM ubuntu:20.04

RUN apt-get update
RUN apt-get update && apt-get install -y build-essential socat libseccomp-dev python3 python3-pip
RUN pip install sqlparse flask openai

COPY . .
RUN ls
CMD ["python3","main.py"]
