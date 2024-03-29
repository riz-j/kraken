import { useEffect } from "react"
import useCountriesStore from "@/stores/countriesStore";
import { Link } from "react-router-dom";
import Navigation from "@/components/Navigation";

export default function Home() {
  const $countries = useCountriesStore();

  useEffect(() => {
    $countries.populateCountries();
  }, [])

  return (
    <>
      <Navigation />
      <h2><b>Welcome to the items page!</b></h2>
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