terrad tx wasm store artifacts/terra_coin_manager.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block

terrad tx wasm instantiate 3 '{"cw20_addr":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}' --from test1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block


terrad query wasm contract terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt

terrad tx wasm execute terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt '{"add_project":{"project_id":"2", "project_wallet":"Hello"}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block

terrad query wasm contract-store terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt '{"get_project":{"id":"2"}}'

