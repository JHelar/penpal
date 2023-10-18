import { VITE_SERVER_API_URL } from "$env/static/private";
import { array, coerce, date, object, parse, string, uuid } from "valibot";

const DateSchema = coerce(date(), (input) => new Date(input as string));

const LetterSchema = object({
  id: string([uuid()]),
  message: string(),
  to_user_id: string([uuid()]),
  by_user_id: string([uuid()]),
  created_at: DateSchema,
  updated_at: DateSchema,
});

const LettersSchema = array(LetterSchema);

/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
  const lettersResponse = await fetch(
    `${VITE_SERVER_API_URL}/me/letter`,
    {
      headers: {
        Authorization: `Bearer 66e3d037-d0c3-48af-b11c-b046b5c2ab63`,
      },
    },
  );

  if (!lettersResponse.ok) {
    return {
      letters: [],
    };
  }

  const letters = parse(LettersSchema, await lettersResponse.json());
  return {
    letters,
  };
}
