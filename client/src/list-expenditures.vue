<template>
<div class="row">
	<div class="small-12 columns">
		<h3>Dépenses</h3>
		<expenditures :src="account.expenditures"></expenditures>
		<a class="button float-right fa fa-plus-circle" v-link="{ name: 'edit-expenditure', params: $route.params }">Nouvelle dépense</a>
	</div>
</div>
</template>

<script>
export default {
	props: {
		account: {
			type: Object,
			required: true
		}
	}
}
</script>
