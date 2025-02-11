import { auth } from "@/lib/auth";
import { defineMiddleware } from "astro:middleware";

// `context` and `next` are automatically typed
export const onRequest = defineMiddleware(async (context, next) => {
    console.log(context)
    console.log(next)
    const isAuthed = await auth.api
        .getSession({
            headers: context.request.headers,
        })
        .catch((e) => {
            console.log(e)
            return null;
        });
    if (context.url.pathname === "/dashboard" && !isAuthed) {
        return context.redirect("/");
    }
    return next();
});