<template>
  <img alt="Rusty!" src="./assets/error.png">
  <div id="app" style="
      background-color:black;
      width: 800px;
      height: 300px;
      padding: 25px;
      margin: 25px;
      box-shadow:3px 3px 10px 0 rgba(0, 0, 0, 0.75);
      font-family:Courier New;
      color: white;"
      >
      <p style="color:DarkOrange">|||||| websocket client |  {{ url }} | {{ status }} |<button style="background-color:black;border:1px solid DarkOrange;padding: 10px 10px;outline:none;margin: 5px 5px;font-family:Courier New;color:DarkOrange;" v-on:click="close()">close</button>|</p>
      <!-- <p style="color:DarkOrange">|||| connection status  {{ status }}</p>
      <p style="color:DarkOrange">||| connection url  {{ url }}</p> -->
      <!-- <button style="background-color:black;border:1px solid DarkOrange;padding: 10px 10px;outline:none;margin: 5px 5px;font-family:Courier New;color:DarkOrange;" v-on:click="join()">join</button>
      <button style="background-color:black;border:1px solid DarkOrange;padding: 10px 10px;outline:none;margin: 5px 5px;font-family:Courier New;color:DarkOrange;" v-on:click="statuscheck()">status</button> -->
      <input v-model="message" placeholder=">"/>
      <button style="background-color:black;border:1px solid DarkOrange;padding: 10px 10px;outline:none;margin: 5px 5px;font-family:Courier New;color:DarkOrange;" v-on:click="send()">send</button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
// const websocketUrl = 'ws://localhost:8888/echo/'
// const websocket = new WebSocket(websocketUrl)
//
// websocket.onmessage = function (onmessage) {
//   console.log(onmessage.data)
// }
//
// websocket.onclose = function (onclose) {
//   console.log(onclose)
// }
//
// websocket.onerror = function (onerror) {
//   console.log(onerror)
// }
//
// function sendMessage (connection: WebSocket) {
//   const message = 'hello!'
//   connection.send(message)
// }
//
// function closeConnection (connection: WebSocket) {
//   const reason = 1000
//   const message = 'goodbye!'
//   connection.close(reason, message)
// }
//
// console.log('Websocket status -', websocket.readyState)
//
// websocket.addEventListener('open', (event) => {
//   console.log(event)
//   sendMessage(websocket)
// })
//
// websocket.addEventListener('message', function (event) {
//   console.log(event)
//   console.log('Message from server ', event.data)
//   closeConnection(websocket)
// })

export default defineComponent({
  name: 'app',
  data () {
    return { connection: WebSocket.prototype, status: '', url: '', message: '' }
  },
  created () {
    const websocketUrl = 'ws://localhost:8888/echo/'
    const websocket:WebSocket = new WebSocket(websocketUrl)
    console.log('this is happening')
    this.connection = websocket
    this.status = this.statuscheck() || ''
    this.url = websocket.url
    websocket.addEventListener('open', (event) => {
      console.log('on open event', event)
      this.statuscheck()
    })
    websocket.addEventListener('close', (event) => {
      console.log('The connection has been closed successfully.', event)
      this.statuscheck()
    })
  },
  methods: {
    join () {
      const websocketUrl = 'ws://localhost:8888/echo/'
      const websocket:WebSocket = new WebSocket(websocketUrl)
      this.connection = websocket
    },
    send () {
      this.connection.send(this.message)
    },
    close () {
      this.connection.close(1000, 'goodbye!')
    },
    statuscheck () {
      console.log(this.connection.readyState)
      switch (this.connection.readyState) {
        case 0:
          this.status = 'CONNECTING'
          return this.status
        case 1:
          this.status = 'OPEN'
          return this.status
        case 2:
          this.status = 'CLOSING'
          return this.status
        case 3:
          this.status = 'CLOSED'
          return this.status
      }
    }
  }
})
</script>

<style>
  body {
    background: PapayaWhip;
  }
</style>
