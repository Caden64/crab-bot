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

export function LoginForm() {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [rememberMe, setRememberMe] = createSignal(false);
  return (
      <Card class="max-w-max">
        <CardHeader>
          <CardTitle class="text-lg md:text-xl">Sign In</CardTitle>
          <CardDescription class="text-xs md:text-sm">
            Enter your email below to login to your account
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
                <div class="flex items-center justify-between">
                  <TextFieldLabel for="password">Password</TextFieldLabel>
                  <a
                      href="/forget-password"
                      class="ml-auto inline-block text-sm underline"
                  >
                    Forgot your password?
                  </a>
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
              <Checkbox
                  class="flex items-center gap-2 z-50"
                  onChange={(e) => {
                    setRememberMe(e);
                  }}
                  checked={rememberMe()}
              >
                <CheckboxControl />
                <CheckboxLabel class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                  Remember Me
                </CheckboxLabel>
              </Checkbox>
              <Button
                  onclick={() => {
                    authClient.signIn.email({
                      email: email(),
                      password: password(),
                      rememberMe: rememberMe(),
                      fetchOptions: {
                        onError(context) {
                          alert(context.error.message);
                        },
                      },
                      callbackURL: "/",
                    });
                  }}
              >
                Sign In
              </Button>
            </div>
            <p class="text-sm text-center">
              Don't have an account yet?{" "}
              <a
                  href="/signup"
                  class="text-blue-900 dark:text-orange-200 underline"
              >
                Sign Up
              </a>
            </p>
          </div>
        </CardContent>
      </Card>
  );
}
