<script lang="ts">

    import TimepointEngineSelector from './TimepointEngineSelector.svelte'
    import SettingsPanel from '$lib/components/interfaces/settings/SettingsPanel.svelte'
    import * as Form from '$lib/components/ui/form'

    import type { Form as FormType } from '$lib/types/form'

    export let form: FormType
</script>

<div class="flex items-center pb-4 space-x-3">
    <TimepointEngineSelector {form} />
    <div class="grow" />
    <SettingsPanel {form} />
    <Form.Button>Process data</Form.Button>
</div>
