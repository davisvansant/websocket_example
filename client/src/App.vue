<template>
  <img alt="Vue logo" src="./assets/logo.png">
  <h1>Welcome to Your Vue.js + TypeScript App</h1>
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
