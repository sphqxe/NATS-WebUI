<template>
  <el-container style="display: flex; flex-direction: row;">
    <el-container style="flex: 1 0 50%;">
      <el-header style="border-bottom: 1px solid #e6e6e6; text-align: left; display: flex; flex-direction: column; padding: 0px; height: auto;">
        <el-breadcrumb separator-class="el-icon-arrow-right" style="vertical-align: middle; padding: 24px 24px 12px 24px; flex: 0 0 auto;" >
          <el-breadcrumb-item @click.native="selectScreen({isServer: true, index: -1})" style="cursor: pointer">NATS-WebUI</el-breadcrumb-item>
          <el-breadcrumb-item @click.native="selectScreen({isServer: false, index: -1})" style="cursor: pointer">Clients</el-breadcrumb-item>
          <el-breadcrumb-item>{{ client.name }}</el-breadcrumb-item>
        </el-breadcrumb>
        <div style="display: flex; flex-direction: row; margin-bottom: 8px; padding: 0px 8px;">
          <el-input placeholder="Client Name" v-model="client.name" style="margin-right: 8px; flex: 1 1 30%;" @change="handleNameChange"></el-input>
          <el-select v-model="client.server_id" placeholder="Choose a server" style="margin-right: 8px; flex: 1 1 100%;" @change="handleSelectServer">
            <el-option v-for="server in servers" :key="server.id" :label="server.name" :value="server.id" :value-key="server.id">
            </el-option>
          </el-select>
          <el-tooltip class="item" effect="dark" content="Subscribe" placement="bottom" v-if="client.socket === null">
            <el-button round plain @click="connect">
              <font-awesome-icon :icon="['fas', 'link']"/>
            </el-button>
          </el-tooltip>
          <el-tooltip class="item" effect="dark" content="Unsubscribe" placement="bottom" v-if="client.socket !== null">
            <el-button type="success" round @click="disconnect">
              <font-awesome-icon :icon="['fas', 'unlink']"/>
            </el-button>
          </el-tooltip>
        </div>
        <div style="flex: 1 1 auto; display: flex; flex-direction: row; margin-left: 8px; padding: 0px 8px 8px 8px;">
          <el-checkbox label="Info" v-model="client.info" @change="handleFilterCheckChange"></el-checkbox>
          <el-checkbox label="Ping" v-model="client.ping" @change="handleFilterCheckChange"></el-checkbox>
          <el-checkbox label="Pong" v-model="client.pong" @change="handleFilterCheckChange"></el-checkbox>
          <el-checkbox label="OK" v-model="client.ok" @change="handleFilterCheckChange"></el-checkbox>
          <el-checkbox label="Err" v-model="client.err" @change="handleFilterCheckChange"></el-checkbox>
          <el-checkbox label="Msg" v-model="client.msg" @change="handleFilterCheckChange"></el-checkbox>
        </div>
      </el-header>
      <el-main style="padding: 0px;">
        <el-table id="message-log" :data="client.messages" height="100%" style="width: 100%" fit>
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
    <el-aside style="border-left: 1px solid #e6e6e6; flex: 1 0 120px; padding: 12px;">
      <h1 style="text-align: left; font-size: 1em; margin: 8px 0px;">Server Subject Hierarchy</h1>
      <el-tree ref="tree" :data="server.subjects" empty-text="No subjects configured for this server." default-expand-all node-key="id"
        :props="{label: 'subject_str', children: 'subjects', disabled: false, isLeaf: checkIsLeaf}" 
        show-checkbox @check="handleCheckChange" :default-checked-keys="checkedKeys" check-strictly>
      </el-tree>
    </el-aside>
  </el-container>
</template>

<script>
import { mapState, mapActions, mapMutations } from 'vuex'
import ReconnectingWebSocket from 'reconnecting-websocket'
import moment from 'moment'

export default {
  data: () => {
    return {
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
          if (node.selected) {
            res.push(node.id)
          }
          for (var i in node.subjects) {
            let child = node.subjects[i]
            getCheckedKeys(child)
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
    ...mapMutations(['selectScreen']),
    connect() {
      this.client.socket = new ReconnectingWebSocket("ws://" + window.location.host + "/client/1")
      this.client.socket.addEventListener('message', this.handleSocketEvent(this.client))
    },
    disconnect() {
      this.client.socket.close()
      this.client.socket = null
    },
    handleSocketEvent(cl) {
      return function (ev) {
        let data = JSON.parse(ev.data)
        data.timestamp = moment(data.timestamp).format("YYYY-MM-DD HH:mm:ss.SSS")
        if (data.typ === 'Msg' && cl.msg
          || data.typ === 'Info' && cl.info
          || data.typ === 'Ping' && cl.ping
          || data.typ === 'Pong' && cl.pong
          || data.typ === 'Ok' && cl.ok
          || data.typ === 'Err' && cl.err) {
            if (cl.messageBuffer.length == 100) {
              cl.messageBuffer.shift()
            }
            cl.messageBuffer.push(data)
        }
      }
    },
    async handleSelectServer(serverId) {
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
      if (this.client.socket !== null) {
        this.disconnect()
      }

      let checked = new Set()
      this.$refs.tree.getCheckedNodes().forEach(function (e) {
        checked.add(e.id)
      })

      let roots = JSON.parse(JSON.stringify(this.server.subjects))
      function setSelected(node) {
        node.selected = checked.has(node.id)
        for (var i in node.subjects) {
          let child = node.subjects[i]
          setSelected(child)
        }
      }

      roots.forEach(node => setSelected(node))
      let client = JSON.parse(JSON.stringify(this.client))
      client.subjects = roots
      await this.updateClient(client)
      await this.getAppState()
    },
    async handleNameChange(newName) {
      if (this.client.socket !== null) {
        this.disconnect()
      }

      let client = JSON.parse(JSON.stringify(this.client))
      client.name = newName
      await this.updateClient(client)
    },
    async handleFilterCheckChange() {
      let client = JSON.parse(JSON.stringify(this.client))
      await this.updateClient(client)
    }
  },
  mounted () {
    window.setInterval(function () {
      if (this.client !== undefined) {
        if (this.client.messageBuffer.length > 0) {
          if (this.client.messageBuffer.length === 100) {
            this.client.messages = this.client.messageBuffer.toArray()
          } else {
            let updated = this.client.messages.concat(this.client.messageBuffer.toArray())
            if (updated.length > 100) {
              updated.slice(updated.length - 100);
            }
            this.client.messages = updated
            
          }
          this.client.messageBuffer.clear()
        }
      }
    }.bind(this),1000)

    window.setInterval(function () {
      if (this.client !== undefined) {
        if (this.client.socket !== null) {
          let list = this.$el.querySelector("div.el-table__body-wrapper.is-scrolling-none")
          if (list !== null) {
            list.scrollTop = list.scrollHeight
          }
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