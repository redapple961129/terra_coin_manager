terrad tx wasm instantiate 1 '{"cw20_addr":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}' --from test1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block


terrad query wasm contract terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5

terrad tx wasm execute terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"add_project":{"project_id":1, "project_wallet":"terra18vd8fpwxzck93qlwghaj6arh4p7c5n896x5"}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block

