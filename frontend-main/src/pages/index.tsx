import { useEffect } from "react"
import useCountriesStore from "@/stores/countriesStore";

export default function Home() {
  const $countries = useCountriesStore();

  useEffect(() => {
    $countries.populateCountries();
  }, [])

  return (
    <>
      <h2>Welcome to the home page!</h2>
      <main>
        <div className="p-5">
          <h3>Here are a list of Countries:</h3>
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