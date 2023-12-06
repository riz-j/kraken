const { createApp, ref } = Vue;

createApp({
    setup() {
        const email = ref("");
        const password = ref("");
        const confirm_password = ref("");
        const first_name = ref("");
        const last_name = ref("");

        const handleSignUp = () => {
            if (password.value !== confirm_password.value) {
                console.log("Password does not match!");
                return;
            }
            fetch("/auth/signup", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    email: email.value,
                    password: password.value,
                    first_name: first_name.value,
                    last_name: last_name.value
                })
            }).then(res => {
                if (res.ok) { window.location.href = "/countries" }
            });
        }

        return {
            email,
            password,
            confirm_password,
            first_name,
            last_name,
            handleSignUp
        }
    }
}).mount("#app")