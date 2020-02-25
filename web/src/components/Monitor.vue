<template>
  <el-container style="display: flex; flex-direction: row;">
    <div id="subjects" style="flex: 1 1 60%; padding: 24px;">
      <h1>Subject Hierarchy</h1>
      <el-tree
        :data="data"
        show-checkbox
        node-key="id"
        :default-expanded-keys="[2, 3]"
        :default-checked-keys="[5]"
        :props="defaultProps">
      </el-tree>
      <!-- <h1>Publications</h1>
      <el-tree
        :data="data"
        show-checkbox
        node-key="id"
        :default-expanded-keys="[2, 3]"
        :default-checked-keys="[5]"
        :props="defaultProps">
      </el-tree> -->
    </div>
    <div id="details" style="flex: 1 1 40%; text-align: left;">
      <h1>{{server.name}}</h1>@{{ server.host }} <br>
      NATS Port: {{ server.port }} <br>
      Monitoring Port: {{ server.monitoring_port }} <br>
      <template v-if="server.varz!==null">
        Server Name: {{ server.varz.server_name }} <br>
        Server Id: {{ server.varz.server_id }} <br>
        Version: {{ server.varz.version }} <br>
        Protocol: {{ server.varz.proto }} <br>
        Go Version: {{ server.varz.go }} <br>
        Max Connections: {{ server.varz.max_connections }} <br>
        Ping Interval: {{ server.varz.ping_interval }} <br>
        Ping Max: {{ server.varz.ping_max }} <br>
        Max Control Line: {{ server.varz.max_control_line }} <br>
        Max Payload: {{ server.varz.max_payload }} <br>
        Max Pending: {{ server.varz.max_pending }} <br>
        TLS Timeout: {{ server.varz.tls_ttimeout }} <br>
        Write Deadline: {{ server.varz.write_deadline }} <br>
        Start Time: {{ server.varz.start }} <br>
        Up-time: {{ server.varz.uptime }} <br>
        Memory Available: {{ server.varz.mem }} <br>
        CPU Cores: {{ server.varz.core }} <br>
        Current Connections: {{ server.varz.connections }} <br>
        Total Connections: {{ server.varz.total_connections }} <br>
        Routes: {{ server.varz.routes }} <br>
        Remotes: {{ server.varz.remotes }} <br>
        Leaf Nodes: {{ server.varz.leafnodes }} <br>
        Messages In: {{ server.varz.in_msgs }} <br>
        Messages Out: {{ server.varz.out_msgs }} <br>
        Bytes In: {{ server.varz.in_bytes }} <br>
        Bytes Out: {{ server.varz.out_bytes }} <br>
        Slow Consumers: {{ server.varz.slow_consumers }} <br>
        Subscriptions: {{ server.varz.subscriptions }}
      </template>

    </div>
  </el-container>
</template>

<script>
import { mapState } from 'vuex'

export default {
  computed: {
    ...mapState({
      server: s => s.transient.serversMap[s.transient.selectedServer]
    })
  },
  data() {
    return {
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
      }
    };
  }
}
</script>

<style scoped>

</style>