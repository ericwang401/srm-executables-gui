<script lang="ts">
    import * as Sheet from '$lib/components/ui/sheet'
    import CogButton from '$lib/components/ui/CogButton.svelte'
    import { Button } from '$lib/components/ui/button'
    import { Icon, InformationCircle } from 'svelte-hero-icons'
    import { Input } from '$lib/components/ui/input'
    import { Label } from '$lib/components/ui/label'
    import * as Tooltip from '$lib/components/ui/tooltip'
    import * as Form from '$lib/components/ui/form'
    import { Switch } from '$lib/components/ui/switch'

    import type { Form as FormType } from '$lib/types/form'
    import SuperDebug from 'sveltekit-superforms'

    export let form: FormType

    const { form: formData } = form

</script>

<Sheet.Root>
    <Sheet.Trigger>
        <CogButton type="button" />
    </Sheet.Trigger>
    <Sheet.Content side="right" portal={false}>
        <Sheet.Header>
            <Sheet.Title>Advanced Settings</Sheet.Title>
            <Sheet.Description>
                For power users 🤠
            </Sheet.Description>
        </Sheet.Header>
        <div class="flex flex-col space-y-4 py-6">
            <div class="flex items-center space-x-2">
                <Switch id="should-remove-na-calculations" bind:checked={$formData.shouldRemoveNACalculations} />
                <Label for="should-remove-na-calculations">Avoid N/A Calculations</Label>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Icon src={InformationCircle} mini class='w-4 h-4' />
                    </Tooltip.Trigger>
                    <Tooltip.Content class='max-w-md'>
                        <p>Removes samples with no detected peaks from the input for a given peptide to avoid
                            "-nan(ind)"
                            calculations. This doesn't get rid of all "-nan(ind)" calculations but helps mitigate
                            them.</p>
                    </Tooltip.Content>
                </Tooltip.Root>
            </div>
            <div class="flex flex-col space-y-1.5">
                <Form.Field {form} name="toleranceMultiplier">
                    <Form.Control let:attrs>
                        <Form.Label class="flex items-center gap-2">
                            Tolerance Multiplier
                            <Tooltip.Root>
                                <Tooltip.Trigger>
                                    <Icon src={InformationCircle} mini class='w-4 h-4' />
                                </Tooltip.Trigger>
                                <Tooltip.Content class='max-w-md'>
                                    <p>SRM heuristically detects data pertaining to different charges for a given
                                        peptide. It
                                        uses
                                        standard deviation times this multipler to determine which mass-charge ratio
                                        don't
                                        belong in
                                        the
                                        same group. Adjust to what works, but 2.0 is recommended.</p>
                                </Tooltip.Content>
                            </Tooltip.Root>
                        </Form.Label>
                        <Input {...attrs} bind:value={$formData.toleranceMultiplier} />
                    </Form.Control>

                    <Form.FieldErrors />
                </Form.Field>
            </div>
        </div>
        <Sheet.Footer>
            <Sheet.Close asChild let:builder>
                <Button builders={[builder]} type="button">Save changes</Button>
            </Sheet.Close>
        </Sheet.Footer>
    </Sheet.Content>
</Sheet.Root>
