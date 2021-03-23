<template>
  <img alt="Rusty!" src="./assets/error.png">
  <div id="console" style="
      background-color:black;
      width: 800px;
      height: 300px;
      padding: 25px;
      margin: 25px;
      box-shadow:3px 3px 10px 0 rgba(0, 0, 0, 0.75);
      font-family:Courier New;
      color: white;"
      >
      <p style="color:DarkOrange">|||||| websocket client</p>
    </div>
</template>

<script lang="ts">
const websocketUrl = 'ws://localhost:8888/echo/'
const websocket = new WebSocket(websocketUrl)

websocket.onmessage = function (onmessage) {
  console.log(onmessage.data)
}

websocket.onclose = function (onclose) {
  console.log(onclose)
}

websocket.onerror = function (onerror) {
  console.log(onerror)
}

function sendMessage (connection: WebSocket) {
  const message = 'hello!'
  connection.send(message)
}

function closeConnection (connection: WebSocket) {
  const reason = 1000
  const message = 'goodbye!'
  connection.close(reason, message)
}

console.log('Websocket status -', websocket.readyState)

websocket.addEventListener('open', (event) => {
  console.log(event)
  sendMessage(websocket)
})

websocket.addEventListener('message', function (event) {
  console.log(event)
  console.log('Message from server ', event.data)
  closeConnection(websocket)
})

export default {
  name: 'app'
}
</script>

<style>
  body {
    background: PapayaWhip;
  }
</style>
