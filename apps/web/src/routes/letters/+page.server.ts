import { getLetters } from "$lib/server/letter";

/** @type {import('./$types').PageLoad} */
export async function load({ request }) {
  const letters = await getLetters(request);
  return {
    letters,
  };
}
