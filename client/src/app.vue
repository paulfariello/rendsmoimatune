<template>
	<div class="top-bar">
		<div class="top-bar-title">
			<h1><a ui-sref="home">Rends-moi ma tune</a><small>beta</small></h1>
		</div>
	</div>
	<router-view></router-view>
</template>

<script>
export default {
	replace: false
}
</script>
