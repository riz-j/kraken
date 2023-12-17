import { useEffect, useState } from 'react'
import './App.css'
import { CountryExtendedSchema } from '@kraken_bindings/CountryExtendedSchema'

function App() {
  const [count, setCount] = useState(0)
  const [country, setCountry] = useState<CountryExtendedSchema | undefined>()

  const fetchCountry = async () => {
    const response = await fetch("http://localhost:2900/countries/1");
    const result = await response.json() as CountryExtendedSchema;
    setCountry(result);
  }

  useEffect(() => {
    fetchCountry()
  }, [])

  return (
    <>
      <h3>The country {country?.name} has {country?.cities.length} cities, namely {country?.cities.map((city, index) => `${index + 1}) ${city.name} `)}</h3>
      <h3>The coutnry of {country?.name}'s archive status is: {JSON.stringify(country?.is_archived)}</h3>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
      </div>
    </>
  )
}

export default App
