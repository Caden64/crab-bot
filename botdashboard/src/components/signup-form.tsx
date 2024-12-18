import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "./ui/button";
import { createSignal } from "solid-js";
import { TextFieldRoot, TextField, TextFieldLabel} from "@/components/ui/textfield.tsx";
import {Checkbox, CheckboxControl, CheckboxLabel} from "@/components/ui/checkbox";
import {authClient} from "@/lib/auth-client.ts";

export function SignupForm() {
  const [email, setEmail] = createSignal("");
  const [name, setName] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [rememberMe, setRememberMe] = createSignal(false);
  return (
      <Card class="max-w-max">
        <CardHeader>
          <CardTitle class="text-lg md:text-xl">Sign In</CardTitle>
          <CardDescription class="text-xs md:text-sm">
            Enter your email below to sign up for an account
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div class="grid gap-4">
            <div class="grid gap-2">
              <TextFieldRoot class="w-full">
                <TextFieldLabel for="email">Email</TextFieldLabel>
                <TextField
                    type="email"
                    placeholder="Email"
                    value={email()}
                    onInput={(e) => {
                      if ("value" in e.target) setEmail(e.target.value as string);
                    }}
                />
              </TextFieldRoot>
              <TextFieldRoot class="w-full">
                <TextFieldLabel for="name">Username</TextFieldLabel>
                <TextField
                    type="text"
                    placeholder="Username"
                    value={name()}
                    onInput={(e) => {
                      if ("value" in e.target) setName(e.target.value as string);
                    }}
                />
              </TextFieldRoot>
              <TextFieldRoot class="w-full">
                <div class="flex items-center justify-between">
                  <TextFieldLabel for="password">Password</TextFieldLabel>
                </div>
                <TextField
                    type="password"
                    placeholder="Password"
                    value={password()}
                    onInput={(e) => {
                      if ("value" in e.target)
                        setPassword(e.target.value as string);
                    }}
                />
              </TextFieldRoot>
              <Button
                  onclick={() => {
                    authClient.signUp.email({
                      email: email(),
                      name: name(),
                      password: password(),
                      fetchOptions: {
                        onError(context) {
                          alert(context.error.message);
                        },
                        onSuccess(context ) {
                          alert("Sign up successfully!");
                        }
                      },
                      callbackURL: "/signin",
                    });
                  }}
              >
                Sign Up
              </Button>
            </div>
            <p class="text-sm text-center">
              Already have an account?{" "}
              <a
                  href="/signin"
                  class="text-blue-900 dark:text-orange-200 underline"
              >
                Sign In
              </a>
            </p>
          </div>
        </CardContent>
      </Card>
  );
}
