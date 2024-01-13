import { useEffect, useState } from "react"
import useCountriesStore from "@/stores/countriesStore";
import { Link } from "react-router-dom";
import { CountryInsert } from "@kraken_bindings/CountryInsert";

export default function Home() {
  const $countries = useCountriesStore();
  const [countryInsert, setCountryInsert] = useState<CountryInsert>({
    name: "",
    continent: ""
  });

  useEffect(() => {
    $countries.populateCountries();
  }, [])

  const handleCountryInsert = (e: React.FormEvent) => {
    e.preventDefault();

    fetch("/api/countries", {
      method: "POST",
      body: JSON.stringify(countryInsert)
    }).then(() => {
      console.log("SUCCESS")
      $countries.populateCountries();
    }).catch(error => {
      console.log(error);
    });
  }

  return (
    <>
      <form onSubmit={handleCountryInsert}>
        <input placeholder="name" onChange={(e) => setCountryInsert(state => ({...state, name: e.target.value }))}/>
        <input placeholder="continent" onChange={(e) => setCountryInsert(state => ({...state, continent: e.target.value }))}/>
      </form>
      <h2>Welcome to the home page!</h2>
      <main>
        <div className="p-5">
          <h3>Here are a list of Countries:</h3>
          <Link to={"/office"}>Office Page</Link>
          {$countries.countries.map(country => 
            <div className="flex">
              <p><b>{country.name}</b>|</p>
              <p>{country.continent}</p>
            </div>
          )}
        </div>
      </main>
    </>
  )
}