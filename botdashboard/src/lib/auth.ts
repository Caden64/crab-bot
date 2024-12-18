import { betterAuth } from "better-auth";
import pkg from 'pg';
const { Pool } = pkg;
export const auth = betterAuth({
    database: new Pool({
        connectionString: "postgres://:user:password@postgres:5432/better_auth"
    }),
    emailAndPassword: {
        enabled: true
    },
})