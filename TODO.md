- better error handling
- better user flow

- NaiveAIPlayer implementation
  - do research on existing hanabi strategies
- tests
- client/server architecture and multiplayer
- make file config format more readable, add display type
- implement simple graphics

network design:
- players receive message with action updates via NetworkPlayer struct
- in first rev, copy game state across players
- one player is host, in charge of receiving and relaying messages

- tokio-proto/tokio-line

protocol:
- host player initializes game
- other players come online and establish a connection
- host player decides to start
- for each player:
  - host blocks, waiting for action to come in
  - all other players block waiting for a state update
