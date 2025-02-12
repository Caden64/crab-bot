import {authClient} from "@/lib/auth-client.ts";
import {Button} from "@/components/ui/button.tsx";

export function OTP() {
    return (
        <Button onclick={async () => {
            await authClient.emailOtp.sendVerificationOtp({
                email: "caden@caden64.com",
                type: "email-verification"
            })
        }}>
            OTP
        </Button>
    )
}
