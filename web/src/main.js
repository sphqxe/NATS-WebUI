import Vue from 'vue'
import App from './App.vue'
import Element from 'element-ui'
import 'element-ui/lib/theme-chalk/index.css'
import store from './store'
import axios from 'axios'
import VueAxios from 'vue-axios'
import { library } from '@fortawesome/fontawesome-svg-core'
import { faUserSecret, faServer, faDesktop } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

library.add(faUserSecret)
library.add(faServer)
library.add(faDesktop)

Vue.component('font-awesome-icon', FontAwesomeIcon)

Vue.use(Element)
Vue.use(VueAxios, axios)
Vue.config.productionTip = false

new Vue({
  render: h => h(App),
  store,
  created() {
    store.dispatch('getAppState')
  }
}).$mount('#app')
