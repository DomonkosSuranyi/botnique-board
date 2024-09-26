![Westiny logo](media/westiny_logo.png)
# westiny
This is a western topview sandbox game.  
The game is written fully in rust.

## usage

### server
Run:
`cargo run --release --bin westiny_server`

By default the server will be listening on `127.0.0.1:5745`.
To modify this address, edit `resources/server_address.ron`.

### client
Specify server address on client:
`export WESTINY_SERVER_ADDRESS=1.2.3.4:5745`

Run:
`cargo run --release --bin westiny_client`

Or a one-liner:
`WESTINY_SERVER_ADDRESS=1.2.3.4:5745 cargo run --release --bin westiny_client`

### running server and client on the same computer
Start the server with default address:
`cargo run --release --bin westiny_server`

Start the client with default address too:
`cargo run --release --bin westiny_client`  
Note, that `WESTINY_SERVER_ADDRESS` has not been set.

## configuration
### player name
The player's name is got from `USER` env var,
so to change the player's name start client with setting the `USER`:  
`USER="Clint Westwood" cargo run --release --bin westiny_client`

### Running multiple clients on the same computer
To try the game alone you might want to run two or more clients on the same computer.  
The server identifies a player by its name and its address.  
To be able to connect multiple clients those has to be started
with a different player name.

## gameplay
### controls
The player turns always towards the cursor. You can move with WASD keys relatively to facing direction.  
W - move forward  
S - move backward  
A - move left  
D - move right  
Left click - shoot  
1 - switch weapon to Revolver  
2 - switch weapon to Shotgun  
3 - switch weapon to Rifle  
R - reload  

### misc
The weapons have different parameters. They differ in damage, pellet number, spread, shoot distance, bullet speed.  
The player is respawned in a few seconds after death.
