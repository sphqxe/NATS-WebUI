<template>
  <el-container style="padding: 0px;">
    <el-header style="padding: 24px;">
      <el-breadcrumb separator-class="el-icon-arrow-right" >
        <el-breadcrumb-item>NATS-WebUI</el-breadcrumb-item>
        <el-breadcrumb-item>Clients</el-breadcrumb-item>
      </el-breadcrumb>
    </el-header>
    <el-main style="padding: 0px;">
      <el-table :data="clients" style="width: 100%; border-bottom: none;" max-height="100%" :fit="true">
        <el-table-column fixed prop="name" label="Name" width="150">
        </el-table-column>
        <el-table-column prop="server.hostname" label="Server Address" width="120">
        </el-table-column>
        <el-table-column prop="" label="Status" width="120">
        </el-table-column>
        <el-table-column label="Operations" width="120">
          <template slot-scope="scope">
            <el-button @click.native.prevent="deleteRow(scope.$index, tableData)" type="text" size="small">
              Remove
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-main>
  </el-container>
</template>

<script>
import { mapState } from 'vuex'

export default {
  computed: {
    ...mapState({
      clients: function(s) {
        let cl = []
        for (var i in s.app_state.clients) {
          let c = JSON.parse(JSON.stringify(s.app_state.clients[i]))
          c.server = s.transient.serversMap[c.server_id]
          cl.push(c)
        }
        return cl
      }
    })
  },
  data: () => {
    return {
      tableData: [
        {
          date: '2016-05-03',
          name: 'Tom',
          state: 'California',
          city: 'Los Angeles',
          address: 'No. 189, Grove St, Los Angeles',
          zip: 'CA 90036'
        },
        {
          date: '2016-05-02',
          name: 'Tom',
          state: 'California',
          city: 'Los Angeles',
          address: 'No. 189, Grove St, Los Angeles',
          zip: 'CA 90036'
        },
        {
          date: '2016-05-04',
          name: 'Tom',
          state: 'California',
          city: 'Los Angeles',
          address: 'No. 189, Grove St, Los Angeles',
          zip: 'CA 90036'
        },
        {
          date: '2016-05-01',
          name: 'Tom',
          state: 'California',
          city: 'Los Angeles',
          address: 'No. 189, Grove St, Los Angeles',
          zip: 'CA 90036'
        },
        {
          date: '2016-05-08',
          name: 'Tom',
          state: 'California',
          city: 'Los Angeles',
          address: 'No. 189, Grove St, Los Angeles',
          zip: 'CA 90036'
        },
        {
          date: '2016-05-06',
          name: 'Tom',
          state: 'California',
          city: 'Los Angeles',
          address: 'No. 189, Grove St, Los Angeles',
          zip: 'CA 90036'
        },
        {
          date: '2016-05-07',
          name: 'Tom',
          state: 'California',
          city: 'Los Angeles',
          address: 'No. 189, Grove St, Los Angeles',
          zip: 'CA 90036'
        }
      ]
    }
  }
}
</script>

<style scoped>

</style>