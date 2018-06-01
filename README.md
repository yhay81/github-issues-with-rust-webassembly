# GitHub Issues with Rust WebAssembly

This was created during my time as a student at Code Chrysalis. This is the service for create resume image and post it on twitter.

## Demo Page

https://yhay81.github.io/github-issues-with-rust-webassembly/

This demo is hosted on github pages.
It serves 4 files in /docs in this repository.
(html, css, js, wasm)

## Getting started

### Prerequisites

`rustc 1.28.0-nightly`
`cargo 1.28.0-nightly`
`cargo-web 0.6.11`

### Build

If you want to run it in local and affect code change immidiately, run this:
`cargo web start --target wasm32-unknown-unknown`
(Runs an embedded web server serving the built project)

If you want to build and make files to deploy, run this:
`cargo web deploy --target wasm32-unknown-unknown`
(Deploys your project so that its ready to be served statically)

## Contributing

Feel free to make issues if there are any probrems or suggestions.
And pull requests are welcome.

## Author

[Yusuke Hayashi](https://github.com/yhay81)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Acknowledgments

* Thank you to [Code Chrysalis](https://www.codechrysalis.io/), without you this project wouldn't exist.
