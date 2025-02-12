import { betterAuth } from "better-auth";
import { emailOTP, twoFactor, username } from "better-auth/plugins";
import { passkey } from "better-auth/plugins/passkey"

import pkg from 'pg';
const { Pool } = pkg;
export const auth = betterAuth({
    appName: "PPSC Cyber Club",
    database: new Pool({
        connectionString: "postgres://user:password@postgres:5432/better_auth"
    }),
    emailAndPassword: {
        enabled: true
    },
    plugins: [
        twoFactor(), username(), passkey(), emailOTP({
            async sendVerificationOTP({email, otp, type}) {
                console.log(email, otp, type)
            },
            otpLength: 6,
            expiresIn: 600,
        })
    ]
})
