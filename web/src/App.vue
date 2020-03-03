<template>
  <el-container id="app" style="height: 100vh;">
    <el-container style="flex: 0 1 100%; overflow-y: hidden;">
      <el-aside style="display: flex; flex-direction: column; border-right: 1px solid rgb(230, 230, 230);">
        <div style="padding: 24px 0px; flex: 0 0 auto;">
          <img style="height: 48px;" src="large-logo.png">
          <div style=" width: fit-content; height: 40px; line-height: 40px; font-size: small; margin-top: -24px; padding-left: 180px; font-family: 'Lexend Mega';">WEB-UI</div>
        </div>
        <el-menu class="el-menu-vertical-demo" style="text-align: left; flex: 1 1 100%; border-right: none;" ref="menu">
          <el-menu-item index="1" ref="servers" @click="handleSelect(true, -1)">
            <template slot="title">
              <font-awesome-icon :icon="['fas', 'server']" style="margin: 0px 8px;" />
              <span>Servers</span>
            </template>
          </el-menu-item>
          <!-- <el-menu-item-group title="Group One"> -->
          <el-menu-item v-for="server in servers" @click="handleSelect(true, server.id)" :key="server.id" :index="'1-' + server.id" style="padding-left: 48px;">
            {{ server.name }}
          </el-menu-item>
          <!-- </el-menu-item-group> -->
          <el-menu-item index="2" ref="clients" @click="handleSelect(false, -1)">
            <template slot="title">
              <font-awesome-icon :icon="['fas', 'desktop']" style="margin: 0px 8px;" />
              <span>Clients</span>
            </template>
          </el-menu-item>
          <!-- <el-menu-item-group title="Group One"> -->
          <el-menu-item index="2-1" @click="handleSelect(false, 0)" style="padding-left: 48px;">Client 1</el-menu-item>
          <!-- </el-menu-item-group> -->
        </el-menu>
      </el-aside>
      <ServerList v-show="server===true && index === -1" @selectServer="handleSelectServer"/>
      <ClientList v-show="server===false && index === -1"/>
      <Client v-show="server===false && index >= 0"/>
      <Monitor v-show="server===true && index >= 0"/>
    </el-container>
    
    <el-footer style="border-top: solid 1px #e6e6e6; height: 24px; text-align: right; vertical-align: middle; font-size: x-small; line-height: 24px;">
      <font-awesome-icon :icon="['fas', 'copyright']"/> sphqxe <a href="https://github.com/sphqxe">github.com/sphqxe</a>
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
      if (server) {
        this.$store.state.transient.selectedServer = i
      }
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
  font-family: 'Lato', sans-serif;
}

h1, h2, h3, h4, h5, h6 {
  font-family: 'Raleway';
}

#app {
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

.el-select-dropdown__item {
  font-family: Avenir, Helvetica, Arial, sans-serif;
}
</style>
