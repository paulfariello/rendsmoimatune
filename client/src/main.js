import 'src/css/main.scss'
import Vue from 'vue'
import VueRouter from 'vue-router'
import VueResource from 'vue-resource'
import App from './app'
import Landing from './landing'
import Account from './account'

Vue.use(VueRouter)
Vue.use(VueResource)

Vue.http.options.root = '/api'

const router = new VueRouter({
	history: false,
	saveScrollPosition: true
})

router.map({
	'/account': {
		component: Account
	},
	'/': {
		component: Landing
	}
})

router.start(App, 'body')
