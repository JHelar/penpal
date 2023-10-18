import { SvelteKitAuth } from "@auth/sveltekit";
import GoogleProvider from "@auth/core/providers/google";
import {
  VITE_AUTH_SECRET,
  VITE_GOOGLE_CLIENT_ID,
  VITE_GOOGLE_CLIENT_SECRET,
} from "$env/static/private";

export const handle = SvelteKitAuth({
  providers: [GoogleProvider({
    clientId: VITE_GOOGLE_CLIENT_ID,
    clientSecret: VITE_GOOGLE_CLIENT_SECRET,
    authorization: {
      params: {
        prompt: "consent",
        access_type: "offline",
        response_type: "code",
      },
    },
  })],
  secret: VITE_AUTH_SECRET,
});
