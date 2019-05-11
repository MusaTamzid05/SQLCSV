FROM rust:1.31

RUN mkdir -p /rust_pro
COPY . /rust_pro
WORKDIR  /rust_pro
RUN cargo build
CMD ["sh" , "./run.sh" ]
