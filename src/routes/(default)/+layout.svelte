<script>
    import ContentContainer from '$lib/components/ContentContainer.svelte'
</script>

<div class="h-2 w-full bg-gradient-to-r from-indigo-600 to-fuchsia-400" />

<ContentContainer>
    <div class="space-y-1.5 pb-16 pt-8 lg:pt-12">
        <h1 class="font-heading text-2xl font-semibold md:text-5xl lg:text-7xl">SRM Executables</h1>
        <p class="max-w-2xl text-muted-foreground">
            I do not know what this does, but I am sure it is important. <a
                class="font-medium underline underline-offset-4"
                href="https://github.com/rgsadygov/SRM_executables"
                target="_blank"
            >
                SRM Executables binaries
            </a>
        </p>
    </div>

    <slot />
</ContentContainer>
