<template>
<table v-if="src.length > 0">
	<thead>
		<tr>
			<th>Date</th>
			<th>De</th>
			<th></th>
			<th>Montant</th>
			<th></th>
			<th>À</th>
			<th>Actions</th>
		</tr>
	</thead>
	<tbody>
		<tr v-for="repayment in src | orderBy 'date' | limitBy limit">
			<td>{{ repayment.date }}</td>
			<td>{{ repayment.payer }}</td>
			<td>a remboursé</td>
			<td>{{ repayment.amount | currency }}</td>
			<td>à</td>
			<td>{{ repayment.beneficiary }}</td>
			<td>
				<a v-link="{ name: 'edit-repayment', params: { repaymentId: repayment.id } }" aria-label="Éditer" class="button"><i class="fa fa-pencil fa-lg"></i></a>
				<button aria-label="Supprimer" v-reveal-open="'delete-repayment-modal-' + repayment.id" class="button alert"><i class="fa fa-trash-o fa-lg"></i></button>
				<div class="reveal" id="delete-repayment-modal-{{ repayment.id }}" v-reveal-data>
					<h1>Supprimer le remboursement de {{ repayment.amount | currency }} de {{ repayment.payer }} à {{ repayment.beneficiary }} ?</h1>
					<p class="lead">Êtes-vous sûr de vouloir supprimer définitivement le remboursement de {{ repayment.amount | currency }} de {{ repayment.payer }} à {{ repayment.beneficiary }} ?</p>
					<button aria-label="Confirmer la suppression" v-reveal-close v-on:click="deleteRepayment(repayment)" class="button alert">Oui, je confirme</button>
					<button aria-label="Confirmer la suppression" v-reveal-close class="button">Non</button>
				</div>
			</td>
		</tr>
	</tbody>
</table>
</template>

<script>
export default {
	props: {
		limit: {
			type: Number,
			default: Infinity
		},
		src: {
			type: Array,
			required: true
		}
	},
	methods: {
		deleteRepayment (repayment) {
			var self = this
			var resource = this.$resource('account/' + this.$route.params.accountId + '/repayments/' + repayment.id)
			resource.delete(this.repayment).then(function (response) {
				self.src.$remove(repayment)
			}, function (response) {
				// TODO error handling
			})
		}
	}
}
</script>
