import { z } from 'zod'
import type { SuperForm } from 'sveltekit-superforms'
import type { PathObject } from 'path-browserify'

export const engineSchema = z.enum(['single', 'multi'])
export type engineType = z.infer<typeof engineSchema>

export const PathSchema: z.ZodType<PathObject> = z.any()
export const inputFileSchema = z.object({
    uuid: z.string(),
    path: PathSchema,
    engineType: engineSchema.nullable(),
    isProcessed: z.boolean(),
})
export type InputFile = z.infer<typeof inputFileSchema>

export const schema = z.object({
    engineType: engineSchema,
    shouldRemoveNACalculations: z.boolean(),
    toleranceMultiplier: z.number().min(0),
    inputFiles: z.array(inputFileSchema).min(0),
})

export type Form = SuperForm<z.infer<typeof schema>>