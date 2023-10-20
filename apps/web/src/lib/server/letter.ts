import { VITE_SERVER_API_URL } from "$env/static/private";
import { array, coerce, date, object, parse, string, uuid } from "valibot";
import { fetch } from "undici";

const DateSchema = coerce(date(), (input) => new Date(input as string));

export const LetterSchema = object({
  id: string([uuid()]),
  message: string(),
  to_user_id: string([uuid()]),
  by_user_id: string([uuid()]),
  created_at: DateSchema,
  updated_at: DateSchema,
});

export const LettersSchema = array(LetterSchema);

export async function getLetters(request: Request) {
  const lettersResponse = await fetch(`${VITE_SERVER_API_URL}/me/letter`, {
    headers: request.headers,
  });
  if (!lettersResponse.ok) {
    return [];
  }

  try {
    const letters = parse(LettersSchema, await lettersResponse.json());
    return letters;
  } catch (error) {
    console.error(error);
    return [];
  }
}

type CreateLetterArgs = {
  message: string;
  request: Request;
};
export async function createLetter({ message, request }: CreateLetterArgs) {
  const letterResponse = await fetch(`${VITE_SERVER_API_URL}/me/letter`, {
    headers: {
      cookie: request.headers.get("cookie") ?? "",
      "content-type": "application/json",
    },
    method: "post",
    body: JSON.stringify({
      message,
      to_user_id: "9d8a6fee-a19a-4372-966b-98ad3be2f591",
    }),
  });

  if (!letterResponse.ok) {
    console.log(letterResponse.statusText);
    return null;
  }

  try {
    const letter = parse(LetterSchema, await letterResponse.json());
    return letter;
  } catch (error) {
    console.error(error);
    return null;
  }
}
