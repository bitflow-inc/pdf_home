// Require Froala Editor js file.
require('froala-editor/js/froala_editor.pkgd.min.js')
// Require Froala Editor css files.
require('froala-editor/css/froala_editor.pkgd.min.css')
require('froala-editor/css/froala_style.min.css')

import Vue from 'vue'
import App from './App.vue'
import VueRouter from 'vue-router'
import routes from './routes';
import VueFroala from 'vue-froala-wysiwyg'

Vue.config.productionTip = false
Vue.use(VueRouter);
Vue.use(VueFroala)

const router = new VueRouter({routes});

new Vue({
  router,
  render: h => h(App)
}).$mount('#app')
