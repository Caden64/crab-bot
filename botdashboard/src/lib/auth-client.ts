import { createAuthClient } from "better-auth/solid"
import {emailOTPClient, twoFactorClient, usernameClient, passkeyClient} from "better-auth/client/plugins";
export const authClient =  createAuthClient({
    plugins: [
        emailOTPClient(), twoFactorClient(), usernameClient(), passkeyClient()
    ]
})