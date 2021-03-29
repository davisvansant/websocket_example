<template>
  <img alt="Rusty!" src="./assets/error.png">
  <div id="app" style="
      background-color:black;
      width: 800px;
      height: 400px;
      padding: 25px;
      margin: 25px;
      box-shadow:3px 3px 10px 0 rgba(0, 0, 0, 0.75);
      font-family:Courier New;
      color: white;"
      >
      <p style="color:DarkOrange">|||||| websocket client | {{ url }} |<button style="background-color:Black;border:1px solid DarkOrange;padding: 5px 75px;outline:none;margin: 5px 5px;font-family:Courier New;color:PapayaWhip;" v-on:click="close()">CLOSE CONNECTION</button>|</p>
      <p style="color:DarkOrange">||||| connection status |<span style="background-color:DarkOrange;padding: 2px 109px;outline:none;margin: 2px 2px;font-family:Courier New;color:Black;">{{ status }}</span>|</p>
      <hr style="height:1px;background-color:DimGrey">
      <!-- <p style="color:DarkOrange">|||| connection status  {{ status }}</p>
      <p style="color:DarkOrange">||| connection url  {{ url }}</p> -->
      <!-- <button style="background-color:black;border:1px solid DarkOrange;padding: 10px 10px;outline:none;margin: 5px 5px;font-family:Courier New;color:DarkOrange;" v-on:click="join()">join</button>
      <button style="background-color:black;border:1px solid DarkOrange;padding: 10px 10px;outline:none;margin: 5px 5px;font-family:Courier New;color:DarkOrange;" v-on:click="statuscheck()">status</button> -->
      <p v-for="message in this.rx" :key="message" style="color:DarkOrange">|<span style="background-color:Black;padding: 2px 10px;outline:none;margin: 2px 2px;font-family:Courier New;color:DarkOrange;">{{ message }}</span></p>
      <input v-model="message" id="send_message" placeholder=">" autofocus=true size="110"/>
      <button onclick="document.getElementById('send_message').value = ''" style="position: bottom;background-color:black;border:1px solid DarkOrange;padding: 10px 10px;outline:none;margin: 5px 5px;font-family:Courier New;color:DarkOrange;" v-on:click="send()">send</button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'app',
  data () {
    return { connection: WebSocket.prototype, status: '', url: '', message: '', rx: [''] }
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

    websocket.addEventListener('message', (event) => {
      console.log(event)
      console.log('Message from server ', event.data)
      // this.receive(event)
      this.dosomething(event.data)
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
    dosomething (something: string) {
      this.rx.push(something)
      return this.rx
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
