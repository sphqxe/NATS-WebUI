<template>
  <el-container style="display: flex; flex-direction: row;">
    <el-container style="flex: 1 0 50%;">
      <el-header style="border-bottom: 1px solid #e6e6e6; text-align: left; display: flex; flex-direction: column; padding: 8px; height: auto;">
        <div style="display: flex; flex-direction: row; margin-bottom: 8px;">
          <el-input placeholder="Client Name" v-model="client.name" style="margin-right: 8px; flex: 1 1 30%;"></el-input>
          <el-select v-model="client.server_id" placeholder="Choose a server" style="margin-right: 8px; flex: 1 1 100%;" @change="handleSelectServer">
            <el-option v-for="server in servers" :key="server.id" :label="server.name" :value="server.id" :value-key="server.id">
            </el-option>
          </el-select>
          <el-button v-if="this.socket === null" round plain @click="connect">
            <font-awesome-icon :icon="['fas', 'link']"/>
          </el-button>
          <el-button v-if="this.socket !== null" type="success" round @click="disconnect">
            <font-awesome-icon :icon="['fas', 'unlink']"/>
          </el-button>
        </div>
        <div style="flex: 1 1 auto; display: flex; flex-direction: row; margin-left: 8px;">
          <el-checkbox label="Info" v-model="client.info"></el-checkbox>
          <el-checkbox label="Ping" v-model="client.ping"></el-checkbox>
          <el-checkbox label="Pong" v-model="client.pong"></el-checkbox>
          <el-checkbox label="OK" v-model="client.ok"></el-checkbox>
          <el-checkbox label="Err" v-model="client.err"></el-checkbox>
          <el-checkbox label="Msg" v-model="client.msg"></el-checkbox>
        </div>
      </el-header>
      <el-main style="padding: 0px;">
        <!-- <div style="height: 100%; overflow-y: auto;">
          <div v-for="(message, idx) in messages" :key="idx" style="font-size: small; text-align: left; border-bottom: 1px solid rgb(245, 245, 245);">
            {{ message }}
          </div>
        </div> -->

        <el-table id="message-log" :data="messages" height="100%" style="width: 100%" fit>
          <div slot="empty">
            No messages to show.
          </div>
          <el-table-column prop="timestamp" label="Timestamp" width="160"></el-table-column>
          <el-table-column prop="typ" label="Type" width="60"></el-table-column>
          <el-table-column prop="subject" label="Subject" width="240" resizable></el-table-column>
          <el-table-column prop="message" label="Message" resizable></el-table-column>
        </el-table>
      </el-main>
    </el-container>
    <el-aside style="border-left: 1px solid #e6e6e6; flex: 1 0 120px;">
      <h1>Server Subject Hierarchy</h1>
      <el-tree ref="tree" :data="server.subjects" empty-text="No subjects configured for this server." default-expand-all node-key="subject_str"
        :props="{label: 'subject_str', children: 'subjects', disabled: false, isLeaf: checkIsLeaf}" show-checkbox @check="handleCheckChange" :default-checked-keys="checkedKeys">
      </el-tree>
    </el-aside>
  </el-container>
</template>

<script>
import { mapState, mapActions } from 'vuex'
import ReconnectingWebSocket from 'reconnecting-websocket'
import moment from 'moment'

var Deque = require("double-ended-queue")

export default {
  data: () => {
    return {
      msgTypes: [],
      value: 1,
      data: [{
        id: 1,
        label: 'Level one 1',
        children: [{
          id: 4,
          label: 'Level two 1-1',
          children: [{
            id: 9,
            label: 'Level three 1-1-1'
          }, {
            id: 10,
            label: 'Level three 1-1-2'
          }]
        }]
      }, {
        id: 2,
        label: 'Level one 2',
        children: [{
          id: 5,
          label: 'Level two 2-1'
        }, {
          id: 6,
          label: 'Level two 2-2'
        }]
      }, {
        id: 3,
        label: 'Level one 3',
        children: [{
          id: 7,
          label: 'Level two 3-1'
        }, {
          id: 8,
          label: 'Level two 3-2'
        }]
      }],
      defaultProps: {
        children: 'children',
        label: 'label'
      },
      messages: [],
      socket: null,
      messageBuffer: new Deque(100)
    }
  },
  computed: {
    ...mapState({
      servers: s => s.app_state.servers,
      clients: s => s.app_state.clients,
      client: s => s.transient.clientsMap[s.transient.selectedClient],
      server: s => s.transient.serversMap[s.transient.clientsMap[s.transient.selectedClient].server_id],
      subjectRoots: function (s) {
        let server = s.transient.serversMap[s.transient.clientsMap[s.transient.selectedClient].server_id]
        let res = new Set()
        server.subjects.forEach(function (e) {
          res.add(e.subject_str)
        })
        return res
      },
      checkedKeys: function(s) {
        let subjects = s.transient.clientsMap[s.transient.selectedClient].subjects
        let res = []
        function getCheckedKeys(node) {
          if (node.subjects.length === 0) {
            res.push(node.subject_str)
          } else {
            for (var i in node.subjects) {
              let child = node.subjects[i]
              getCheckedKeys(child)
            }
          }
        }
        for (var i in subjects) {
          let node = subjects[i]
          getCheckedKeys(node)
        }
        return res
      }
    })
  },
  methods: {
    ...mapActions(['updateClient', 'getAppState']),
    selectServer(id) {
      this.value = id
    },
    connect() {
      this.socket = new ReconnectingWebSocket("ws://" + window.location.hostname + "/client/1")
      this.socket.addEventListener('message', this.handleSocketEvent)
    },
    disconnect() {
      this.socket.close()
      this.socket = null
    },
    handleSocketEvent(ev) {
      let data = JSON.parse(ev.data)
      this.lastTimestamp = data.timestamp
      data.timestamp = moment(data.timestamp).format("YYYY-MM-DD HH:mm:ss.SSS")
      if (this.messageBuffer.length == 100) {
        this.messageBuffer.shift()
      }
      this.messageBuffer.push(data)
    },
    async handleSelectServer(serverId) {
      console.log(serverId)
      let client = JSON.parse(JSON.stringify(this.client))
      client.subjects = []
      client.server_id = serverId
      await this.updateClient(client)
      await this.getAppState()
    },
    checkIsLeaf() {
      return false
    },
    async handleCheckChange() {
      console.log('handleCheckChange called')
      let halfChecked = new Set()
      this.$refs.tree.getHalfCheckedNodes().forEach(function (e) {
        halfChecked.add(e.subject_str)
      })
      let checked = new Set()
      this.$refs.tree.getCheckedNodes().forEach(function (e) {
        checked.add(e.subject_str)
      })
      let roots = JSON.parse(JSON.stringify(this.$refs.tree.getHalfCheckedNodes().filter(e => this.subjectRoots.has(e.subject_str))))
      function removeUnchecked(node) {
        if (halfChecked.has(node.subject_str)) {
          node.subjects = node.subjects.filter(e => halfChecked.has(e.subject_str) || checked.has(e.subject_str))
          for (var i in node.subjects) {
            let child = node.subjects[i]
            if (halfChecked.has(child.subject_str)) {
              removeUnchecked(child)
            }
          }
        }
      }

      roots.forEach(node => removeUnchecked(node))
      this.$refs.tree.getCheckedNodes().forEach(function (e) {
        if (this.subjectRoots.has(e.subject_str)) {
          roots.push(e)
        }
      }.bind(this))
      let client = JSON.parse(JSON.stringify(this.client))
      client.subjects = roots;
      await this.updateClient(client)
      await this.getAppState()
    }
  },
  mounted () {
    window.setInterval(function () {
      if (this.messageBuffer.length > 0) {
        if (this.messageBuffer.length === 100) {
          this.messages = this.messageBuffer.toArray()
        } else {
          let updated = this.messages.concat(this.messageBuffer.toArray())
          if (updated.length > 100) {
            updated.slice(updated.length - 100);
          }
          this.messages = updated
          
        }
        this.messageBuffer.clear()
      }
      
    }.bind(this),1000)

    window.setInterval(function () {
      if (this.socket !== null) {
        let list = this.$el.querySelector("div.el-table__body-wrapper.is-scrolling-none")
        if (list !== null) {
          list.scrollTop = list.scrollHeight
        }
      }
    }.bind(this), 200)
  }
}
</script>

<style>
#message-log td .cell {
  font-size: 10px;
  line-height: 12px;
  vertical-align: top;
  font-family: 'Roboto Mono', monospace;
}

#message-log td {
  padding: 0px 0px;
}

#message-log th {
  padding: 4px 0px;
}
</style>