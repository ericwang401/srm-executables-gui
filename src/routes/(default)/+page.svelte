<script lang='ts'>
    import InstallDepsCard from './InstallDepsCard.svelte'
    import InputDataCard from './InputDataCard.svelte'
    import ProcessDataButton from './ProcessDataButton.svelte'

</script>

<ProcessDataButton />
<div class="grid gap-4 grid-cols-1 md:grid-cols-4">
    <InstallDepsCard />
    <InputDataCard />
</div>
<ProcessDataButton />
