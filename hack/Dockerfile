FROM registry.ci.openshift.org/ocp/builder:rhel-8-golang-1.20-openshift-4.14

RUN wget https://sh.rustup.rs -O rustup.sh
RUN sh ./rustup.sh -y
RUN cp /root/.cargo/bin/* /usr/bin/
RUN rustup default stable
