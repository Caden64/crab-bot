import { betterAuth } from "better-auth";
// @ts-ignore
import Database from "better-sqlite3";

export const auth = betterAuth({
    database: new Database("./sqlite.db"),
    emailAndPassword: {
        enabled: true
    },
})