FROM adrianbrink/rust:nightly

VOLUME ["/source"]
WORKDIR /source
CMD ["/bin/bash"]