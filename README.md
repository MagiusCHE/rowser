# Rowser
a Rust Browser

## Develop
- Requires: `rustc`,`yarn`
## Compile/Build Debug Mode
- Linux: `yarn build`
- Window: `yarn build:win`
## Run
- Linux: `yarn start`
  - This will build and run via `cargo run`
- Linux Wine: `yarn start:wine`
  - Will _not_ build but run target via `wine`. Requires `yarn build` and `wine` installed.