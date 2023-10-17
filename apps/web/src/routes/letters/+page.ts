import { array, date, object, parse, string, uuid } from "valibot";

/**
 * pub struct Letter {
    pub id: uuid::Uuid,
    pub message: String,
    pub to_user_id: uuid::Uuid,
    pub by_user_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub sending_info_id: Option<uuid::Uuid>,
}
 */

const Letter = object({
  id: string([uuid()]),
  message: string(),
  to_user_id: string([uuid()]),
  by_user_id: string([uuid()]),
  created_at: date(),
  updated_at: date(),
});

const Letters = array(Letter);

/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
  const lettersResponse = await fetch(
    `${import.meta.env.SERVER_API_URL}/me/letters`,
  );
  const letters = parse(Letters, await lettersResponse.json());

  return {
    letters,
  };
}
