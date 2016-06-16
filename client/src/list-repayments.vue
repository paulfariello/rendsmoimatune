<template>
<div class="row">
	<div class="small-12 columns">
		<h3>Remboursements</h3>
		<repayments :src="account.repayments"></repayments>
		<a class="button float-right fa fa-plus-circle" v-link="{ name: 'edit-repayment', params: $route.params }">Nouveau remboursement</a>
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
