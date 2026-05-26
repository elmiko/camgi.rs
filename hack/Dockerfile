FROM registry.ci.openshift.org/ocp/builder:rhel-9-golang-1.25-openshift-4.22

RUN wget https://sh.rustup.rs -O rustup.sh
RUN sh ./rustup.sh -y
RUN cp /root/.cargo/bin/* /usr/bin/
RUN rustup default stable
