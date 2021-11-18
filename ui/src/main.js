import Vue from "vue";
import App from "./App.vue";
import "@/assets/style.css";

Vue.config.productionTip = false;

window.app = new Vue({
  render: (h) => h(App),
}).$mount("#app");

window.get_app = function() { return window.app.$children[0]; }

