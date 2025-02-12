import { auth } from "@/lib/auth";
// @ts-ignore
import { defineMiddleware } from "astro:middleware";

// @ts-ignore
export const onRequest = defineMiddleware(async (context, next) => {
    const isAuthed = await auth.api
        .getSession({
            headers: context.request.headers,
        })
        .catch(() => {
            return null;
        });
    if (context.url.pathname === "/dashboard" && !isAuthed) {
        return context.redirect("/");
    }
    return next();
});
