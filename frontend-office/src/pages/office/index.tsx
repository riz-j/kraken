import { useEffect, useState } from "react"
import useCountriesStore from "@/stores/countriesStore";
import { Link } from "react-router-dom";
import Navigation from "@/components/Navigation";
import { CountryUpdate } from "@kraken_bindings/CountryUpdate";
import { CountryInsert } from "@kraken_bindings/CountryInsert";

export default function Home() {
  const $countries = useCountriesStore();
  const [countryUpdate, setCountryUpdate] = useState<Partial<CountryUpdate>>({});
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
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(countryInsert)
    }).then(() => {
      console.log("SUCCESS")
      $countries.populateCountries();
    }).catch(error => {
      console.log(error);
    });
  }

  const handleCountryUpdate = (e: React.FormEvent) => {
    e.preventDefault();

    fetch("/api/countries/1", {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(countryUpdate)
    }).then(() => {
      console.log("SUCCESS")
      $countries.populateCountries();
    }).catch(error => {
      console.log(error);
    });
  }

  return (
    <>
      <Navigation />
      <form onSubmit={handleCountryInsert}>
        <input style={{color: "black"}} type="text" placeholder="name" onChange={(e) => setCountryInsert(state => ({...state, name: e.target.value }))}/>
        <input style={{color: "black"}} type="text" placeholder="continent" onChange={(e) => setCountryInsert(state => ({...state, continent: e.target.value }))}/>
        <input style={{color: "white"}} type="submit" />
      </form>

      <form onSubmit={handleCountryUpdate}>
        <input style={{color: "black"}} type="text" placeholder="name" onChange={(e) => setCountryUpdate(state => ({...state, name: e.target.value }))}/>
        <input style={{color: "black"}} type="text" placeholder="continent" onChange={(e) => setCountryUpdate(state => ({...state, continent: e.target.value }))}/>
        <input style={{color: "white"}} type="submit" />
      </form>
      <h2>Welcome to the office page!</h2>
      <main>
        <div className="p-5">
          <h1>THE BASE URL IS: {JSON.stringify(import.meta.env)}</h1>
          <h3>Here are a list of Countries:</h3>
          <Link to={"/"}>Home Page</Link>
          {$countries.countries.map(country => 
            <div className="flex">
              <p><b>{country.id.toString()}|</b></p>
              <p><b>{country.name}</b>|</p>
              <p>{country.continent}</p>
            </div>
          )}
        </div>
      </main>
    </>
  )
}