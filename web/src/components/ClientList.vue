<template>
  <el-container>
    <el-header style="padding: 12px; display: flex; flex-direction: row; justify-content: space-between; height: 60px;">
      <el-breadcrumb separator-class="el-icon-arrow-right" style="vertical-align: middle; padding: 12px;">
        <el-breadcrumb-item>NATS-WebUI</el-breadcrumb-item>
        <el-breadcrumb-item>Clients</el-breadcrumb-item>
      </el-breadcrumb>
      <el-tooltip class="item" effect="dark" content="You must have configure at least 1 server before adding a client." placement="bottom-end" :disabled="servers.length!==0">
        <el-button @click.native.stop="handleCreateNewClient()" icon="el-icon-plus" size="small" :disabled="servers.length === 0">
          Add Client
        </el-button>
      </el-tooltip>
    </el-header>
    <el-main style="padding: 0px 20px;">
      <el-table :data="clients" style="width: 100%; border-bottom: none;" max-height="100%" :fit="true">
        <el-table-column fixed prop="name" label="Name" width="150">
        </el-table-column>
        <el-table-column prop="server.hostname" label="Server Address" resizable>
        </el-table-column>
        <el-table-column prop="" label="Status" width="120">
        </el-table-column>
        <el-table-column label="Operations" width="120">
          <template slot-scope="scope">
            <!-- <el-popconfirm confirmButtonText='OK' cancelButtonText='Cancel' title="Confirm delete?"> -->
              <el-button @click.native.stop="removeClient(scope.$index)" icon="el-icon-delete" size="small">
              </el-button>
            <!-- </el-popconfirm> -->
          </template>
        </el-table-column>
      </el-table>
    </el-main>
  </el-container>
</template>

<script>
import { mapState, mapActions } from 'vuex'

export default {
  computed: {
    ...mapState({
      clients: function(s) {
        let cl = []
        for (var i in s.app_state.clients) {
          let c = JSON.parse(JSON.stringify(s.app_state.clients[i]))
          c.server = JSON.parse(JSON.stringify(s.transient.serversMap[c.server_id]))
          cl.push(c)
        }
        return cl
      },
      servers: s => s.app_state.servers
    })
  },
  data: () => {
    return {
    }
  },
  methods: {
    ...mapActions(['createClient', 'getAppState', 'deleteClient']),
    async handleCreateNewClient() {
      let cl = {
        id: null,
        name: "New Client",
        server_id: this.servers[0].id,
        subjects: [],
        info: true,
        ping: true,
        pong: true,
        ok: true,
        err: true,
        publ: true,
        sub: true,
        unsub: true,
        connect: true,
        msg: true
      }
      console.log(JSON.stringify(cl))
      await this.createClient(cl)
      await this.getAppState()
    },
    async removeClient(idx) {
      await this.deleteClient(this.clients[idx].id)
    }
  }
}
</script>

<style scoped>

</style>