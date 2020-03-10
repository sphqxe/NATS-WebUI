<template>
  <el-container id="app" style="height: 100vh;">
    <el-container style="flex: 0 1 100%; overflow-y: hidden;">
      <el-aside style="display: flex; flex-direction: column; border-right: 1px solid rgb(230, 230, 230);">
        <div style="padding: 24px 0px; flex: 0 0 auto;">
          <img style="height: 48px;" src="large-logo.png">
          <div style=" width: max-content; height: 40px; line-height: 40px; font-size: small; margin-top: -24px; padding-left: 180px; font-family: 'Lexend Mega';">WEB-UI</div>
        </div>
        <el-menu class="el-menu-vertical-demo" style="text-align: left; flex: 0 1 100%; border-right: none; overflow-y: auto;" ref="menu" active-text-color="#303133">
          <el-menu-item index="'1'" ref="servers" @click="handleSelect(true, -1)" :class="{active: server===true && index === -1}">
            <template slot="title">
              <font-awesome-icon :icon="['fas', 'server']" style="margin: 0px 8px;" />
              <span>Servers</span>
            </template>
          </el-menu-item>
          <!-- <el-menu-item-group title="Group One"> -->
          <el-menu-item v-for="svr in servers" @click="handleSelect(true, svr.id)" :key="'server' + svr.id" :index="'1-' + svr.id" style="padding-left: 48px;"  :class="{active: server===true && index === svr.id}">
            <div style="display: flex; flex-direction: row; justify-content: space-between;">
              <span>{{ svr.name }}</span>
              <span :style="{color: svr.varz === null ? '#F56C6C' : '#67C23A'}">●</span>
            </div>
          </el-menu-item>
          <!-- </el-menu-item-group> -->
          <el-menu-item index="2" ref="clients" @click="handleSelect(false, -1)" :class="{active: server===false && index===-1}">
            <template slot="title">
              <font-awesome-icon :icon="['fas', 'desktop']" style="margin: 0px 8px;" />
              <span>Clients</span>
            </template>
          </el-menu-item>
          <!-- <el-menu-item-group title="Group One"> -->
          <el-menu-item v-for="client in clients" :index="'2-' + client.id" @click="handleSelect(false, client.id)" :key="'client' + client.id" style="padding-left: 48px;" :class="{active: server===false && index===client.id}">
            {{ client.name }}
          </el-menu-item>
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
import { mapState, mapMutations } from 'vuex'

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
      activeIndex: '1'
    }
  },
  methods: {
    ...mapMutations(['selectScreen']),
    handleSelectServer(i) {
      this.selectScreen({isServer: true, index: i})
    },
    handleSelect(server, i) {
      this.selectScreen({isServer: server, index: i})
    }
  },
  mounted() {
    this.$refs.menu.open('1')
  }, 
  computed: {
    ...mapState ({
      servers: s => s.app_state.servers,
      clients: s => s.app_state.clients,
      server: s => s.transient.server,
      index: s => s.transient.index,
      screen: s => [s.transient.server, s.transient.index]
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

.active {
  background: #ECF5FF;
}
</style>
