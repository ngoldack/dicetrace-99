import { expect, test } from "vitest";
import { Game, GameSchema } from "./game";

// Edit an assertion and save to see HMR in action

test("GameSchema::valid", () => {
  const game: Game = {
    bgg_id: 1,
    name: "foo",
  };

  expect(GameSchema.parse(game)).toStrictEqual(game);
});

test("GameSchema::invalid", () => {
  const game: any = {
    bgg_id: "test",
    name: undefined,
  };

  expect(GameSchema.safeParse(game).success).toBe(false);
});
