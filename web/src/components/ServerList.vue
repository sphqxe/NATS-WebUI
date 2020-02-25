<template>
  <el-container style="padding: 24px;">
    <el-table :data="servers" style="width: 100%; border-bottom: none;" max-height="100%" fit="true" @row-click="selectServer">
      <!-- <el-table-column fixed prop="date" label="Date" width="150">
      </el-table-column> -->
      <el-table-column prop="name" label="Name" resizable>
      </el-table-column>
      <el-table-column prop="host" label="Hostname" resizable>
      </el-table-column>
      <el-table-column prop="port" label="Port" width="160">
      </el-table-column>
      <el-table-column prop="monitoring_port" label="Monitoring Port" width="160">
      </el-table-column>
      <el-table-column prop="varz.connections" label="Connections" width="120">
      </el-table-column>
      <el-table-column prop="varz.in_msgs" label="Messages In" width="120">
      </el-table-column>
      <el-table-column prop="varz.out_msgs" label="Messages Out" width="120">
      </el-table-column>
      <el-table-column label="Status" width="120">
        <template slot-scope="scope">
          <span :style="{color: servers[scope.$index].varz === null ? 'red' : 'green'}">⬤</span>
        </template>
      </el-table-column>
      <!-- <el-table-column label="Operations" width="120">
        <template slot-scope="scope">
          <el-button @click.native.prevent="deleteRow(scope.$index, tableData)" type="text" size="small">
            Remove
          </el-button>
        </template>
      </el-table-column> -->
    </el-table>
  </el-container>
</template>

<script>
import { mapState } from 'vuex'

export default {
  data: () => {
    return {
    }
  },
  computed: {
    ...mapState ({
      servers: s => s.app_state.servers
    })
  },
  methods: {
    selectServer(s) {
      this.$emit('selectServer', s.id)
    }
  }
}
</script>

<style>
.el-table tr {
  cursor: pointer !important;
}
</style>