<template>
  <el-container id="app">
    <el-container>
      <el-aside>
        <el-menu class="el-menu-vertical-demo" style="text-align: left; height: 100%;" ref="menu">
          <el-menu-item index="1" ref="servers" @click="handleSelect(true, -1)">
            <template slot="title">
              <i class="el-icon-location"></i>
              <span>Servers</span>
            </template>
          </el-menu-item>
          <!-- <el-menu-item-group title="Group One"> -->
          <el-menu-item v-for="server in servers" @click="handleSelect(true, server.id)" :key="server.id" :index="'1-' + server.id">
            {{ server.name }}
          </el-menu-item>
          <!-- </el-menu-item-group> -->
          <el-menu-item index="2" ref="clients" @click="handleSelect(true, -1)">
            <template slot="title">
              <i class="el-icon-location"></i>
              <span>Clients</span>
            </template>
          </el-menu-item>
          <!-- <el-menu-item-group title="Group One"> -->
          <el-menu-item index="2-1" @click="handleSelect(false, 0)">Client 1</el-menu-item>
          <el-menu-item index="2-2">Client 2</el-menu-item>
          <!-- </el-menu-item-group> -->
          <!-- <el-menu-item-group title="Group Two"> -->
          <el-menu-item index="2-3">Client 3</el-menu-item>
          <!-- </el-menu-item-group> -->
        </el-menu>
      </el-aside>
      <ServerList v-show="server===true && index === -1" @selectServer="handleSelectServer"/>
      <ClientList v-show="server===false && index === -1"/>
      <Client v-show="server===false && index >= 0"/>
      <Monitor v-show="server===true && index >= 0"/>
    </el-container>
    
    <el-footer style="border-top: solid 1px #e6e6e6; height: 24px; text-align: right;">
      Copyright Theodore Lee
    </el-footer>
  </el-container>
</template>

<script>
import ServerList from './components/ServerList.vue'
import ClientList from './components/ClientList.vue'
import Client from './components/Client.vue'
import Monitor from './components/Monitor.vue'
import { mapState } from 'vuex'

export default {
  name: 'App',
  components: {
    Client, 
    Monitor,
    ServerList,
    ClientList
  },
  data: () => {
    return {
      activeIndex: '1',
      server: true,
      index: -1
    }
  },
  methods: {
    handleSelectServer(i) {
      this.handleSelect(true, i)
    },
    handleSelect(server, i) {
      this.server = server;
      this.index = i;
    },
    selectMainPage() {

    }
  },
  mounted() {
    this.$refs.menu.open(1)
    this.$refs.menu.open(2)
  }, 
  computed: {
    ...mapState ({
      servers: s => s.app_state.servers,
      clients: s => s.app_state.clients
    })
  }
}
</script>

<style>
body {
  margin: 0px !important;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  height: 100vh;
  width: 100vw;
  overflow-x: auto;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

#main-space {
  display: flex;
  flex-direction: row;
  flex: 1 1 100%;
}
</style>
