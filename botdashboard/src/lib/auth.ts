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
                try {
                   const url = `http://api:8080/mail`
                    const data = `${email} ${otp} ${type}`
                    await fetch(url, {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'text/plain'
                        },
                        body: data
                    })
                } catch (error) {
                    console.error('Error sending email verification OTP: ', error);
                }
            },
            otpLength: 6,
            expiresIn: 600,
        })
    ]
})
