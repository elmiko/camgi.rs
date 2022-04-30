# camgi.rs

A tool for examining [OKD/OpenShift must-gather][mustgather] records
to investigate cluster autoscaler behavior and configuration.

## Quickstart

1. Install from crates.io with `cargo install camgi`
1. Have a must-gather ready to go in `$MUST_GATHER_PATH`
1. Run `camgi $MUST_GATHER_PATH > mustgather.html`
1. Open `mustgather.html` in a webbrowser

Your web browser should now show a page with a summary of the must-gather and some interactive navigation
buttons. If your browser does not open, you will see the URL printed on the terminal, copy it into a new
browser tab or window.

## History

This is a conversion of the [okd-camgi][okdcamgi] tool from Python into Rust.
The conversion is being written to make usage and distribution of the camgi
output easier for end users. For example, adding the camgi output into a
[must-gather][mustgather] requires chaning the underlying must-gather image
to allow for the dependencies required by the Python version. With a compiled
binary, the distribution process can be simpler by requiring only a download
of the binary executable.


[okdcamgi]: https://github.com/elmiko/okd-camgi
[mustgather]: https://github.com/openshift/must-gather
