<template>
  <el-container style="display: flex; flex-direction: row;">
    <el-container style="flex: 1 0 50%;">
      <el-header style="border-bottom: 1px solid #e6e6e6; text-align: left; display: flex; flex-direction: column; padding: 8px; height: auto;">
        <div style="display: flex; flex-direction: row; margin-bottom: 8px;">
          <el-select v-model="value" placeholder="Choose a server" style="margin-right: 8px; flex: 1 1 100%;" @change="selectServer">
            <el-option
              v-for="server in servers"
              :key="server.id"
              :label="server.name"
              :value="server.id"
              :value-key="server.id">
            </el-option>
          </el-select>
          <el-button type="success" round plain>Connect</el-button>
          <el-button type="primary" round plain>Subscribe</el-button>
        </div>
        <el-checkbox-group v-model="msgTypes" style="flex: 1 1 auto;">
          <el-checkbox label="Info"></el-checkbox>
          <el-checkbox label="Ping"></el-checkbox>
          <el-checkbox label="Pong"></el-checkbox>
          <el-checkbox label="OK"></el-checkbox>
          <el-checkbox label="Err"></el-checkbox>
          <el-checkbox label="Pub"></el-checkbox>
          <el-checkbox label="Sub"></el-checkbox>
          <el-checkbox label="Unsub"></el-checkbox>
          <el-checkbox label="Connect"></el-checkbox>
          <el-checkbox label="Msg"></el-checkbox>
        </el-checkbox-group>
      </el-header>
      <el-main>
        main
      </el-main>
    </el-container>
    <el-aside style="border-left: 1px solid #e6e6e6; flex: 1 0 120px;">
      <h1>Subject Hierarchy</h1>
      <el-tree
        :data="data"
        show-checkbox
        node-key="id"
        :default-expanded-keys="[2, 3]"
        :default-checked-keys="[5]"
        :props="defaultProps">
      </el-tree>
    </el-aside>
  </el-container>
</template>

<script>
import { mapState } from 'vuex'

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
      }
    }
  },
  computed: {
    ...mapState({
      servers: s => s.app_state.servers
    })
  },
  methods: {
    selectServer(id) {
      this.value = id
    }
  }
}
</script>

<style scoped>

</style>