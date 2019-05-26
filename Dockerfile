FROM debian:9.9 as build

WORKDIR /root

# this feels wrong i.e. I'd far prefer if rustup was installed via an os package
# installer
RUN apt-get update
RUN apt-get install curl -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

ENV PATH=/root/.cargo/bin:$PATH

COPY . /root

RUN cargo build --release

