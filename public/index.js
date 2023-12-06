Vue.createApp({
data: () => ({
    greeting: "Hello Joe!",
    countries: [],
    newCountry: {
        name: "",
        continent: ""
    }
}),
computed: {
    continents() {
        return [...new Set(this.countries.map(country => country.continent))];
    }
},
mounted() {
    this.populateCountries();
},
methods: {
    populateCountries() {
        fetch("/countries")
            .then(res => res.json())
            .then(data => this.countries = data);
    },
    handleSaveCountry() {
        if (this.newCountry.name.length < 1 || this.newCountry.continent.length < 1) {
            return;
        }
            
        fetch("/countries", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                name: this.newCountry.name,
                continent: this.newCountry.continent
            })
        })
        .then(res => res.ok
            ? this.populateCountries() 
            : console.log("Error saving country")
        );
    },
    handleDelete(countryId) {
        fetch(`/countries/${countryId}`, {
            method: "DELETE"
        })
        .then(res => res.ok 
            ? this.populateCountries() 
            : console.log("Error deleting country")
        );
    },
    filteredCountries(continent) {
        return this.countries.filter(country => country.continent === continent);
    }
}
}).mount("#app")