import { writable } from 'svelte/store'
import type { EngineType } from '$lib/stores/settings'

export interface InputFile {
    path: string
    engineType: EngineType | null
    outputUuid: string | null
}

export const createInputFile = (path: string): InputFile => {
    return {
        path,
        engineType: null,
        outputUuid: null,
    }
}

export const inputFiles = writable<InputFile[]>([])
