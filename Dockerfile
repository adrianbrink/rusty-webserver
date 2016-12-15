FROM adrianbrink/rust:stable

VOLUME ["/source"]
WORKDIR /source
CMD ["/bin/bash"]