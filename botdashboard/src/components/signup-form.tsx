import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import React from "react";
import {authClient} from "@/lib/auth-client.ts";

export function SignupForm({
  className,
  ...props
}: React.ComponentPropsWithoutRef<"div">) {

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const data = new FormData(form);
    authClient.signUp.email(
        {
          email: data.get("email") as string,
          name: data.get("name") as string,
          password: data.get("password") as string,
        },
        {
          onError: (error) => {
            console.warn(error);
            // toast.error(error.error.message);
          },
          onSuccess: () => {
            console.log("Sign up successfully");
            // toast.success("You have been logged in!");
          },
        },
    );
  }

  return (
    <div className={cn("flex flex-col gap-6", className)} {...props}>
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">Sign up</CardTitle>
          <CardDescription>
            Enter your email below to sign up for an account
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit}>
            <div className="flex flex-col gap-6">
              <div className="grid gap-2">
                <Label htmlFor="email">Email</Label>
                <Input
                    id="email"
                    type="email"
                    placeholder="you@example.com"
                    required
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="text">Username</Label>
                <Input
                    id="name"
                    type="text"
                    placeholder="you"
                    required
                />
              </div>
              <div className="grid gap-2">
                <div className="flex items-center">
                  <Label htmlFor="password">Password</Label>
                </div>
                <Input id="password" type="password" required/>
              </div>
              <Button type="submit" className="w-full bg-neutral-950 dark:bg-neutral-100">
                Sign up
              </Button>
            </div>
            <div className="mt-4 text-center text-sm">
              Already have an account?{" "}
              <a href="/login" className="underline underline-offset-4">
                Login
              </a>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}
