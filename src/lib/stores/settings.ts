import { writable } from 'svelte/store'

export const shouldRemoveNACalculations = writable<boolean>(false)
export const toleranceMultiplier = writable<number>(2.0)

export type EngineType = 'single' | 'multi'
export const engineType = writable<EngineType>('single')
