const { createApp, ref } = Vue;

createApp({
	setup() {
		const email = ref("");
		const password = ref("");

		const handleLogin = () => {
			fetch("/auth/login", {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({
					email: email.value,
					password: password.value
				})
			}).then(res => {
				if (res.ok) {
					window.location.href = "/api/countries";
				}
			});
		};

		return {
			email,
			password,
			handleLogin
		}
	}
}).mount("#app")