# ppte
welcome guys to my github repo. this is ppte (not an acronym for anyting) and it will be a clone of the highly regarded puzzle mashup game.

https://erm.0000727.xyz is where the game is hosted

- build by using `build.sh`
- also cargo build `web`
- run the server in `/client_app_output/web`
- you need to be in this folder do `./web` for it to work
- this depends on having cargo, and `wasm-bindgen-cli` and `python`
- the build system for wasm is really scuffed

the plan is to have a webgame that is similar to tetrio but with more block stacking puzzle games. currently working on the buyo game.

# why wasm
i hate javascript, no javascript will every be committed to this repo

# 3ds port
i am now porting the game to 3ds, browser game is on hold

https://github.com/aerits/ds-ppte

- not sure if i can get websockets to work on 3ds
- if not, i will just have local multiplayer for 3ds
