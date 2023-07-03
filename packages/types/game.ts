import { z } from "zod";

export let GameSchema = z.object({
  bgg_id: z.number(),
  name: z.string(),
});

export type Game = z.infer<typeof GameSchema>;
