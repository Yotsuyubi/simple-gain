import Vue from 'vue'
import App from './App'
import Element from 'element-ui'
// import 'element-theme-chalk';

Vue.use(Element)

new Vue({
    el: '#app',
    components: { App },
    template: '<app/>',
})