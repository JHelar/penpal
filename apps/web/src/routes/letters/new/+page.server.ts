import { createLetter } from "$lib/server/letter";
import type { Actions } from "./$types";

export const actions = {
    default: async ({ request }) => {
        const data = await request.formData()
        const message = data.get('message');

        if(!message || typeof message !== 'string') {
            return {
                success: false
            }
        }

        const letter = await createLetter({ message, request })
        
        if(!letter) {
            return {
                success: false
            }
        }

        console.log(letter)

        return {
            success: true
        }
    }
} satisfies Actions