#### websocket example

An example websocket server and client

###### | server

Built with
  - https://crates.io/crates/actix-web-actors
  - https://crates.io/crates/actix
  - https://crates.io/crates/actix-web

To run - `cargo run server --release`

###### | client

Built with

 - https://v3.vuejs.org/

To run (currently) -
```
cd client
npm run serve
```

Browse to the following - `http://localhost:8080/`

Once the server and client are up, you can open another browser window and try something like the following:

```
const ws = new WebSocket('ws://localhost:8888/echo/');

ws.onmessage = function(message) {
  console.log(message.data);
}

ws.onclose = function(message) {
  console.log(message.data);
}

ws.send('hi!');
ws.close(1000, 'bye!');

```
