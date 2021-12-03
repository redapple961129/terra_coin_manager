terrad tx wasm store artifacts/terra_coin_manager.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block

terrad tx wasm instantiate 2 '{"cw20_addr":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}' --from test1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block


terrad query wasm contract terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4

terrad tx wasm execute terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4 '{"add_project":{"project_id":"1", "project_wallet":"terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5"}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block

terrad query wasm contract-store terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4 '{"get_project":{"id":"1"}}'

