<template>
  <img alt="Rusty!" src="./assets/error.png">
  <div id="console">
    <p class="dark_orange">|||||| websocket client | {{ url }} |
      <button class="close_connection_button" v-on:click="close()">
        CLOSE CONNECTION
      </button>|
    </p>
    <p class="dark_orange">
      ||||| connection status |
      <span class="connection_status">{{ status }}</span>
      |
    </p>
    <hr class="hr">
    <div id="rx_messages" class="rx_messages">
      <p v-for="message in this.rx_messages" :key="message" class="dark_orange">
      |
      <span>{{ message }}</span>
      </p>
    </div>
    <input
      class="tx_message"
      v-model="tx_message"
      id="tx_message"
      type="text"
      placeholder=">"
      autofocus=true
      size="110"
      @keyup.enter="send()"
    />
    <button class="send_button" v-on:click="send()">
      send
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'app',
  data () {
    return {
      connection: WebSocket.prototype,
      status: '',
      url: '',
      tx_message: '',
      rx_messages: ['']
    }
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

      this.collect_message(event.data)
    })

    websocket.addEventListener('close', (event) => {
      console.log('The connection has been closed successfully.', event)

      this.statuscheck()
    })
  },
  methods: {
    send () {
      this.connection.send(this.tx_message)
      this.tx_message = ''

      return this.tx_message
    },
    collect_message (message: string) {
      this.rx_messages.push(message)

      return this.rx_messages
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
    },
    message_scroll () {
      const element = document.getElementById('rx_messages')

      if (element !== null) {
        element.scrollTop = element.scrollHeight
      }
    }
  },
  updated () {
    this.$nextTick(() => this.message_scroll())
  }
})
</script>

<style>
  body {
    background: PapayaWhip;
  }
  #console {
    background-color:black;
    width: 800px;
    height: 400px;
    padding: 25px;
    margin: 25px;
    box-shadow:3px 3px 10px 0 rgba(0, 0, 0, 0.75);
    font-family:Courier New;
    color: white;
  }
  .dark_orange {
    color: DarkOrange;
  }
  .close_connection_button {
    background-color:Black;
    border:1px solid DarkOrange;
    padding: 5px 75px;
    outline:none;
    margin: 5px 5px;
    font-family:Courier New;
    color:PapayaWhip;
    transition: background-color 1s;
  }
  .close_connection_button:hover {
    background-color: DarkOrange;
  }
  .connection_status {
    background: linear-gradient(90deg, Black, DimGrey, Black);
    background-size: 800% 800%;
    width: 250px;
    display: inline-block;
    text-align: center;
    outline:none;
    border:1px solid DarkOrange;
    margin: 2px 2px;
    font-family:Courier New;
    color:PapayaWhip;
    animation: status_animation 15s ease-in-out infinite;
  }
  @keyframes status_animation {
    0% {
        background-position: 0% 50%;
    }
    50% {
        background-position: 100% 50%;
    }
    100% {
        background-position: 0% 50%;
    }
}
  .hr {
    height:1px;
    background-color:DimGrey;
  }
  .rx_messages {
    background-color:Black;
    padding: 2px 10px;
    outline:none;
    margin: 2px 2px;
    font-family:Courier New;
    color:DarkOrange;
    overflow-y: auto;
    height: 250px;
  }
  .tx_message:focus ~ .send_button {
    background-color: DarkOrange;
  }
  .send_button {
    position: bottom;
    background-color:Black;
    border:1px solid DarkOrange;
    padding: 10px 10px;
    outline:none;
    margin: 5px 5px;
    font-family:Courier New;
    color:PapayaWhip;
    transition: background-color 1s;
  }
  .send_button:hover {
    background-color: DarkOrange;
  }
</style>
