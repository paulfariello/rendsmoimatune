var router = new VueRouter();
router.map({
	'/': {
		component: home
	},
});

router.start(Home, '#app')
