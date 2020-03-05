import Vue from 'vue'
import App from './App.vue'
import Element from 'element-ui'
import 'element-ui/lib/theme-chalk/index.css'
import store from './store'
import axios from 'axios'
import VueAxios from 'vue-axios'
import { library } from '@fortawesome/fontawesome-svg-core'
import { faUserSecret, faServer, faDesktop, faLink, faUnlink, faCopyright } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import '../fonts/fonts.css'
import "prismjs";
import "prismjs/themes/prism.css";
import vueNumeralFilterInstaller from 'vue-numeral-filter'

library.add(faUserSecret)
library.add(faServer)
library.add(faDesktop)
library.add(faLink)
library.add(faUnlink)
library.add(faCopyright)

Vue.component('font-awesome-icon', FontAwesomeIcon)

Vue.use(Element)
Vue.use(VueAxios, axios)
Vue.use(vueNumeralFilterInstaller)
Vue.use(require('vue-moment'))
Vue.config.productionTip = false

new Vue({
  render: h => h(App),
  store,
  created() {
    store.dispatch('getAppState')
  }
}).$mount('#app')

