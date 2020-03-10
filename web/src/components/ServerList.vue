<template>
  <el-container>
    <el-header style="padding: 12px; display: flex; flex-direction: row; justify-content: space-between; height: 60px;">
      <el-breadcrumb separator-class="el-icon-arrow-right" style="vertical-align: middle; padding: 12px;">
        <el-breadcrumb-item>NATS-WebUI</el-breadcrumb-item>
        <el-breadcrumb-item style="line-height: 16px; vertical-align: bottom;">Servers</el-breadcrumb-item>
      </el-breadcrumb>
      <el-button @click.native.stop="openCreateServerForm(null)" icon="el-icon-plus" size="small">
        Add Server
      </el-button>
    </el-header>
    <el-main style="padding: 0px 20px; overflow-y: auto;">
      <el-table :data="servers" style="width: 100%; border-bottom: none;" :fit="true" @row-click="selectServer" height="100%">
        <!-- <el-table-column fixed prop="date" label="Date" width="150">
        </el-table-column> -->
        <div slot="empty">
          No servers configured. <br>
          <span @click="openCreateServerForm(null)">Create</span> a new one.
        </div>
        <el-table-column prop="name" label="Name" width="150">
        </el-table-column>
        <el-table-column prop="host" label="Hostname" resizable>
        </el-table-column>
        <el-table-column prop="port" label="Port" width="90">
        </el-table-column>
        <el-table-column prop="monitoring_port" label="Monitoring Port" width="160">
        </el-table-column>
        <el-table-column prop="varz.connections" label="Connections" width="120">
        </el-table-column>
        <el-table-column prop="varz.in_msgs" :formatter="numericFormatter" label="Messages In" width="120">
        </el-table-column>
        <el-table-column prop="varz.out_msgs" :formatter="numericFormatter" label="Messages Out" width="120">
        </el-table-column>
        <el-table-column prop="varz.in_bytes" :formatter="bytesFormatter" label="Bytes In" width="90">
        </el-table-column>
        <el-table-column prop="varz.out_bytes" :formatter="bytesFormatter" label="Bytes Out" width="90">
        </el-table-column>
        <el-table-column label="Status" width="120">
          <template slot-scope="scope">
            <span style="padding: 4px 8px 2px 8px; border-radius: 4px; vertical-align: middle; color: #f6f6f6;" :style="{background: servers[scope.$index].varz === null ? '#F56C6C' : '#67C23A'}">
              {{servers[scope.$index].varz === null ? 'Unreachable' : 'Connected' }}
            </span>
          </template>
        </el-table-column>
        <el-table-column label="Operations" width="120">
          <template slot-scope="scope">
            <el-button @click.native.stop="editServer(scope.$index)" icon="el-icon-edit" size="small">
            </el-button>
            <el-button @click.native.stop="removeServer(scope.$index)" icon="el-icon-delete" size="small">
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-main>
    
    <el-dialog title="Configure New NATS Server" :visible.sync="createServerDialogVisible" width="30%" center>
      <el-form ref="form" :model="createServerForm" label-width="120px">
        <el-form-item label="Server Name">
          <el-input v-model="createServerForm.name"></el-input>
        </el-form-item>
        <el-form-item label="Hostname">
          <el-input v-model="createServerForm.host"></el-input>
        </el-form-item>
        <el-form-item label="Message Port">
          <el-input-number :controls="false" v-model="createServerForm.port"></el-input-number>
        </el-form-item>
        <el-form-item label="Monitoring Port">
          <el-input-number :controls="false" v-model="createServerForm.monitoring_port"></el-input-number>
        </el-form-item>
      </el-form>
      <span slot="footer" class="dialog-footer">
        <el-button @click="createServerDialogVisible = false">Cancel</el-button>
        <el-button type="primary" @click="serverFormSave">Save</el-button>
      </span>
    </el-dialog>
  </el-container>
</template>

<script>
import { mapState, mapActions } from 'vuex'
import ReconnectingWebSocket from 'reconnecting-websocket'

var numeral = require('numeral')

export default {
  data: () => {
    return {
      createServerDialogVisible: false,
      createServerForm: {
        id: null,
        name: '',
        host: '',
        port: 0,
        monitoring_port: 0,
        subjects: [],
        publications: []
      },
      socket: null
    }
  },
  computed: {
    ...mapState ({
      servers: s => s.app_state.servers,
      serversMap: s => s.transient.serversMap
    })
  },
  methods: {
    ...mapActions(['createServer', 'updateServer', 'deleteServer']),
    selectServer(s) {
      this.$emit('selectServer', s.id)
    },
    openCreateServerForm(id) {
      this.createServerForm = {
        id: id,
        name: '',
        host: '',
        port: 4222,
        monitoring_port: 8222,
        subjects: [],
        publications: []
      }
      this.createServerDialogVisible = true
    },
    async serverFormSave() {
      if (this.createServerForm.id === null) {
        await this.createServer(JSON.parse(JSON.stringify(this.createServerForm)))
      } else {
        await this.updateServer(JSON.parse(JSON.stringify(this.createServerForm)))
      }
      this.createServerDialogVisible = false
    },
    editServer(idx) {
      this.createServerForm = JSON.parse(JSON.stringify(this.servers[idx]))
      this.createServerDialogVisible = true
    },
    async removeServer(idx) {
      await this.deleteServer(this.servers[idx].id)
    },
    numericFormatter(row, col, cellValue) {
      return numeral(cellValue).format('0.[00]a')
    },
    bytesFormatter(row, col, cellValue) {
      return numeral(cellValue).format('0.00b')
    }
  },
  mounted () {
    this.socket = new ReconnectingWebSocket('ws://' + window.location.host + '/api/state/ws')
    this.socket.addEventListener('message', function (ev) {
      let msg = JSON.parse(ev.data)
      this.serversMap[msg.server_id].varz = msg.varz
      if (this.serversMap[msg.server_id].timeoutId !== undefined) {
        window.clearTimeout(this.serversMap[msg.server_id].timeoutId)
      }
      this.serversMap[msg.server_id].timeoutId = window.setTimeout(function() {
        this.serversMap[msg.server_id].varz = null
      }, 5000)
    }.bind(this))
  }
}
</script>

<style>
.el-table tr {
  cursor: pointer !important;
}
</style>