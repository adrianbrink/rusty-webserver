FROM adrianbrink/rust:nightly

RUN apt-get update && \
    apt-get install -y sqlite libsqlite3-dev
RUN apt-get install -y libpq-dev
RUN cargo install diesel_cli
EXPOSE 4443
VOLUME ["/source"]
WORKDIR /source
CMD ["/bin/bash"]