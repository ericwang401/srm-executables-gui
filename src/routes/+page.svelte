<script lang='ts'>
    import ActionBar from './ActionBar.svelte'

    const onDragEnter = (e:DragEvent) => {
        e.preventDefault()
        e.stopPropagation()
    }

    const onDragLeave = (e:DragEvent) => {
        e.preventDefault()
        e.stopPropagation()
    }

    const onDragOver = (e: DragEvent) => {
        e.preventDefault()
        e.stopPropagation()
        console.log('dragging over')
    }

    const onDrop = (e: DragEvent) => {
        e.preventDefault()
        e.stopPropagation()
        console.log('dropped')
        console.log({ e })
    }
</script>

<div
    class="fixed top-0 left-0 z-[9999999999] w-full h-full"
    on:dragenter={onDragEnter}
    on:dragleave={onDragLeave}
    on:dragover={onDragOver}
    on:drop={onDrop}
/>

<ActionBar />

<div class="p-12 w-full mt-4 border-2 border-dashed border-slate-300 h-40 rounded-md"></div>