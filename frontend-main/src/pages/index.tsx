import { CountryExtendedSchema } from "@kraken_bindings/CountryExtendedSchema";
import { useEffect, useState } from "react"
import useCountriesStore from "../stores/countriesStore";

export default function Home() {
  const $countries = useCountriesStore();
  const [country, setCountry] = useState<CountryExtendedSchema | undefined>();

  const fetchCountry = async () => {
    const response = await fetch("http://localhost:2900/countries/1");
    const country = await response.json();
    setCountry(country);
  }

  useEffect(() => {
    fetchCountry();
    $countries.populateCountries();
  }, [])

  return (
    <>
      <h1>Welcome to the home page!</h1>
      <main>
        <p>The country of {country?.name} is situated in the continent of {country?.continent}.</p>
        <p>There are {country?.cities.length} cities in {country?.name}, namely {country?.cities.map((city, index) => `${index + 1}) ${city.name} `)}</p>
        <p>The arhive status of {country?.name} is {JSON.stringify(country?.is_archived)}</p>
      </main>
      <div>
        <h3>Countries:</h3>
        {$countries.countries.map(country => 
          <p>{JSON.stringify(country)}</p>  
        )}
      </div>
    </>
  )
}