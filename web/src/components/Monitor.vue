<template>
  <el-container style="padding: 0px;">
    <el-header style="padding-top: 24px; padding-left: 24px;">
      <el-breadcrumb separator-class="el-icon-arrow-right">
        <el-breadcrumb-item>NATS-WebUI</el-breadcrumb-item>
        <el-breadcrumb-item>Servers</el-breadcrumb-item>
        <el-breadcrumb-item>{{server.name}}</el-breadcrumb-item>
      </el-breadcrumb>
    </el-header>
    <el-main style="display: flex; flex-direction: row; padding: 0px;">
      <div id="details" style="flex: 1 1 100%; text-align: left; padding: 0px 24px;">
        <div style="display: flex; align-items: flex-end; margin-bottom: 12px;">
          <h1 style="font-size: 3em; margin: 0px;">
            {{server.name}}
          </h1>
          <span style="margin-bottom: 8px; margin-left: 8px;">
            @ {{ server.host }}{{ server.port === 4222 ? '' : (':' + server.port) }}
          </span>
        </div>
        
        <div v-if="server.varz!==null" style="display: flex; flex-direction: column;">
          <div class="metric-row">
            <div class="metric">
              <h1 class="metric-label">Messages In</h1>
              <span class="metric-value">{{ server.varz.in_msgs | numeral('0.[00]a') }}</span>
            </div>
            <div style="display: flex; flex-direction: column;">
              <h1 class="metric-label">Messages Out</h1>
              <span class="metric-value">{{ server.varz.out_msgs | numeral('0.00a') }}</span>
            </div>
            <div style="display: flex; flex-direction: column;">
              <h1 class="metric-label">Volume In</h1>
              <span class="metric-value">{{ server.varz.in_bytes | numeral('0.00b') }}</span>
            </div>
            <div style="display: flex; flex-direction: column;">
              <h1 class="metric-label">Volume Out</h1>
              <span class="metric-value">{{ server.varz.out_bytes | numeral('0.00b') }}</span>
            </div>
          </div>
          <div id="small-metrics" style="display: flex; flex-direction: row; flex-wrap: wrap;">
            <div class="small-metric">
              <span class="metric-label-small">
                Version
              </span>
              <span class="metric-value-small">
                {{ server.varz.version }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Protocol
              </span>
              <span class="metric-value-small">
                {{ server.varz.proto }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Go Version
              </span>
              <span class="metric-value-small">
                {{ server.varz.go }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Maximum Connections
              </span>
              <span class="metric-value-small">
                {{ server.varz.max_connections }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Ping Interval
              </span>
              <span class="metric-value-small">
                {{ server.varz.ping_interval }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Ping Max
              </span>
              <span class="metric-value-small">
                {{ server.varz.ping_max }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Max Control Line
              </span>
              <span class="metric-value-small">
                {{ server.varz.max_control_line }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Maximum Payload Size
              </span>
              <span class="metric-value-small">
                {{ server.varz.max_payload }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Maximum Pending
              </span>
              <span class="metric-value-small">
                {{ server.varz.max_pending }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                TLS Timeout
              </span>
              <span class="metric-value-small">
                {{ server.varz.tls_timeout }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Write Deadline
              </span>
              <span class="metric-value-small">
                {{ server.varz.write_deadline }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Start Time
              </span>
              <span class="metric-value-small">
                {{ server.varz.start }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Uptime
              </span>
              <span class="metric-value-small">
                {{ server.varz.uptime }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Memory Available
              </span>
              <span class="metric-value-small">
                {{ server.varz.mem }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                CPU Cores
              </span>
              <span class="metric-value-small">
                {{ server.varz.cores }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Current Connections
              </span>
              <span class="metric-value-small">
                {{ server.varz.connections }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Total Connections
              </span>
              <span class="metric-value-small">
                {{ server.varz.total_connections }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Routes
              </span>
              <span class="metric-value-small">
                {{ server.varz.routes }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Remotes
              </span>
              <span class="metric-value-small">
                {{ server.varz.remotes }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Leaf Nodes
              </span>
              <span class="metric-value-small">
                {{ server.varz.leafnodes }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Slow Consumers
              </span>
              <span class="metric-value-small">
                {{ server.varz.slow_consumers }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Subscriptions
              </span>
              <span class="metric-value-small">
                {{ server.varz.subscriptions }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Monitoring Port
              </span>
              <span class="metric-value-small">
                {{ server.monitoring_port }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Server Name
              </span>
              <span class="metric-value-small">
                {{ server.varz.server_name }}
              </span>
            </div>
            <div class="small-metric">
              <span class="metric-label-small">
                Server ID
              </span>
              <span class="metric-value-small">
                {{ server.varz.server_id}}
              </span>
            </div>
          </div>
        </div>
      </div>
      <div id="subjects" style="flex: 1 0 360px; padding: 24px; text-align: left;">
        <h1>Server Subject Hierarchy</h1>
        <el-button type="text" @click="editSubjectHierarchy">Edit</el-button>
        <el-tree :data="server.subjects" empty-text="No subjects configured for this server." default-expand-all
          :props="{label: 'subject_str', children: 'subjects', disabled: false, isLeaf: checkIsLeaf}">
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
    </el-main>
    <el-dialog title="Edit Subject Tree" :visible.sync="subjectHierarchyDialogVisible" width="30%" style="height: auto;" center>
      <prism-editor v-model="tabtree" language="md"></prism-editor>
      <span slot="footer" class="dialog-footer">
        <el-button @click="subjectHierarchyDialogVisible = false">Cancel</el-button>
        <el-button type="primary" @click="saveSubjectTree">Save</el-button>
      </span>
    </el-dialog>
  </el-container>
</template>

<script>
import { mapState, mapActions } from 'vuex'
import tabdown from 'tabdown-sacha'
import PrismEditor from 'vue-prism-editor'

export default {
  components: {
    PrismEditor,
  },
  computed: {
    ...mapState({
      server: s => s.transient.serversMap[s.transient.selectedServer]
    })
  },
  data() {
    return {
      tabtree: "",
      subjectHierarchyDialogVisible: false
    }
  },
  methods: {
    ...mapActions(['updateServer', 'getAppState']),
    editSubjectHierarchy() {
      this.tabtree = ""
      tabdown.traverse(this.subjectTreeToTabdownTreeRoot(this.server.subjects), function (node) {
        this.tabtree = this.tabtree.concat(this.tabtree.length === 0 ? '' : '\n','\t'.repeat(node.depth) + node.data)
      }.bind(this))
      // this.tabtree = tabdown.print(this.subjectTreeToTabdownTreeRoot(this.server.subjects))
      this.subjectHierarchyDialogVisible = true
    },
    async saveSubjectTree() {
      let server = JSON.parse(JSON.stringify(this.server))
      console.log(this.tabtree)
      server.subjects = this.tabdownTreeToSubjectTree(tabdown.parse(this.tabtree.replace(/ {2}/g, '\t').split('\n'), '\t'))
      await this.updateServer(server)
      await this.getAppState()
      this.subjectHierarchyDialogVisible = false
    },
    subjectTreeToTabdownTree(tree, parent, depth) {
      let node = {
        data: tree.subject_str,
        depth: depth,
        parent: parent,
        children: [],
        tabcount: 0
      }
      for (var i in tree.subjects) {
        let child_node = tree.subjects[i]
        node.children.push(this.subjectTreeToTabdownTree(child_node, node, depth+1))
      }
      return node
    },
    subjectTreeToTabdownTreeRoot(nodes) {
      let node = {
        data: null,
        depth: -1,
        parent: null,
        children:[]
      }
      for (var i in nodes) {
        let child_node = nodes[i]
        node.children.push(this.subjectTreeToTabdownTree(child_node, node, 0))
      }
      return node
    },
    tabdownTreeToSubjectTree(tree) {
      let subjectTreeNodes = []
      for (var i in tree.children) {
        let node = tree.children[i]
        let subjectTreeNode = {
          subject_str: node.data,
          subjects: this.tabdownTreeToSubjectTree(node)
        }
        subjectTreeNodes.push(subjectTreeNode)
      }
      return subjectTreeNodes
    },
    checkIsLeaf() {
      return false
    }
  }
}
</script>

<style scoped>
h1.metric-label {
  margin: 0px;
}

span.metric-value {
  font-size: 2.5em;
  font-weight: 100;
}

.metric-row {
  display: flex; flex-direction: row nowrap;
  justify-content: space-between;
  margin-bottom: 24px;
}

.metric {
  display: flex;
  flex-direction: column;
}

#small-metrics {
  border: solid 1px rgb(230, 230, 230)
}

.small-metric:nth-child(even) {
  background: rgb(246, 246, 246);
}

.small-metric:nth-child(odd) {
  background: rgb(255, 255, 255);
}

.small-metric {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  padding-right: 48px;
  padding-left: 8px;
  padding-top: 12px;
  padding-bottom: 0px;
  border: solid 1px rgb(230, 230, 230);
}

.metric-label-small {
  font-size: x-small;
  white-space: nowrap;
}

.metric-value-small {
  font-size: normal;
  white-space: nowrap;
}
</style>