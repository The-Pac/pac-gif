import {createApp} from "vue";
import "./app/bootstrap.scss";
import App from "./app/App.vue";
import router from "./app/Router";
import {addIcons, OhVueIcon} from "oh-vue-icons";

const app = createApp(App)

//ICONS
addIcons()
app.component("v-icon", OhVueIcon);

//ROUTES
app.use(router)

//MOUNT
app.mount('#app');