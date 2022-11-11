# Releasing Camgi

Camgi is used by the OpenShift CI infrastructure (see [gather-must-gather-commands.sh](https://github.com/openshift/release/blob/master/ci-operator/step-registry/gather/must-gather/gather-must-gather-commands.sh)),
as such it should be released in a manner that can be pulled by that tooling.
To create a release do the following:

1. update Cargo.toml and Cargo.lock
2. create a new release tag and commit, make sure to sign the tag
3. use a builder image from current OpenShift to create the binary, we use an image because it needs to match the run environment that OpenShift CI will use.
4. create a release tar file with the binary and a sha256 sum, name it `camgi-<version>-linux-x86_64.tar`
5. make a new release on github with the tar file artifact.

Look at the `hack/Dockerfile` to see how the builder image is created.
