<template>
<div class="cover">
	<div class="row">
		<div class="small-12 columns">
			<h2>Créer un nouveau compte</h2>
			<form v-on:submit="createAccount">
				<div class="input-group">
					<input type="text" class="input-group-field" v-model="account_name" placeholder="Nom" required />
					<div class="input-group-button">
						<button type="submit" class="button">Créer</button>
					</div>
				</div>
			</form>
		</div>
	</div>
</div>
</template>

<script>
export default {
	data () {
		return {
			'account_name': ''
		}
	},
	methods: {
		createAccount () {
			var resource = this.$resource('account/{id}')

			resource.save({name: this.account_name}).then(function (response) {
				console.log(response)
				this.$router.go({name: 'account', params: { accountId: response.data.uid }})
			}, function (response) {
				// TODO error handling
			})
		}
	}
}
</script>

<style>
</style>
