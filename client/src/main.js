import 'src/css/main.scss'
import Vue from 'vue'
import VueRouter from 'vue-router'
import App from './app'
import Landing from './landing'
import Account from './account'

Vue.use(VueRouter)

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
