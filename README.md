#### websocket example

| echo server

```
cargo run server --release
```

Launch Firefox web console

```
const ws = new WebSocket('ws://localhost:8080/echo/');

ws.onmessage = function(message) {
  console.log(message.data);
}

ws.onclose = function(message) {
  console.log("Code:", message.code);
  console.log("Reason:", message.reason);
  console.log("Clean:", message.wasClean);
}

ws.send('hi!');
ws.close(1000, 'bye!');

```
