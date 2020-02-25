import Vue from 'vue'
import Vuex from 'vuex'
import axios from 'axios'
import VueAxios from 'vue-axios'

Vue.use(Vuex)
Vue.use(VueAxios, axios)

export default new Vuex.Store({
  state: {
    app_state: {
      servers: [],
      clients: []
    },
    transient: {
      serversMap: {},
      clientsMap: {},
      selectedServer: 1
    }
  },
  getters: {},
  mutations: {
    updateAppState(state, appState) {
      state.app_state = appState
      state.transient.serversMap = state.app_state.servers.reduce((o, e) => { o[e.id] = e; return o }, {})
      state.transient.clientsMap = state.app_state.clients.reduce((o, e) => { o[e.id] = e; return o }, {})
    }
  },
  actions: {
    async getAppState (ctx) {
      var resp = await axios.get('/api/state')
      ctx.commit('updateAppState', resp.data)
    }
  }
})