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
				<button aria-label="Supprimer" class="button alert"><i class="fa fa-trash-o fa-lg"></i></button>
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
	}
}
</script>
