# camgi.rs

This is a conversion of the [okd-camgi][okdcamgi] tool from Python into Rust.
The conversion is being written to make usage and distribution of the camgi
output easier for end users. For example, adding the camgi output into a
[must-gather][mustgather] requires chaning the underlying must-gather image
to allow for the dependencies required by the Python version. With a compiled
binary, the distribution process can be simpler by requiring only a download
of the binary executable.


[okdcamgi]: https://github.com/elmiko/okd-camgi
[mustgather]: https://github.com/openshift/must-gather
