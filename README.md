# AUTOLANG

This Project is used to run Automations remote with a real language, not some Visual scripting.

# Building

## The language

To just build the languange for some reasong use

`cargo build -p lang`

To run it with an input file

`cargo run -p lang --bin run <filename>`

If you want to see all the tokens of an input file. (see also [#23](https://github.com/104-Berlin/autolang/issues/23))

`cargo run --bin tokens <filename>`

## Running the frontend for dev

### Prerequisites

- [Tailwind](https://tailwindcss.com/)

  Can be done via

  `npm install -g tailwindcss`

- [Trunk](https://trunkrs.dev/)

### Windows

To run the fronend on windows you unfortunately remove the pre_build hook from the [Trunk.toml](/frontend/Trunk.toml).

```
# [[hooks]]
# stage = "pre_build"
# command = "tailwindcss"
# command_arguments = [
#     "-i",
#     "input.css",
#     "-c",
#     "tailwind.config.js",
#     "-o",
#     "output.css",
# ]
```

We also need to start Tailwind manually

`tailwindcss -i input.css -o output.css --watch`

### Serving

`cd frontend`

`trunk serve`
