# ppte
welcome guys to my github repo. this is ppte (not an acronym for anyting) and it will be a clone of the highly regarded puzzle mashup game.

https://erm.0000727.xyz is where the game is hosted

- build by using `build.sh`
- also cargo build `wserver`
- run the server in `/client_app_output/wserver`
- use some sort of webserver in `/client_app_output/` like `python -m http.server`
- this depends on having cargo, and `wasm-bindgen-cli` and `python` and `flask`
- the build system for wasm is really scuffed

the plan is to have a webgame that is similar to tetrio but with more block stacking puzzle games. currently working on the buyo game.

# why wasm
i hate javascript, no javascript will every be committed to this repo

# why python server
the brains of the app should be in the webapp, and if theres anything that runs too slow in python, i'll just make a rust module for python
