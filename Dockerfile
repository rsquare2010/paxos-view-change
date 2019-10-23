FROM ubuntu:latest

RUN apt-get update
RUN apt-get install -y gcc
RUN apt-get install curl -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
ENV PATH="/root/.cargo/bin:${PATH}"
ADD ./src /src/
WORKDIR /src/
RUN rustc main.rs

ENTRYPOINT /src/main